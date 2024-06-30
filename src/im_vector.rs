pub trait AsImVec<A: Clone>: Clone {
    fn push_back(&mut self, value: A);
    fn to_pushed_back(&self, value: A) -> Self {
        let mut new = self.clone();

        new.push_back(value);

        new
    }

    fn push_front(&mut self, value: A);
    fn to_pushed_front(&self, value: A) -> Self {
        let mut new = self.clone();

        new.push_front(value);

        new
    }

    fn pop_back(&mut self) -> Option<A>;
    fn to_pop_back(&self) -> (Self, Option<A>) {
        let mut new = self.clone();

        let res = new.pop_back();

        (new, res)
    }

    fn pop_front(&mut self) -> Option<A>;
    fn to_pop_front(&self) -> (Self, Option<A>) {
        let mut new = self.clone();

        let res = new.pop_front();

        (new, res)
    }

    fn sort_by<F: Fn(&A, &A) -> std::cmp::Ordering>(&mut self, cmp: F);
    fn to_sorted_by<F: Fn(&A, &A) -> std::cmp::Ordering>(&self, cmp: F) -> Self {
        let mut new = self.clone();

        new.sort_by(cmp);

        new
    }

    fn insert(&mut self, index: usize, value: A);
    fn to_inserted(&mut self, index: usize, value: A) -> Self {
        let mut new = self.clone();

        new.insert(index, value);

        new
    }

    fn sort(&mut self)
    where
        A: Ord;
    fn to_sorted(&self) -> Self
    where
        A: Ord,
    {
        let mut new = self.clone();

        new.sort();

        new
    }

    fn insert_ord(&mut self, item: A)
    where
        A: Ord;
    fn to_inserted_ord(&mut self, item: A) -> Self
    where
        A: Ord,
    {
        let mut new = self.clone();

        new.insert_ord(item);

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

impl<A: Clone> AsImVec<A> for im_rc::Vector<A> {
    fn push_back(&mut self, value: A) {
        im_rc::Vector::push_back(self, value)
    }

    fn push_front(&mut self, value: A) {
        im_rc::Vector::push_front(self, value)
    }

    fn pop_back(&mut self) -> Option<A> {
        im_rc::Vector::pop_back(self)
    }

    fn pop_front(&mut self) -> Option<A> {
        im_rc::Vector::pop_front(self)
    }

    fn sort_by<F: Fn(&A, &A) -> std::cmp::Ordering>(&mut self, cmp: F) {
        im_rc::Vector::sort_by(self, cmp)
    }

    fn insert(&mut self, index: usize, value: A) {
        im_rc::Vector::insert(self, index, value)
    }

    fn sort(&mut self)
    where
        A: Ord,
    {
        im_rc::Vector::sort(self)
    }

    fn insert_ord(&mut self, item: A)
    where
        A: Ord,
    {
        im_rc::Vector::insert_ord(self, item)
    }
}

impl<A: Clone> AsImVec<A> for im::Vector<A> {
    fn push_back(&mut self, value: A) {
        im::Vector::push_back(self, value)
    }

    fn push_front(&mut self, value: A) {
        im::Vector::push_front(self, value)
    }

    fn pop_back(&mut self) -> Option<A> {
        im::Vector::pop_back(self)
    }

    fn pop_front(&mut self) -> Option<A> {
        im::Vector::pop_front(self)
    }

    fn sort_by<F: Fn(&A, &A) -> std::cmp::Ordering>(&mut self, cmp: F) {
        im::Vector::sort_by(self, cmp)
    }

    fn insert(&mut self, index: usize, value: A) {
        im::Vector::insert(self, index, value)
    }

    fn sort(&mut self)
    where
        A: Ord,
    {
        im::Vector::sort(self)
    }

    fn insert_ord(&mut self, item: A)
    where
        A: Ord,
    {
        im::Vector::insert_ord(self, item)
    }
}
