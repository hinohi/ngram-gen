use std::{collections::HashMap, hash::Hash};

use rand::Rng;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Dist<K>
where
    K: Hash + Eq,
{
    total_counts: u32,
    map: HashMap<K, u32>,
}

impl<K> Dist<K>
where
    K: Hash + Eq,
{
    pub fn new() -> Dist<K> {
        Dist {
            total_counts: 0,
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: K) {
        self.total_counts += 1;
        *self.map.entry(key).or_insert(0) += 1;
    }

    pub fn from_hashmap(map: HashMap<K, u32>) -> Dist<K> {
        let total_counts = map.values().sum();
        Dist { total_counts, map }
    }

    pub fn sample<R: Rng>(&self, rng: &mut R) -> &K {
        let n = rng.gen_range(0..self.total_counts);
        let mut s = 0;
        for (key, &count) in self.map.iter() {
            s += count;
            if s >= n {
                return key;
            }
        }
        unreachable!();
    }
}

impl<K> Default for Dist<K>
where
    K: Hash + Eq,
{
    fn default() -> Dist<K> {
        Dist::new()
    }
}
