use std::{collections::HashMap, hash};

use maplit::hashmap;

use crate::HasDigest;

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
        digest
    }
}
