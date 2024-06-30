pub trait AsImOrdMap<K: Ord + Clone, V: Clone>: Clone {
    fn insert(&mut self, k: K, v: V) -> Option<V>;
    fn to_inserted(&self, k: K, v: V) -> (Self, Option<V>) {
        let mut new = self.clone();

        let res = new.insert(k, v);

        (new, res)
    }

    fn remove<BK: Ord>(&mut self, k: &BK) -> Option<V>
    where
        K: std::borrow::Borrow<BK>;
    fn to_removed<BK: Ord>(&mut self, k: &BK) -> (Self, Option<V>)
    where
        K: std::borrow::Borrow<BK>,
    {
        let mut new = self.clone();

        let res = new.remove(k);

        (new, res)
    }

    fn remove_with_key<BK: Ord>(&mut self, k: &BK) -> Option<(K, V)>
    where
        K: std::borrow::Borrow<BK>;
    fn to_removed_with_key<BK: Ord>(&mut self, k: &BK) -> (Self, Option<(K, V)>)
    where
        K: std::borrow::Borrow<BK>,
    {
        let mut new = self.clone();

        let res = new.remove_with_key(k);

        (new, res)
    }

    fn to_extended<I: IntoIterator<Item = (K, V)>>(&self, iter: I) -> Self
    where
        Self: Extend<(K, V)>,
    {
        let mut new = self.clone();

        new.extend(iter);

        new
    }

    fn iter_clone(&self) -> Self::IntoIter
    where
        Self: IntoIterator,
    {
        self.clone().into_iter()
    }
}

impl<K: Ord + Clone, V: Clone> AsImOrdMap<K, V> for im_rc::OrdMap<K, V> {
    fn insert(&mut self, k: K, v: V) -> Option<V> {
        im_rc::OrdMap::insert(self, k, v)
    }

    fn remove<BK: Ord>(&mut self, k: &BK) -> Option<V>
    where
        K: std::borrow::Borrow<BK>,
    {
        im_rc::OrdMap::remove(self, k)
    }

    fn remove_with_key<BK: Ord>(&mut self, k: &BK) -> Option<(K, V)>
    where
        K: std::borrow::Borrow<BK>,
    {
        im_rc::OrdMap::remove_with_key(self, k)
    }
}

impl<K: Ord + Clone, V: Clone> AsImOrdMap<K, V> for im::OrdMap<K, V> {
    fn insert(&mut self, k: K, v: V) -> Option<V> {
        im::OrdMap::insert(self, k, v)
    }

    fn remove<BK: Ord>(&mut self, k: &BK) -> Option<V>
    where
        K: std::borrow::Borrow<BK>,
    {
        im::OrdMap::remove(self, k)
    }

    fn remove_with_key<BK: Ord>(&mut self, k: &BK) -> Option<(K, V)>
    where
        K: std::borrow::Borrow<BK>,
    {
        im::OrdMap::remove_with_key(self, k)
    }
}
