#[derive(Debug, Clone)]
pub struct ZipperList<T, U> {
    prev: Vec<T>,
    current: U,
    next: Vec<T>,
}

#[derive(Debug, Clone)]
pub enum Item<T, U> {
    Current(T),
    Other(U),
}

impl<T, U> ZipperList<T, U>
where
    T: From<U>,
    U: From<T>,
{
    pub fn current(&self) -> &U {
        &self.current
    }

    pub fn current_mut(&mut self) -> &mut U {
        &mut self.current
    }

    pub fn select_index(&mut self, i: usize) {}
}

impl<T, U> From<(Vec<T>, T, Vec<T>)> for ZipperList<T, U>
where
    U: From<T>,
{
    fn from(input: (Vec<T>, T, Vec<T>)) -> Self {
        Self {
            prev: input.0,
            current: input.1.into(),
            next: input.2,
        }
    }
}

impl<T, U> From<ZipperList<T, U>> for (Vec<T>, T, Vec<T>)
where
    T: From<U>,
{
    fn from(input: ZipperList<T, U>) -> Self {
        (input.prev, input.current.into(), input.next)
    }
}
