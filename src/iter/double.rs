pub(crate) struct DoubleIter<T, U>
where
    T: Iterator<Item = U>
{
    inner: T,
    item0: Option<U>,
    item1: Option<U>,
}

impl<T, U> DoubleIter<T, U>
where
    T: Iterator<Item = U>
{
    pub(crate) fn new(inner: T) -> Self {
        let mut this = Self {
            inner,
            item0: None,
            item1: None,
        };

        this.bump();
        this.bump();

        this
    }

    pub(crate) fn bump(&mut self) {
        self.item0 = self.item1.take();
        self.item1 = self.inner.next();
    }
}

impl<T, U> Iterator for DoubleIter<T, U>
where
    T: Iterator<Item = U>,
    U: Clone
{
    type Item = (U, Option<U>);
    fn next(&mut self) -> Option<Self::Item> {
        if self.item0.is_some() {
            let item = Some((self.item0.take().unwrap(), self.item1.clone()));
            self.bump();
            item
        } else {
            None
        }
    }
}

pub(crate) trait IntoDoubleIter<T, I>
where
    T: Iterator<Item = I>
{
    fn into_double_iter(self) -> DoubleIter<T, I>;
}

impl<T, I> IntoDoubleIter<T, I> for T
where
    T: Iterator<Item = I>
{
    fn into_double_iter(self) -> DoubleIter<T, I> {
        DoubleIter::new(self)
    }
}
