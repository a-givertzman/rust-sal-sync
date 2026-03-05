use std::{fmt::Debug, hash::BuildHasherDefault, sync::Arc};
use hashers::fx_hash::FxHasher;
use sal_core::error::Error;
use crate::{collections::FxDashMap, services::entity::Point, sync::{RwLock, channel::Sender}};
///
/// Unique id of the service receiving the Point's by the subscription
/// This id used to identify the service produced the Points. 
/// To avoid send back self produced Point's.
type ReceiverId = usize;
///
/// Destination of the point,
/// Currently it's just a concat of the Point.cot & Point.id 
type PointDest = String; 
///
/// Оптимизированная структура хранения.
/// Вместо Map используем отсортированный Vec для молниеносной итерации.
type SubscriberList = Arc<Vec<(ReceiverId, Sender<Point>)>>;
// type SubscriberList = Arc<FxDashMap<ReceiverId, Sender<Point>>>;

///
/// Contains map of Sender's
/// - Where Sender - is pair of String ID & Sender<PointType>
#[derive(Clone)]
pub struct Subscriptions {
    dbg: String,
    /// Справочник для расширения Multicast подписок (добавление новых PointDest к существующим получателям)
    registry: FxDashMap<ReceiverId, Sender<Point>>,
    /// Multicast подписки (по конкретным PointDest)
    multicast: FxDashMap<PointDest, SubscriberList>,
    /// Broadcast подписки (На все возможные PointDest)
    broadcast: Arc<RwLock<SubscriberList>>,
}
//
// 
impl Subscriptions {
    ///
    /// Creates new instance of Subscriptions
    pub fn new(parent: impl Into<String>, ) -> Self {
        Self {
            dbg: format!("{}/Subscriptions", parent.into()),
            registry: FxDashMap::with_hasher(BuildHasherDefault::<FxHasher>::default()),
            multicast: FxDashMap::with_hasher(BuildHasherDefault::<FxHasher>::default()),
            broadcast: Arc::new(RwLock::new(Arc::new(vec![]))),
        }
    }
    ///
    /// Adds subscription for `receiver_id` with destination 
    /// 
    /// COW Write: Медленнее (Copy), но безопасно для Readers
    pub fn add_multicast(&self, receiver_id: usize, destination: &str, sender: Sender<Point>) {
        // 1. Обновляем Registry новым получателем для будущего возможного расширения подписки
        self.registry.entry(receiver_id).or_insert(sender.clone());
        // 2. Атомарная транзакция для Multicast
        // DashMap держит Write Lock на этот бакет, пока выполняется замыкание.
        // Никто другой не сможет прочитать или записать в этот ключ, пока мы не закончим.
        self.multicast.entry(destination.to_owned())
            .and_modify(|arc_vec| {
                // Внутри этого блока мы в безопасности (Critical Section)
                // 1. Проверяем, есть ли уже такой получатель
                if !arc_vec.iter().any(|(id, _)| *id == receiver_id) {
                    // 2. Клонируем если нужно добавить
                    let mut new_vec = (**arc_vec).clone();
                    new_vec.push((receiver_id, sender.clone()));
                    // 3. Подменяем обновленный массив получателей
                    *arc_vec = Arc::new(new_vec);
                }
            })
            .or_insert_with(|| {
                // Если ключа не было - создаем новый
                Arc::new(vec![(receiver_id, sender)])
            });
    }
    ///
    /// Extends subscription for `receiver_id` if exists, otherwise returns error
    pub fn extend_multicast(&self, receiver_id: usize, destination: &str) -> Result<(), Error> {
        let error = Error::new(&self.dbg, "extend_multicast");
        log::trace!("{}.extend_multicast | Extending (multicast) for receiver: {} ({})...", self.dbg, destination, receiver_id);
        // 1. Берем из Registry получателя если такой есть
        match self.registry.get(&receiver_id).map(|s| s.clone()) {
            Some(sender) => {
                // 2. Атомарная транзакция для Multicast
                self.multicast.entry(destination.to_owned())
                    .and_modify(|arc_vec| {
                        // Внутри этого блока мы в безопасности (Critical Section)
                        // 1. Проверяем, есть ли уже такой получатель
                        if !arc_vec.iter().any(|(id, _)| *id == receiver_id) {
                            // 2. Клонируем массив получателей если нужно добавить
                            let mut new_vec = (**arc_vec).clone();
                            new_vec.push((receiver_id, sender.clone()));
                            // 3. Подменяем обновленный массив получателей
                            *arc_vec = Arc::new(new_vec);
                        }
                    })
                    .or_insert_with(|| {
                        // Если ключа не было - создаем новый
                        Arc::new(vec![(receiver_id, sender)])
                    });
                log::trace!("{}.extend_multicast | Extending (multicast) for receiver: {} ({}) - Ok", self.dbg, destination, receiver_id);
                Ok(())
            }
            None => {
                log::warn!("{}.extend_multicast | Extending (multicast) for receiver: {} ({receiver_id}) - Receiver '{receiver_id}' - not found", self.dbg, destination);
                Err(error.err(format!("Receiver '{}' - not found in subscriptions", receiver_id)))
            }
        }
    }
    ///
    /// Adds subscription for receiver_id without destination, all destinations will be received
    pub fn add_broadcast(&self, receiver_id: usize, sender: Sender<Point>) {
        let mut lock = self.broadcast.write();
        let mut new_vec = (**lock).clone();
        new_vec.push((receiver_id, sender));
        *lock = Arc::new(new_vec);
    }
    ///
    /// ## Returns all pairs of `key`, `Sender`'s for the specified `point_id`
    /// 
    /// Worck slow because returns vactor of `Sender`'s copies
    /// 
    /// For faster applications use `get_view` method
    pub fn get(&self, point_id: &str) -> Vec<(usize, Sender<Point>)> {
        let broadcast = self.broadcast.read();
        // Pre-allocation для оптимизации, мы знаем точный размер заранее, избегаем reallocations
        let mc_len = self.multicast.get(point_id).map(|v| v.len()).unwrap_or(0);
        let capacity = broadcast.len() + mc_len;
        let mut result = Vec::with_capacity(capacity);
        // Быстрое копирование broadcast
        result.extend(broadcast.iter().map(|(id, s)| (*id, s.clone())));
        // Быстрое копирование multicast
        if let Some(list) = self.multicast.get(point_id) {
             result.extend(list.iter().map(|(id, s)| (*id, s.clone())));
        }
        result
    }
    ///
    /// ## Returns a snapshort of `Sender`'s for the specified `point_id`
    /// 
    /// Fastest read access, non-blocking non-copy.
    pub fn get_view(&self, point_id: &str) -> SubscribersView {
        // 1. Дешевый клон Arc для Broadcast (просто инкремент счетчика ссылок)
        let broadcast = self.broadcast.read().clone();
        // 2. Дешевый клон Arc для Multicast (если есть)
        let multicast = self.multicast.get(point_id).map(|v| v.clone());
        SubscribersView {
            broadcast,
            multicast,
        }
    }
    ///
    /// Removes single subscription by Point Id for receiver ID
    pub fn remove(&self, receiver_id: usize, destination: &str) -> Result<(), Error> {
        let error = Error::new(&self.dbg, "remove");
        // 1. Обновляем Registry новым получателем для будущего возможного расширения подписки
    // self.registry.entry(receiver_id).or_insert(sender.clone());
        // 2. Атомарная транзакция для Multicast
        // DashMap держит Write Lock на этот бакет, пока выполняется замыкание.
        // Никто другой не сможет прочитать или записать в этот ключ, пока мы не закончим.
        let mut removed = false;
        self.multicast.entry(destination.to_owned())
            .and_modify(|arc_vec| {
                // Внутри этого блока мы в безопасности (Critical Section)
                // 1. Ищем получателя которого надо удалить
                if let Some(i) = arc_vec.iter().enumerate().find_map(|(i, (id, _))| (*id == receiver_id).then(|| i)) {
                    // 2. Клонируем массив получателей если нужно удалить
                    let mut new_vec = (**arc_vec).clone();
                    new_vec.remove(i);
                    // 3. Подменяем обновленный массив получателей
                    *arc_vec = Arc::new(new_vec);
                }
                removed = true;
            });
        match removed {
            true => Ok(()),
            false => Err(error.err(format!("Subscription '{destination}' - NOT FOUND for receiver '{receiver_id}'"))),
        }
    }
    ///
    /// Removes all subscriptions for `receiver_id`
    pub fn remove_all(&self, receiver_id: usize) -> Result<(), Error> {
        let error = Error::new(&self.dbg, "remove_all");
        let mut errors = vec![];
        let destinations: Vec<PointDest> = self.multicast.iter().map(|r| r.key().clone()).collect();
        for destination in destinations {
            if let Err(err) = self.remove(receiver_id, &destination) {
                errors.push(format!("{}.remove_all | Can't remove multicast subscription for receiver '{}', error: \n\t {:?}", self.dbg, receiver_id, err));
            }
            // match self.multicast.get(&destination).map(|r| r.value().clone()) {
            //     Some(senders) => {
            //         match senders.remove(receiver_id) {
            //             Some(_) => {
            //                 changed |= true;
            //             }
            //             None => {
            //                 messages.push(format!("{}.run | Multicast Subscription '{}', receiver '{}' - not found", self.dbg, destination, receiver_id));
            //             }
            //         }
            //     }
            //     None => {
            //         messages.push(format!("{}.run | Multicast Subscription '{}' - not found", self.dbg, destination));
            //     }
            // }
        }
        self.registry.remove(&receiver_id);
        // Удаляем Broadcast подписку для получателя (receiver_id)
        let broadcast= self.broadcast.read();
        if let Some(i) = broadcast.iter().enumerate().find_map(|(i, (id, _))| (*id == receiver_id).then(|| i)) {
            // 2. Клонируем массив получателей если нужно удалить
            let mut new_vec = (**broadcast).clone();
            drop(broadcast);
            new_vec.remove(i);
            // 3. Подменяем обновленный массив получателей
            *self.broadcast.write() = Arc::new(new_vec);
        }
        match errors.is_empty() {
            true => Ok(()),
            false => Err(error.err(errors.join("\n"))),
        }
    }
    ///
    /// Removes all subscriptions
    pub fn exit(&self) {
        self.registry.clear();
        *self.broadcast.write() = Arc::new(vec![]);
        self.multicast.clear();
    }
}
//
// 
impl Debug for Subscriptions {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Subscriptions")
            .field("dbg", &self.dbg)
            .finish()
    }
}
///
/// Легковесная структура-вьювер.
/// Она владеет ссылками на данные (через Arc), поэтому данные не исчезнут,
/// пока жив этот объект.
pub struct SubscribersView {
    broadcast: SubscriberList,
    multicast: Option<SubscriberList>,
}
//
impl SubscribersView {
    /// Возвращает итератор по всем подписчикам (broadcast + multicast)
    pub fn iter(&self) -> impl Iterator<Item = &(usize, Sender<Point>)> {
        // Создаем цепочку итераторов: сначала broadcast, потом multicast (если есть)
        let mc_iter = self.multicast.as_deref().into_iter().flat_map(|v| v.iter());
        self.broadcast.iter().chain(mc_iter)
    }
}
