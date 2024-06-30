pub trait AsImOrdSet<A: Ord + Clone>: Clone {
    fn insert(&mut self, a: A) -> Option<A>;
    fn to_inserted(&mut self, a: A) -> (Self, Option<A>) {
        let mut new = self.clone();

        let res = new.insert(a);

        (new, res)
    }

    fn remove<BA: Ord>(&mut self, a: &BA) -> Option<A>
    where
        A: std::borrow::Borrow<BA>;
    fn to_removed<BA: Ord>(&self, a: &BA) -> (Self, Option<A>)
    where
        A: std::borrow::Borrow<BA>,
    {
        let mut new = self.clone();

        let res = new.remove(a);

        (new, res)
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

impl<A: Ord + Clone> AsImOrdSet<A> for im_rc::OrdSet<A> {
    fn insert(&mut self, a: A) -> Option<A> {
        im_rc::OrdSet::insert(self, a)
    }

    fn remove<BA: Ord>(&mut self, a: &BA) -> Option<A>
    where
        BA: Ord + ?Sized,
        A: std::borrow::Borrow<BA>,
    {
        im_rc::OrdSet::remove(self, a)
    }
}

impl<A: Ord + Clone> AsImOrdSet<A> for im::OrdSet<A> {
    fn insert(&mut self, a: A) -> Option<A> {
        im::OrdSet::insert(self, a)
    }

    fn remove<BA: Ord>(&mut self, a: &BA) -> Option<A>
    where
        BA: Ord + ?Sized,
        A: std::borrow::Borrow<BA>,
    {
        im::OrdSet::remove(self, a)
    }
}
