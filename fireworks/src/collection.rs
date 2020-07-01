pub struct Collection<T> {
    items: Vec<T>,
}

impl<T> Collection<T> {
    pub fn new() -> Collection<T> {
        Collection { items: vec![] }
    }

    pub fn attach(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn accept_mut<F>(&mut self, visit_func: F)
    where
        F: Fn(&mut T) -> T,
    {
        self.items = self.items.iter_mut().map(visit_func).collect();
    }

    pub fn accept<F>(&self, visit_func: F)
    where
        F: Fn(&T) -> (),
    {
        self.items.iter().for_each(visit_func);
    }
}
