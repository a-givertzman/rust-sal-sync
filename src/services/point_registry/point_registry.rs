use crate::{collections::FxHashMap, services::entity::{PointConf, PointType}, sync::Mutex, thread_pool::Scheduler};
use std::{collections::HashMap, env, ffi::OsStr, fmt::{Debug, Display}, fs, hash::BuildHasherDefault, path::{Path, PathBuf}, sync::Arc, time::Instant};
use api_tools::{api::reply::api_reply::ApiReply, client::{api_query::{ApiQuery, ApiQueryKind, ApiQuerySql}, api_request::ApiRequest}};
use dashmap::DashMap;
use hashers::fx_hash::FxHasher;
use concat_string::concat_string;
use indexmap::IndexMap;
use sal_core::{dbg::Dbg, error::Error};
use serde::{Deserialize, Serialize};
use super::registry_conf::RegistryConf;

#[deprecated(note = "Use PointRegistry instead")]
pub type RetainPointId = PointRegistry;
///
/// Local registry alias
type RegistryCahe = FxHashMap<String, FxHashMap<String, RetainedPointConf>>;
///
/// Provides access, stores and syncing for the unique Point ID
/// - In the local registry store (json file)
/// - In the database registry table, if `api` is specified
pub struct PointRegistry {
    cache: Arc<DashMap<String, Vec<PointConf>>>,
    path: PathBuf,
    conf: RegistryConf,
    /// Points pending for insertion
    pending: Arc<Mutex<(bool, Vec<InsertTask>)>>, 
    scheduler: Option<Scheduler>,
    dbg: Dbg,
}
//
//
impl PointRegistry {
    ///
    /// Creates `PointRegistry` new instance
    ///  - `parent` - the name of the parent object
    ///  - `services` - Services thread safe mutable reference
    ///  - `conf` - path to the file, where point id's will be stored
    ///  - `conf.api` - API parameters to send Point's to the database 
    pub fn new(parent: impl Into<String>, conf: RegistryConf, scheduler: Option<Scheduler>) -> Self {
        let dbg = Dbg::new(parent, "PointRegistry");
        let path = match conf.point_path() {
            Ok(path) => path,
            Err(err) => panic!("{}.new | Error: {:#?}", dbg, err),
        };
        Self {
            cache: Arc::new(DashMap::new()),
            path,
            conf,
            pending: Arc::new(Mutex::new((false, vec![]))),
            scheduler,
            dbg,
        }
    }
    ///
    /// Returns true if already cached
    pub fn is_cached(&self) -> bool {
        !self.cache.is_empty()
    }
    ///
    /// Inserts collection of `points` owned by `owner`
    pub fn insert(&self, owner: &str, points: Vec<PointConf>) {
        let dbg = self.dbg.clone();
        log::debug!("{dbg}.insert | Equeuing {} Point's from '{}' for caching", points.len(), owner);
        let mut pending_guard = self.pending.lock();
        // Добавляем в буфер
        pending_guard.1.push(InsertTask {
            owner: owner.to_owned(),
            points: points,
        });
        // Если воркер уже работает, просто выходим (он заберет нашу задачу на следующем цикле)
        if pending_guard.0 {
            return;
        }
        // Если воркера нет, помечаем, что он запущен
        pending_guard.0 = true;
        drop(pending_guard);
        // Клонируем Arc-и для передачи в фоновую задачу
        let conf = self.conf.clone();
        let cache = self.cache.clone();
        let pending = self.pending.clone();
        let path = self.path.clone();
        match &self.scheduler {
            Some(scheduler) => _ = scheduler.spawn(|| {
                Self::insert_task(dbg, conf, path, pending, cache);
                Ok(())
            }),
            None => _ = std::thread::spawn(|| {
                Self::insert_task(dbg, conf, path, pending, cache);
            }),
        }
    }
    ///
    /// Worker inserts points into the cache, storing it to the file and DB
    fn insert_task(dbg: Dbg, conf: RegistryConf, path: PathBuf, state_arc: Arc<Mutex<(bool, Vec<InsertTask>)>>, cache: Arc<DashMap<String, Vec<PointConf>>>) {
        loop {
            let t = Instant::now();
            // Атомарно забираем весь буфер, оставляя пустой
             let tasks = {
                let mut lock = state_arc.lock();
                if lock.1.is_empty() {
                    // Если задач больше нет, атомарно снимаем флаг и выходим из потока!
                    // Так как мы под Mutex, никто не сможет "протиснуться" между проверкой и выходом.
                    lock.0 = false;
                    log::debug!("{dbg}.insert | Worker exit");
                    return; 
                }
                // Забираем данные, на их место ставим пустой вектор
                let mut state = vec![];
                std::mem::swap(&mut lock.1, &mut state);
                state
            };
            log::debug!("{dbg}.insert | Processing {} new insertion requests", tasks.len());
            let mut update_retained = false;
            let mut retained: RegistryCahe = Self::read(&dbg, path.clone());
            // log::trace!("{dbg}.insert | retained: {:#?}", retained);
            let mut next_id = retained.values()
                .flat_map(|v| v.values())
                .map(|conf| conf.id)
                .max()
                .map_or(0, |id| id + 1);
            log::debug!("{dbg}.insert | next id: {next_id}");
            for task in tasks {
                let task_points_len = task.points.len();
                log::debug!("{dbg}.insert | Caching {task_points_len} Point's from '{}'...", task.owner);
                for mut point in task.points {
                    log::trace!("{dbg}.insert | point: {}...", point.name);
                    point.id = retained
                        .entry(task.owner.clone())
                        .or_insert(FxHashMap::with_hasher(BuildHasherDefault::<FxHasher>::default()))
                        .entry(point.name.clone())
                        .or_insert_with(|| {
                            let id = next_id;
                            next_id += 1;
                            update_retained = true;
                            RetainedPointConf { id, name: point.name.clone(), typ: point.type_.clone() }
                        }).id;
                    cache
                        .entry(task.owner.to_owned())
                        .or_insert(vec![])
                        .push(point);
                }
                log::debug!("{dbg}.insert | Caching {task_points_len} Point's from '{}' - Ok", task.owner);
            }
            if update_retained {
                if let Err(err) = Self::write(&dbg, &path, &retained) {
                    log::warn!("{dbg}.insert | Can't store Point's \n\terror: {:?}", err);
                }
                Self::sql_write(&dbg, &conf, &retained)
            }
            log::debug!("{dbg}.insert | Elapsed {:?}", t.elapsed());
        }
    }
    ///
    /// Returns configuration of the Point's
    pub fn points(&self) -> IndexMap<String, Vec<PointConf>> {
        let points = self.cache
            .iter()
            .map(|r| (r.key().clone(), r.value().clone()));
        IndexMap::from_iter(points)
    }
    ///
    /// Creates directiry (all necessary folders in the 'path' if not exists)
    ///  - path is relative, will be joined with current working dir
    fn create_dir(dbg: &Dbg, path: &Path) -> Result<PathBuf, Error> {
        let current_dir = env::current_dir().map_err(|err| Error::new(dbg, "create_dir").pass(err.to_string()))?;
        let path = current_dir.join(path);
        match path.exists() {
            true => Ok(path),
            false => {
                match fs::create_dir_all(&path) {
                    Ok(_) => Ok(path),
                    Err(err) => {
                        let err = Error::new(dbg, "create_dir").pass_with(format!("Can't create dir: '{:?}'", path), err.to_string());
                        log::error!("{}", err);
                        Err(err)
                    }
                }
            }
        }
    }
    ///
    /// Reads file contains json map:
    /// ```json
    /// {
    ///     "/path/Point.name1": 0,
    ///     "/path/Point.name2": 1,
    ///     ...
    /// }
    /// ```
    fn read<P: AsRef<Path> + AsRef<OsStr> + std::fmt::Debug>(dbg: impl Display, path: P) -> FxHashMap<String, FxHashMap<String, RetainedPointConf>> {
        match fs::read_to_string(&path) {
            Ok(json_string) => {
                match serde_json::from_str(&json_string) {
                    Ok(config) => {
                        return config
                    }
                    Err(err) => {
                        log::warn!("{dbg}.read | Error in config: {:?}\n\terror: {:?}", json_string, err);
                    }
                }
            }
            Err(err) => {
                log::debug!("{dbg}.read | File '{:?}' reading error: {:?}", path, err);
            }
        };
        HashMap::with_hasher(BuildHasherDefault::<FxHasher>::default())
    }
    ///
    /// Writes file json map to the file:
    /// ```json
    /// {
    ///     "/path/Point.name1": 0,
    ///     "/path/Point.name2": 1,
    ///     ...
    /// }
    /// ```
    fn write<P: AsRef<Path>, S: Serialize>(dbg: &Dbg, path: P, points: S) -> Result<(), Error> {
        let error = Error::new(dbg, "write");
        let path = path.as_ref();
        let dir = path.parent()
            .ok_or_else(|| error.err(format!("Can't get parent: '{:?}'", path)))?;
        Self::create_dir(dbg, dir)
            .map_err(|err| error.pass(err))?;
        // 1. Формируем имя временного файла (например: config.json.tmp)
        let tmp_path = path.with_extension("tmp");
        let f = fs::OpenOptions::new()
            .truncate(true)
            .create(true)
            .write(true)
            .open(&tmp_path)
            .map_err(|err| error.pass_with(format!("Can't open tmp file {}", tmp_path.display()), err.to_string()))?;
        serde_json::to_writer_pretty(&f, &points)
            .map_err(|err| error.pass_with("JSON serialization failed", err.to_string()))?;
        f.sync_all()
            .map_err(|err| error.pass_with("Disk sync failed", err.to_string()))?;
        fs::rename(&tmp_path, path)
            .map_err(|err| error.pass_with(format!("File rename failed {} -> {}", tmp_path.display(), path.display()), err.to_string()))
    }
    ///
    /// Stores points into the database
    fn sql_write(dbg: &Dbg, conf: &RegistryConf, retained: &RegistryCahe) {
        match conf.point_api() {
            Ok(api) => {
                let api_keep_alive = true;
                let sql_keep_alive = true;
                let mut request = ApiRequest::new(
                    dbg,
                    &api.address,
                    &api.auth_token,
                    ApiQuery::new(
                        ApiQueryKind::Sql(ApiQuerySql::new(&api.database, "select 1;")),
                        sql_keep_alive,
                    ),
                    api_keep_alive,
                    false,
                );
                _ = Self::sql_request(dbg, conf, &mut request, "truncate public.tags;", api_keep_alive);
                for (_owner, points) in retained {
                    for point in points.values() {
                        let point_name = point.name.replace("'", "''"); // Заменяем одну кавычку на две
                        let sql = format!("insert into public.tags (id, type, name) values ({},'{:?}','{}');", point.id, point.typ, point_name);
                        _ = Self::sql_request(dbg, conf, &mut request, &sql, api_keep_alive);
                    }
                }
            }
            Err(err) => log::warn!("{dbg}.sql_write | Database cant be updates, api is not specified, \n\t error: {:#?}", err),
        }
    }
    ///
    /// Make the sql request to store ponts to the database
    fn sql_request(dbg: &Dbg, conf: &RegistryConf, request: &mut ApiRequest, sql: &str, keep_alive: bool) -> Result<ApiReply, Error> {
        let error = Error::new(dbg, "sql_request");
        match conf.point_api() {
            Ok(api) => {
                let query = ApiQuery::new(
                    ApiQueryKind::Sql(ApiQuerySql::new(&api.database, sql)),
                    true,
                );
                match request.fetch_with(&query, keep_alive) {
                    Ok(reply) => {
                        if log::max_level() > log::LevelFilter::Debug {
                            let reply_str = std::str::from_utf8(&reply).map_err(|err| error.pass(err.to_string()))?;
                            log::debug!("{dbg}.send | reply str: {:?}", reply_str);
                        }
                        match serde_json::from_slice(&reply) {
                            Ok(reply) => Ok(reply),
                            Err(err) => {
                                let reply = match std::str::from_utf8(&reply) {
                                    Ok(reply) => reply.to_string(),
                                    Err(err) => concat_string!(dbg, ".send | Error parsing reply to utf8 string: ", err.to_string()),
                                };
                                let err = error.pass_with(format!("Can't parsing API reply: \n\t{:?}", reply), err.to_string());
                                log::warn!("{}", err);
                                Err(err)
                            }
                        }
                    }
                    Err(err) => {
                        let err = error.pass_with(format!("Can't send API request"), err.to_string());
                        log::warn!("{}", err);
                        Err(err)
                    }
                }
            }
            Err(err) => {
                let err = error.pass_with(format!("API is not specified"), err.to_string());
                log::warn!("{}", err);
                Err(err)
            }
        }
    }
}
//
impl Debug for PointRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PointRegistry")
            .field("id", &self.dbg)
            .field("cache", &self.cache)
            .field("path", &self.path)
            .field("conf", &self.conf)
            // .field("state", &self.state)
            // .field("scheduler", &self.scheduler)
            .finish()
    }
}
///
/// Private wrapper for Point to be stored
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct RetainedPointConf {
    pub id: usize,
    pub name: String,
    #[serde(rename = "type")]
    #[serde(alias = "type", alias = "Type")]
    pub typ: PointType,
}
// Private entity for enqueue insertion
struct InsertTask {
    owner: String,
    points: Vec<PointConf>,
}
///
/// Базовое тестирование
#[cfg(test)]
mod tests {
    use debugging::session::debug_session::{DebugSession, LogLevel};
    use crate::services::point_registry::PointRegistryConf;
    use super::*;
    use std::{fs, thread, time::Duration};
    ///
    /// Вспомогательная функция для создания точки (подставь свои реальные поля, если нужно)
    fn make_point(name: &str) -> PointConf {
        PointConf {
            id: 0, // Изначально всегда 0
            name: name.to_string(),
            type_: PointType::Int,
            history: Default::default(),
            alarm: Default::default(),
            address: Default::default(),
            filters: Default::default(),
            comment: Default::default(),
        }
    }
    /// 
    /// Тест проверки точности.
    /// - Имитирует спам метриками от разных устройств,
    /// - Ждет завершения фонового воркера и жестко проверяет консистентность IDs в горячем кэше и на диске.
    #[test]
    fn test_retain_point_id_accuracy() {
        DebugSession::new().filter(LogLevel::Debug).init();
        // 1. Подготовка чистого окружения
        let retain_dir = "src/tests/unit/services/retain/retain_point_id";
        let file_name = "id1.json";
        // Предполагаем, что можно создать RetainConf программно (подставь нужный способ)
        let conf = RegistryConf::new(Some(retain_dir), Some(PointRegistryConf::new(file_name, None))); 
        let dbg = Dbg::own("Test");
        let retainer = PointRegistry::new(&dbg, conf, None);
        // 2. Имитация нагрузки. 
        // Важно: мы шлем данные вперемешку, чтобы проверить правильность сквозной нумерации
        retainer.insert("Device_A", vec![make_point("Temp"), make_point("Press")]); // Ожидаем ID: 0, 1
        retainer.insert("Device_B", vec![make_point("Speed")]); // Ожидаем ID: 2
        // Повторная вставка существующих + добавление новых
        // Temp уже имеет ID 0, он должен сохраниться. Voltage должен получить ID 3.
        retainer.insert("Device_A", vec![make_point("Temp"), make_point("Voltage")]); 
        // 3. Ожидание обработки очереди
        // Так как воркер работает в фоне, мы должны дождаться опустошения очереди и снятия флага.
        let mut attempts = 0;
        loop {
            {
                let lock = retainer.pending.lock();
                if !lock.0 && lock.1.is_empty() {
                    break;
                }
            }
            attempts += 1;
            if attempts > 50 {
                panic!("Worker timeout! Очередь не очистилась.");
            }
            thread::sleep(Duration::from_millis(50));
        }
        // 4. Валидация горячего кэша (DashMap)
        let dev_a = retainer.cache.get("Device_A").expect("Device_A not in cache");
        assert_eq!(dev_a.len(), 4, "Device_A should have 4 points in cache (Temp, Press, Temp, Voltage)");
        // Проверяем точные ID для Device_A
        assert_eq!(dev_a[0].name, "Temp");
        assert_eq!(dev_a[0].id, 0);
        assert_eq!(dev_a[1].name, "Press");
        assert_eq!(dev_a[1].id, 1);
        assert_eq!(dev_a[2].name, "Temp");
        assert_eq!(dev_a[2].id, 0, "Duplicate point should retain the same ID");
        assert_eq!(dev_a[3].name, "Voltage");
        assert_eq!(dev_a[3].id, 3, "New point should get sequential ID");
        let dev_b = retainer.cache.get("Device_B").expect("Device_B not in cache");
        assert_eq!(dev_b[0].name, "Speed");
        assert_eq!(dev_b[0].id, 2);
        // 5. Валидация файла (Диск)
        let file_content = fs::read_to_string(&Path::new(retain_dir).join(file_name)).expect("Can't read result file");
        let retained: RegistryCahe = serde_json::from_str(&file_content).expect("Invalid JSON");
        assert_eq!(retained["Device_A"]["Temp"].id, 0);
        assert_eq!(retained["Device_A"]["Press"].id, 1);
        assert_eq!(retained["Device_B"]["Speed"].id, 2);
        assert_eq!(retained["Device_A"]["Voltage"].id, 3);
        println!("{dbg} | ✅ Accuracy test passed! Logic is rock solid.");
    }
    ///
    /// Стресс-тест на состояние гонки и консистентность ID.
    /// - 50 потоков одновременно пытаются вставить по 100 точек.
    /// - Имена точек специально частично пересекаются, чтобы спровоцировать конфликты.
    /// - Проверяет, что итоговый словарь ID не имеет пропусков и дубликатов.
    #[test]
    fn test_retain_point_id_stress() {
        // Желательно поставить Info или Warn, чтобы логи не замедляли тест
        DebugSession::new().filter(LogLevel::Info).init(); 
        let retain_dir = "src/tests/unit/services/retain/retain_point_id";
        let file_name = "id0_stress.json";
        let file_path = Path::new(retain_dir).join(file_name);
        // Очищаем от предыдущих запусков
        let conf = RegistryConf::new(Some(retain_dir), Some(PointRegistryConf::new(file_name, None))); 
        let dbg = Dbg::own("StressTest");
        // Оборачиваем в Arc для шаринга между потоками
        let retainer = Arc::new(PointRegistry::new(&dbg, conf, None));
        let num_threads = 50;
        let points_per_thread = 100;
        let mut handles = vec![];
        // 1. Атака из множества потоков
        for t_idx in 0..num_threads {
            let retainer_clone = retainer.clone();
            handles.push(thread::spawn(move || {
                // Симулируем 5 разных девайсов-владельцев
                let owner = format!("Device_{}", t_idx % 5); 
                let mut points = vec![];
                for p_idx in 0..points_per_thread {
                    // Создаем перекрывающиеся имена (один девайс пришлет Point_x_y несколько раз)
                    let point_name = format!("Point_{}_{}", t_idx % 10, p_idx); 
                    points.push(make_point(&point_name));
                }
                retainer_clone.insert(&owner, points);
            }));
        }
        // Ждем завершения всех генерирующих потоков
        for handle in handles {
            handle.join().unwrap();
        }
        // 2. Ждем, пока воркер переварит всю очередь
        let mut attempts = 0;
        loop {
            {
                let lock = retainer.pending.lock();
                if !lock.0 && lock.1.is_empty() {
                    break;
                }
            }
            attempts += 1;
            if attempts > 200 { // Ждем до 10 секунд
                panic!("Worker timeout! Воркер не справился со стресс-тестом.");
            }
            thread::sleep(Duration::from_millis(50));
        }
        // 3. Жесткая валидация результатов на диске
        let file_content = fs::read_to_string(&file_path).expect("Can't read result file");
        let retained: RegistryCahe = serde_json::from_str(&file_content).expect("Invalid JSON");
        let mut all_ids = vec![];
        // Собираем все присвоенные ID со всех девайсов
        for (_, points) in retained {
            for (_, point_conf) in points {
                all_ids.push(point_conf.id);
            }
        }
        // Сортируем ID по возрастанию
        all_ids.sort_unstable();
        let unique_points_count = all_ids.len();
        assert!(unique_points_count > 0, "Файл пуст, данные не сохранились!");
        // 4. Проверка №1: Нет ли дубликатов?
        let mut deduplicated_ids = all_ids.clone();
        deduplicated_ids.dedup(); // Удаляет дубликаты
        assert_eq!(
            all_ids.len(), 
            deduplicated_ids.len(), 
            "КРИТИЧЕСКАЯ ОШИБКА: Найдены дублирующиеся ID!"
        );
        // 5. Проверка №2: Нет ли пропусков (дырок) в нумерации?
        // Если ID раздавались корректно, они должны идти строго: 0, 1, 2, 3...
        for (index, id) in all_ids.into_iter().enumerate() {
            assert_eq!(
                index, 
                id, 
                "Рассинхрон нумерации! Ожидался ID {}, но встречен {}", index, id
            );
        }
        println!("{} | ✅ Stress test passed! Успешно обработано {} уникальных тегов.", dbg, unique_points_count);
    }
}
