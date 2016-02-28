pub mod map {
    use Cap;
    use collections::stdhash::map::HashMap as RHashMap;
    use core::hash::{Hash, BuildHasher};
    use core::cmp::Eq;
    use core::borrow::Borrow;

    pub use collections::stdhash::map::{RandomState, Keys, Entry,
                                        Values, Iter, IterMut, Drain};

    pub struct HashMap<K, V, S = RandomState>(RHashMap<K, V, S>);

    impl<K: Hash + Eq, V> HashMap<K, V, RandomState> {
        pub fn new() -> HashMap<K, V, RandomState> {
            HashMap(RHashMap::new())
        }
        pub fn with_capacity(_: &Cap, capacity: usize) -> HashMap<K, V, RandomState> {
            HashMap(RHashMap::with_capacity(capacity))
        }
    }

    impl<K, V, S> HashMap<K, V, S> where K: Eq + Hash, S: BuildHasher {
        pub fn capacity(&self) -> usize {
            self.0.capacity()
        }

        pub fn reserve(&mut self, _: &Cap, additional: usize) {
            self.0.reserve(additional)
        }

        pub fn shrink_to_fit(&mut self, _: &Cap) {
            self.0.shrink_to_fit()
        }

        pub fn keys<'a>(&'a self) -> Keys<'a, K, V> {
            self.0.keys()
        }

        pub fn values<'a>(&'a self) -> Values<'a, K, V> {
            self.0.values()
        }

        pub fn iter(&self) -> Iter<K, V> {
            self.0.iter()
        }

        pub fn iter_mut(&mut self) -> IterMut<K, V> {
            self.0.iter_mut()
        }

        pub fn entry(&mut self, key: K) -> Entry<K, V> {
            self.0.entry(key)
        }

        pub fn len(&self) -> usize {
            self.0.len()
        }

        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }

        pub fn drain(&mut self) -> Drain<K, V> {
            self.0.drain()
        }

        pub fn clear(&mut self) {
            self.0.clear()
        }

        pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
            where K: Borrow<Q>, Q: Hash + Eq
        {
            self.0.get(k)
        }

        pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool
            where K: Borrow<Q>, Q: Hash + Eq
        {
            self.0.contains_key(k)
        }

        pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
            where K: Borrow<Q>, Q: Hash + Eq
        {
            self.0.get_mut(k)
        }

        pub fn insert(&mut self, _: &Cap, k: K, v: V) -> Option<V> {
            self.0.insert(k, v)
        }

        pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
            where K: Borrow<Q>, Q: Hash + Eq
        {
            self.0.remove(k)
        }
    }
}

pub mod set {
}
