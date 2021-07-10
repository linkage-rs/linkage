#[derive(Debug)]
pub struct ZipperList<T, U> {
    prev: Vec<T>,
    current: U,
    next: Vec<T>,
}

#[derive(Debug, Clone)]
pub enum Item<T, U> {
    Other(T),
    Current(U),
}

impl<T, U> ZipperList<T, U> {
    pub fn current(&self) -> &U {
        &self.current
    }

    pub fn current_mut(&mut self) -> &mut U {
        &mut self.current
    }

    pub fn len(&self) -> usize {
        self.prev.len() + 1 + self.next.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = Item<&T, &U>> {
        self.prev
            .iter()
            .map(|item| Item::Other(item))
            .chain(std::iter::once(Item::Current(&self.current)))
            .chain(self.next.iter().map(|item| Item::Other(item)))
    }

    pub fn push(&mut self, item: T) {
        self.next.push(item);
    }
}

impl<T, U> ZipperList<T, U>
where
    T: From<U> + Clone,
    U: From<T>,
{
    pub fn select(&mut self, index: usize) -> bool {
        if index < self.prev.len() {
            let next_current = self.prev.remove(index);
            let old_current = std::mem::replace(&mut self.current, next_current.into());
            let mut next: Vec<_> = self.prev.drain(index..).collect();
            next.push(old_current.into());
            next.append(&mut self.next);
            self.next = next;
            true
        } else if index >= self.prev.len() + 1 && index < self.len() {
            let index = index - self.prev.len() - 1;
            let next_current = self.next.remove(index);
            let old_current = std::mem::replace(&mut self.current, next_current.into());
            self.prev.push(old_current.into());
            self.prev.append(&mut self.next.drain(..index).collect());
            true
        } else {
            false
        }
    }
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
