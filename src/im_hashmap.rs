use core::hash::Hash;

pub trait AsImHashMap<K: Hash + Eq + Clone, V: Clone>: Clone {
    fn insert(&mut self, k: K, v: V) -> Option<V>;
    fn to_inserted(&self, k: K, v: V) -> (Self, Option<V>) {
        let mut new = self.clone();

        let res = new.insert(k, v);

        (new, res)
    }

    fn remove<BK: Hash + Eq + ?Sized>(&mut self, k: &BK) -> Option<V>
    where
        K: std::borrow::Borrow<BK>;
    fn to_removed<BK: Hash + Eq + ?Sized>(&mut self, k: &BK) -> (Self, Option<V>)
    where
        K: std::borrow::Borrow<BK>,
    {
        let mut new = self.clone();

        let res = new.remove(k);

        (new, res)
    }

    fn remove_with_key<BK: Hash + Eq + ?Sized>(&mut self, k: &BK) -> Option<(K, V)>
    where
        K: std::borrow::Borrow<BK>;
    fn to_removed_with_key<BK: Hash + Eq + ?Sized>(&mut self, k: &BK) -> (Self, Option<(K, V)>)
    where
        K: std::borrow::Borrow<BK>,
    {
        let mut new = self.clone();

        let res = new.remove_with_key(k);

        (new, res)
    }

    fn retain<F: FnMut(&K, &V) -> bool>(&mut self, f: F);
    fn to_retained<F: FnMut(&K, &V) -> bool>(&self, f: F) -> Self {
        let mut new = self.clone();

        new.retain(f);

        new
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

impl<K: Hash + Eq + Clone, V: Clone, S: std::hash::BuildHasher> AsImHashMap<K, V>
    for im_rc::HashMap<K, V, S>
{
    fn insert(&mut self, k: K, v: V) -> Option<V> {
        im_rc::HashMap::insert(self, k, v)
    }

    fn remove<BK: Hash + Eq + ?Sized>(&mut self, k: &BK) -> Option<V>
    where
        K: std::borrow::Borrow<BK>,
    {
        im_rc::HashMap::remove(self, k)
    }

    fn remove_with_key<BK: Hash + Eq + ?Sized>(&mut self, k: &BK) -> Option<(K, V)>
    where
        K: std::borrow::Borrow<BK>,
    {
        im_rc::HashMap::remove_with_key(self, k)
    }

    fn retain<F: FnMut(&K, &V) -> bool>(&mut self, f: F) {
        im_rc::HashMap::retain(self, f)
    }
}

impl<K: Hash + Eq + Clone, V: Clone, S: std::hash::BuildHasher> AsImHashMap<K, V>
    for im::HashMap<K, V, S>
{
    fn insert(&mut self, k: K, v: V) -> Option<V> {
        im::HashMap::insert(self, k, v)
    }

    fn remove<BK: Hash + Eq + ?Sized>(&mut self, k: &BK) -> Option<V>
    where
        K: std::borrow::Borrow<BK>,
    {
        im::HashMap::remove(self, k)
    }

    fn remove_with_key<BK: Hash + Eq + ?Sized>(&mut self, k: &BK) -> Option<(K, V)>
    where
        K: std::borrow::Borrow<BK>,
    {
        im::HashMap::remove_with_key(self, k)
    }

    fn retain<F: FnMut(&K, &V) -> bool>(&mut self, f: F) {
        im::HashMap::retain(self, f)
    }
}
