use std::{fmt::Debug, hash::BuildHasherDefault, sync::{Arc, atomic::{AtomicUsize, Ordering}}};
use hashers::fx_hash::FxHasher;
use sal_core::error::Error;
use crate::{collections::{FxDashMap, FxDashSet}, services::entity::Point, sync::{RwLock, channel::Sender}};
///
/// Unique id of the service (TxId) receiving the Point's by the subscription
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
pub struct Subscriptions {
    dbg: String,
    /// Справочник для расширения Multicast подписок (добавление новых PointDest к существующим получателям)
    registry: FxDashMap<ReceiverId, Arc<ReceiverInfo>>,
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
    pub fn add_multicast(&self, receiver_id: ReceiverId, destination: &str, sender: Sender<Point>) {
        // Атомарная транзакция для Multicast
        // DashMap держит Write Lock на этот бакет, пока выполняется замыкание.
        // Никто другой не сможет прочитать или записать в этот ключ, пока мы не закончим.
        let mut added = false;
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
                    added = true;
                }
            })
            .or_insert_with(|| {
                added = true;
                // Если ключа не было - создаем новый
                Arc::new(vec![(receiver_id, sender.clone())])
            });
        if added {
            // Обновляем Registry новым получателем для будущего возможного расширения подписки
            self.registry.entry(receiver_id)
                .and_modify(|receiver_info| receiver_info.inc(Some(destination)))
                .or_insert(Arc::new(ReceiverInfo::new(sender, Some(destination))));
        }
    }
    ///
    /// Extends subscription for `receiver_id` if exists, otherwise returns error
    pub fn extend_multicast(&self, receiver_id: ReceiverId, destination: &str) -> Result<(), Error> {
        let error = Error::new(&self.dbg, "extend_multicast");
        log::trace!("{}.extend_multicast | Extending (multicast) for receiver: {} ({})...", self.dbg, destination, receiver_id);
        // 1. Берем из Registry получателя если такой есть
        let mut extended = false;
        match self.registry.get(&receiver_id).map(|receiver_info| receiver_info.clone()) {
            Some(receiver_info) => {
                // 2. Атомарная транзакция для Multicast
                self.multicast.entry(destination.to_owned())
                    .and_modify(|arc_vec| {
                        // Внутри этого блока мы в безопасности (Critical Section)
                        // 1. Проверяем, есть ли уже такой получатель
                        if !arc_vec.iter().any(|(id, _)| *id == receiver_id) {
                            // 2. Клонируем массив получателей если нужно добавить
                            let mut new_vec = (**arc_vec).clone();
                            new_vec.push((receiver_id, receiver_info.sender()));
                            // 3. Подменяем обновленный массив получателей
                            *arc_vec = Arc::new(new_vec);
                            extended = true;
                        }
                    })
                    .or_insert_with(|| {
                        // Если ключа не было - создаем новый
                        extended = true;
                        Arc::new(vec![(receiver_id, receiver_info.sender())])
                    });
                if extended {
                    receiver_info.inc(Some(destination));
                }
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
    pub fn add_broadcast(&self, receiver_id: ReceiverId, sender: Sender<Point>) {
        let mut lock = self.broadcast.write();
        let mut new_vec = (**lock).clone();
        match new_vec.iter_mut().find(|(id, _)| *id == receiver_id) {
            Some((_, old_sender)) => *old_sender = sender.clone(),
            None => new_vec.push((receiver_id, sender.clone())),
        }
        *lock = Arc::new(new_vec);
        // Обновляем Registry новым получателем для будущего возможного расширения подписки
        self.registry.entry(receiver_id)
            .and_modify(|receiver_info| receiver_info.inc(None::<PointDest>))
            .or_insert(Arc::new(ReceiverInfo::new(sender, None::<PointDest>)));
    }
    ///
    /// ## Returns all pairs of `ReceiverId`, `Sender`'s for the specified `point_id`
    /// 
    /// Worck slow because returns vactor of `Sender`'s copies
    /// 
    /// For faster applications use `get_view` method
    pub fn get(&self, destination: &str) -> Vec<(ReceiverId, Sender<Point>)> {
        let broadcast = self.broadcast.read();
        // Pre-allocation для оптимизации, мы знаем точный размер заранее, избегаем reallocations
        let mc_len = self.multicast.get(destination).map(|v| v.len()).unwrap_or(0);
        let capacity = broadcast.len() + mc_len;
        let mut result = Vec::with_capacity(capacity);
        // Быстрое копирование broadcast
        result.extend(broadcast.iter().map(|(id, s)| (*id, s.clone())));
        // Быстрое копирование multicast
        if let Some(list) = self.multicast.get(destination) {
             result.extend(list.iter().map(|(id, s)| (*id, s.clone())));
        }
        result
    }
    ///
    /// ## Returns a snapshort of `Sender`'s for the specified `point_id`
    /// 
    /// Fastest read access, non-blocking non-copy.
    pub fn get_view(&self, destination: &str) -> SubscribersView {
        // 1. Дешевый клон Arc для Broadcast (просто инкремент счетчика ссылок)
        let broadcast = self.broadcast.read().clone();
        // 2. Дешевый клон Arc для Multicast (если есть)
        let multicast = self.multicast.get(destination).map(|v| v.clone());
        SubscribersView {
            broadcast,
            multicast,
        }
    }
    ///
    /// Removes single subscription by Point Id for receiver ID
    pub fn remove(&self, receiver_id: ReceiverId, destinations: impl IntoIterator<Item = impl Into<PointDest>>) {
        for destination in destinations {
            let destination: PointDest = destination.into();
            // Атомарная транзакция для Multicast
            // DashMap держит Write Lock на этот бакет, пока выполняется замыкание.
            // Никто другой не сможет прочитать или записать в этот ключ, пока мы не закончим.
            if let dashmap::Entry::Occupied(mut entry) = self.multicast.entry(destination.clone()) {
                let arc_vec = entry.get_mut();
                // .and_modify(|arc_vec| {
                    // Внутри этого блока мы в безопасности (Critical Section)
                    // Ищем получателя которого надо удалить
                if let Some(pos) = arc_vec.iter().position(|(id, _)| *id == receiver_id) {
                    // Клонируем массив получателей если нужно удалить
                    let mut new_vec = (**arc_vec).clone();
                    new_vec.remove(pos);
                    match new_vec.is_empty() {
                        // Подменяем обновленный массив получателей
                        false => *arc_vec = Arc::new(new_vec),
                        // Удаляем массив получателей если он пуст
                        true => _ = entry.remove_entry(),
                    }
                    // Удаляем из Registry получателя
                    if let Some(receiver_info) = self.registry.get(&receiver_id) {
                        receiver_info.dec(Some(destination))
                    }
                }
            }
        }
        // Удаляем из Registry если получателя больше нет в подписках
        self.registry.remove_if(&receiver_id, |_, r| !r.is_subscribed());
    }
    ///
    /// Removes all subscriptions for `receiver_id`
    pub fn remove_all(&self, receiver_id: ReceiverId) {
        // Удаляем Broadcast подписку для получателя (receiver_id)
        let mut lock= self.broadcast.write();
        if let Some(pos) = lock.iter().position(|(id, _)| *id == receiver_id) {
            // 2. Клонируем массив получателей если нужно удалить
            let mut new_vec = (**lock).clone();
            new_vec.remove(pos);
            // 3. Подменяем обновленный массив получателей
            *lock = Arc::new(new_vec);
            self.registry.entry(receiver_id)
            .and_modify(|receiver_info| receiver_info.dec(None::<PointDest>));
        }
        let destinations: Vec<PointDest> = self.registry.get(&receiver_id)
            // .filter(|entry| entry.value().iter().any(|(id, _)| *id == receiver_id))
            .map(|entry| entry.destinations())
            .unwrap_or_default();
        if !destinations.is_empty() {
            self.remove(receiver_id, &destinations);
        } else {
            self.registry.remove_if(&receiver_id, |_, r| !r.is_subscribed());
        }
    }
    ///
    /// Returns true if [ReceiverId] has any subscriptions
    pub fn is_subscribed(&self, receiver_id: ReceiverId) -> bool {
        self.registry
            .get(&receiver_id)
            .map(|r| r.is_subscribed())
            .unwrap_or(false)
        // if self.broadcast.read().iter().any(|(id, _)| *id == receiver_id) {
        //     return true;
        // }
        // self.multicast.iter().any(|entry| entry.iter().any(|(id, _)| *id == receiver_id))
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
    pub fn iter(&self) -> impl Iterator<Item = &(ReceiverId, Sender<Point>)> {
        // Создаем цепочку итераторов: сначала broadcast, потом multicast (если есть)
        let mc_iter = self.multicast.as_deref().into_iter().flat_map(|v| v.iter());
        self.broadcast.iter().chain(mc_iter)
    }
}
///
/// Контейнер для хранения информации о подписчике
#[derive(Debug)]
struct ReceiverInfo {
    /// Sender получателя, `subscriptions` раз упомянутый в Broadcast и Multicast подписках
    sender: Sender<Point>,
    /// Количество подписок (Broadcast + Multicast)
    subscriptions: AtomicUsize,
    /// Нименования всех подписок получателя, что бы не искать их при удалении
    destinations: FxDashSet<PointDest>,
}
//
impl ReceiverInfo {
    ///
    /// Returns [ReceiverInfo] new instance
    /// - `sender` - Channel to the coresponding Receiver
    pub fn new(sender: Sender<Point>, destination: Option<impl Into<PointDest>>) -> Self {
        let destinations = FxDashSet::default();
        if let Some(dest) = destination {
            destinations.insert(dest.into());
        }
        Self {
            sender,
            subscriptions: AtomicUsize::new(1),
            destinations,
        }
    }
    ///
    /// Returns clone of the Receiver's Sender
    pub fn sender(&self) -> Sender<Point> {
        self.sender.clone()
    }
    ///
    /// Returns all `destinations` for the receiver
    pub fn destinations(&self) -> Vec<PointDest> {
        self.destinations.iter().map(|d| d.clone()).collect()
    }
    ///
    /// Increments by one a count of the Receiver's subscriptions
    pub fn inc(&self, destination: Option<impl Into<PointDest>>) {
        self.subscriptions.fetch_add(1, Ordering::AcqRel);
        if let Some(dest) = destination {
            self.destinations.insert(dest.into());
        }
    }
    ///
    /// Decrements by one a count of the Receiver's subscriptions
    pub fn dec(&self, destination: Option<impl Into<PointDest>>) {
        if self.subscriptions.load(Ordering::Acquire) > 0 {
            self.subscriptions.fetch_sub(1, Ordering::AcqRel);
        }
        if let Some(dest) = destination {
            self.destinations.remove(&dest.into());
        }
    }
    ///
    /// Returns `true` if number of subscriptions is not a zero
    pub fn is_subscribed(&self) -> bool {
        self.subscriptions.load(Ordering::Acquire) > 0
    }
}