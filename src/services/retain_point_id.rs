use crate::{collections::map::HashMapFxHasher, services::entity::point::{point_config::PointConfig, point_config_type::PointConfigType}};
use std::{collections::HashMap, env, ffi::OsStr, fs, hash::BuildHasherDefault, path::{Path, PathBuf}};
use api_tools::{api::reply::api_reply::ApiReply, client::{api_query::{ApiQuery, ApiQueryKind, ApiQuerySql}, api_request::ApiRequest}};
use hashers::fx_hash::FxHasher;
use concat_string::concat_string;
use indexmap::IndexMap;
use log::{debug, error, info, trace};
use serde::{Deserialize, Serialize};
type RetainedCahe = HashMapFxHasher<String, HashMapFxHasher<String, RetainedPointConfig>>;
///
/// Stores unique Point ID in the json file
/// - Additionaly copy all Points into the database, if `api` is specified
#[derive(Debug)]
pub struct RetainPointId {
    id: String,
    path: String,
    cache: IndexMap<String, Vec<PointConfig>>,
    api: Option<RetainPointApi>,
}
//
//
impl RetainPointId {
    ///
    /// Creates new instance of the RetainPointId
    ///  - `parent` - the name of the parent object
    ///  - `services` - Services thread safe mutable reference
    ///  - `path` - path to the file, where point id's will be stored
    ///  - `api` - API parameters to send Point's to the database 
    pub fn new(parent: &str, path: &str, api: Option<RetainPointApi>) -> Self {
        Self {
            id: format!("{}/RetainPointId", parent),
            path: path.to_owned(),
            cache: IndexMap::new(),
            api,
        }
    }
    ///
    /// Returns true if already cached
    pub fn is_cached(&self) -> bool {
        !self.cache.is_empty()
    }
    ///
    /// Inserts collection of [points] owned by [owner]
    pub fn insert(&mut self, owner: &str, points: Vec<PointConfig>) {
        info!("{}.points | Caching Point's from '{}'...", self.id, owner);
        let mut update_retained = false;
        let mut retained: RetainedCahe = self.read(self.path.clone());
        trace!("{}.points | retained: {:#?}", self.id, retained);
        for mut point in points {
            trace!("{}.points | point: {}...", self.id, point.name);
            let retained_clone = retained.clone();
            point.id = retained
                .entry(owner.to_owned())
                .or_insert(HashMapFxHasher::with_hasher(BuildHasherDefault::<FxHasher>::default()))
                .entry(point.name.clone())
                .or_insert_with(|| {
                    let id = retained_clone.values().map(|v| {
                        v.values()
                        .map(|conf| conf.id)
                        .max().unwrap_or(0)
                    })
                    .max()
                    .map_or(0, |id| id + 1);
                    update_retained = true;
                    RetainedPointConfig { id, name: point.name.clone(), _type: point.type_.clone() }
                }).id;
            self.cache
                .entry(owner.to_owned())
                .or_insert(vec![])
                .push(point.clone());
        }
        if update_retained {
            self.write(&self.path, &retained).unwrap();
            self.sql_write(&retained)
        }
        info!("{}.points | Caching Point's from '{}' - Ok", self.id, owner);
    }
    ///
    /// Returns configuration of the Point's
    pub fn points(&mut self) -> IndexMap<String, Vec<PointConfig>> {
        self.cache.clone()
    }
    ///
    /// Creates directiry (all necessary folders in the 'path' if not exists)
    ///  - path is relative, will be joined with current working dir
    fn create_dir(self_id: &str, path: &str) -> Result<PathBuf, String> {
        let current_dir = env::current_dir().unwrap();
        let path = current_dir.join(path);
        match path.exists() {
            true => Ok(path),
            false => {
                match fs::create_dir_all(&path) {
                    Ok(_) => Ok(path),
                    Err(err) => {
                        let message = format!("{}.create_dir | Error create path: '{:?}'\n\terror: {:?}", self_id, path, err);
                        error!("{}", message);
                        Err(message)
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
    fn read<P: AsRef<Path> + AsRef<OsStr> + std::fmt::Display>(&self, path: P) -> HashMapFxHasher<String, HashMapFxHasher<String, RetainedPointConfig>> {
        match fs::read_to_string(&path) {
            Ok(json_string) => {
                match serde_json::from_str(&json_string) {
                    Ok(config) => {
                        return config
                    }
                    Err(err) => {
                        log::warn!("{}.read | Error in config: {:?}\n\terror: {:?}", self.id, json_string, err);
                    }
                }
            }
            Err(err) => {
                debug!("{}.read | File {} reading error: {:?}", self.id, path, err);
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
    fn write<P: AsRef<Path>, S: Serialize>(&self, path: P, points: S) -> Result<(), String> {
        let path = Path::new(path.as_ref());
        match Self::create_dir(&self.id, path.parent().unwrap().to_str().unwrap()) {
            Ok(_) => {
                match fs::OpenOptions::new().truncate(true).create(true).write(true).open(path) {
                    Ok(f) => {
                        match serde_json::to_writer_pretty(f, &points) {
                            Ok(_) => Ok(()),
                            Err(err) => Err(format!("{}.read | Error writing to file: '{:?}'\n\terror: {:?}", self.id, path, err)),
                        }
                    }
                    Err(err) => {
                        Err(format!("{}.read | Error open file: '{:?}'\n\terror: {:?}", self.id, path, err))
                    }
                }
            }
            Err(err) => {
                error!("{:#?}", err);
                Err(err)
            }
        }
    }
    ///
    /// Stores points into the database
    fn sql_write(&self, retained: &RetainedCahe) {
        match &self.api {
            Some(api) => {
                let api_keep_alive = true;
                let sql_keep_alive = true;
                let mut request = ApiRequest::new(
                    &self.id,
                    &api.address,
                    &api.auth_token,
                    ApiQuery::new(
                        ApiQueryKind::Sql(ApiQuerySql::new(&api.database, "select 1;")),
                        sql_keep_alive,
                    ),
                    api_keep_alive,
                    false,
                );
                _ = self.sql_request(&mut request, "truncate public.tags;", api_keep_alive);
                for (_owner, points) in retained {
                    for point in points.values() {
                        let sql = format!("insert into public.tags (id, type, name) values ({},'{:?}','{}');", point.id, point._type, point.name);
                        _ = self.sql_request(&mut request, &sql, api_keep_alive);
                    }
                }
            }
            None => log::warn!("{}.sql_write | Database cant be updates, api is not specified", self.id),
        }
    }
    ///
    /// Make the sql request to store ponts to the database
    fn sql_request(&self, request: &mut ApiRequest, sql: &str, keep_alive: bool) -> Result<ApiReply, String> {
        match &self.api {
            Some(api) => {
                let query = ApiQuery::new(
                    ApiQueryKind::Sql(ApiQuerySql::new(&api.database, sql)),
                    true,
                );
                match request.fetch(&query, keep_alive) {
                    Ok(reply) => {
                        if log::max_level() > log::LevelFilter::Debug {
                            let reply_str = std::str::from_utf8(&reply).unwrap();
                            debug!("{}.send | reply str: {:?}", &self.id, reply_str);
                        }
                        match serde_json::from_slice(&reply) {
                            Ok(reply) => Ok(reply),
                            Err(err) => {
                                let reply = match std::str::from_utf8(&reply) {
                                    Ok(reply) => reply.to_string(),
                                    Err(err) => concat_string!(self.id, ".send | Error parsing reply to utf8 string: ", err.to_string()),
                                };
                                let message = concat_string!(self.id, ".send | Error parsing API reply: {:?} \n\t reply was: {:?}", err.to_string(), reply);
                                log::warn!("{}", message);
                                Err(message)
                            }
                        }
                    }
                    Err(err) => {
                        let message = concat_string!(self.id, ".send | Error sending API request: {:?}", err);
                        log::warn!("{}", message);
                        Err(message)
                    }
                }
            }
            None => {
                let message = concat_string!("{}.sql_request | Database cant be updates, api is not specified", self.id);
                log::warn!("{}", message);
                Err(message)
            }
        }
    }
}
///
/// Private wroper for Point to be stored
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct RetainedPointConfig {
    pub id: usize,
    pub name: String,
    #[serde(rename = "type")]
    #[serde(alias = "type", alias = "Type")]
    pub _type: PointConfigType,
}
///
/// Table parameters to acces and store Point's Id's into the databases table
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetainPointApi {
    table: String,
    address: String,
    auth_token: String,
    database: String,
}
//
//
impl RetainPointApi {
    ///
    /// 
    pub fn new(table: impl Into<String>, address: impl Into<String>, auth_token: impl Into<String>, database: impl Into<String>) -> Self {
        Self { 
            table: table.into(),
            address: address.into(),
            auth_token: auth_token.into(),
            database: database.into()
        }
    }
}
//
//
impl Default for RetainPointApi {
    ///
    /// **Returns `RetainPointIdTable` with the default walues**
    /// 
    /// ```
    /// RetainPointApi {
    ///    table: "public.tags",
    ///    address: "0.0.0.0:8080",
    ///    auth_token: "123!@#",
    ///    database: "crane_data_server",
    /// }
    /// ```
    fn default() -> Self {
        Self {
            table: "public.tags".to_owned(),
            address: "0.0.0.0:8080".to_owned(),
            auth_token: "123!@#".to_owned(),
            database: "crane_data_server".to_owned(),
        }
    }
}