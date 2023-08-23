use std::{borrow::Borrow, collections::HashMap, hash::Hash};

// 封装 HashMap, 节省内存空间

#[derive(Debug, Default)]
pub struct CustomHashMap<K, V>
where
    K: Clone + Hash + Eq,
{
    data: Vec<(K, V)>,
    map: HashMap<K, usize>,
}

pub type CustomHashMapState<K, V> = (Vec<(K, V)>,);

impl<K, V> CustomHashMap<K, V>
where
    K: Clone + Hash + Eq,
{
    pub fn store(&mut self) -> CustomHashMapState<K, V> {
        let data = std::mem::take(&mut self.data);
        (data,)
    }

    pub fn restore(&mut self, state: CustomHashMapState<K, V>) {
        let _ = std::mem::replace(&mut self.data, state.0);
        self.map = self
            .data
            .iter()
            .enumerate()
            .map(|(index, item)| (item.0.clone(), index))
            .collect();
    }

    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map
            .get(k)
            .and_then(|index| self.data.get(*index))
            .and_then(|item| Some(&item.1))
    }

    pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let index = self.map.get(k);
        if let Some(index) = index {
            let item = self.data.remove(*index);
            return Some(item.1);
        }
        None
    }
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        let index = self.map.get(&k);
        if let Some(index) = index {
            let old = std::mem::replace(&mut self.data[*index], (k.clone(), v));
            return Some(old.1);
        } else {
            self.data.push((k.clone(), v));
            self.map.insert(k, self.data.len() - 1);
            return None;
        }
    }
    pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.contains_key(k)
    }
    pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map
            .get(k)
            .and_then(|index| self.data.get_mut(*index))
            .and_then(|item| Some(&mut item.1))
    }
}
