use core::hash::Hash;

pub trait AsImHashSet<A: Hash + Eq + Clone>: Clone {
    fn insert(&mut self, a: A) -> Option<A>;
    fn to_inserted(&mut self, a: A) -> (Self, Option<A>) {
        let mut new = self.clone();

        let res = new.insert(a);

        (new, res)
    }

    fn remove<BA>(&mut self, a: &BA) -> Option<A>
    where
        BA: Hash + Eq + ?Sized,
        A: std::borrow::Borrow<BA>;
    fn to_removed<BA>(&self, a: &BA) -> (Self, Option<A>)
    where
        BA: Hash + Eq + ?Sized,
        A: std::borrow::Borrow<BA>,
    {
        let mut new = self.clone();

        let res = new.remove(a);

        (new, res)
    }

    fn retain<F: FnMut(&A) -> bool>(&mut self, f: F);
    fn to_retained<F: FnMut(&A) -> bool>(&self, f: F) -> Self {
        let mut new = self.clone();

        new.retain(f);

        new
    }

    fn to_extended<I: IntoIterator<Item = A>>(&self, iter: I) -> Self
    where
        Self: Extend<A>,
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

impl<A: Hash + Eq + Clone, S: std::hash::BuildHasher> AsImHashSet<A> for im_rc::HashSet<A, S> {
    fn insert(&mut self, a: A) -> Option<A> {
        im_rc::HashSet::insert(self, a)
    }

    fn remove<BA>(&mut self, a: &BA) -> Option<A>
    where
        BA: Hash + Eq + ?Sized,
        A: std::borrow::Borrow<BA>,
    {
        im_rc::HashSet::remove(self, a)
    }

    fn retain<F: FnMut(&A) -> bool>(&mut self, f: F) {
        im_rc::HashSet::retain(self, f)
    }
}
