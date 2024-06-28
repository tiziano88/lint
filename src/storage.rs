use leptos::*;
use std::{collections::HashMap, hash};

use leptos::{Signal, SignalGetUntracked};
use leptos_use::{
    storage::use_local_storage,
    utils::FromToStringCodec,
};
use maplit::hashmap;
use serde::Serialize;

use crate::{HasDigest, Node, D};

pub trait Store<T, D>
where
    T: HasDigest<Digest = D>,
{
    fn get(&self, digest: &D) -> Option<&T>;
    fn has(&self, digest: &D) -> bool;
    fn put(&mut self, value: T) -> D;
}

pub struct LocalStorage<T, D>
where
    T: HasDigest<Digest = D>,
{
    storage: HashMap<D, T>,
}

impl<T, D> LocalStorage<T, D>
where
    T: HasDigest<Digest = D>,
{
    pub fn new() -> Self {
        LocalStorage {
            storage: hashmap! {},
        }
    }
}

impl<T, D> Store<T, D> for LocalStorage<T, D>
where
    T: HasDigest<Digest = D>,
    D: Eq + hash::Hash + Clone,
{
    fn get(&self, digest: &D) -> Option<&T> {
        self.storage.get(digest)
    }

    fn has(&self, digest: &D) -> bool {
        self.storage.contains_key(digest)
    }

    fn put(&mut self, value: T) -> D {
        let digest = value.digest();
        self.storage.insert(digest.clone(), value);
        let (_state, set_state, _) = use_local_storage::<String, FromToStringCodec>("cc");
        set_state("xxx".to_string());
        digest
    }
}

pub fn get_item(digest: &D) -> Signal<Option<Node>> {
    let (item, _set_item, _) = use_local_storage::<String, FromToStringCodec>(digest.to_hex());
    Signal::derive(move || {
        let item = item();
        Node::deserialize(&item)
    })
}

pub fn get_item_untracked(digest: &D) -> Option<Node> {
    let (item, _set_item, _) = use_local_storage::<String, FromToStringCodec>(digest.to_hex());
    Node::deserialize(&item.get_untracked())
}

pub fn set_item(node: &Node) -> D {
    let digest = node.digest();
    let (_item, set_item, _) = use_local_storage::<String, FromToStringCodec>(digest.to_hex());
    let serialized_node = node.serialize();
    set_item(serialized_node);
    digest
}

pub fn set_root(d: &D) {
    logging::log!("set root: {}", d.to_hex());
    let (_item, set_item, _) = use_local_storage::<String, FromToStringCodec>("root");
    set_item(d.to_hex());
}

pub fn get_root() -> D {
    let (item, _set_item, _) = use_local_storage::<String, FromToStringCodec>("root");
    let hex = item();
    logging::log!("root: {}", &hex);
    D::from_hex(&hex)
}

pub fn get_value(key: &str) -> Signal<String> {
    let (value, _set_value, _) = use_local_storage::<String, FromToStringCodec>(key);
    Signal::derive(move || value())
}

pub fn set_value(key: &str, value: &str) {
    let (_value, set_value, _) = use_local_storage::<String, FromToStringCodec>(key);
    set_value(value.to_string());
}
