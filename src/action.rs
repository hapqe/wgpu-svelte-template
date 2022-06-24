pub struct Action<T> {
    listeners: Vec<Box<dyn Fn(T)>>,
}

impl<T> Default for Action<T> {
    fn default() -> Self {
        Action {
            listeners: Vec::new(),
        }
    }
}

impl<T> Action<T> {
    pub fn add<F>(&mut self, listener: F)
    where
        F: Fn(T) + 'static,
    {
        self.listeners.push(Box::new(listener));
    }

    pub fn invoke<A>(&self, arg: A)
    where
        A: Fn() -> T,
    {
        for listener in self.listeners.iter() {
            listener(arg());
        }
    }

    pub fn new() -> Self {
        Default::default()
    }
}
