use std::option::Option;

pub struct Lateinit<'a, T> {
    val: Option<T>,
    init: &'a dyn Fn() -> T,
}

impl Lateinit<_> {
    pub fn new<T>(initializer: &dyn Fn() -> T) -> Self<T> {
        Self {val: None, init: initializer}
    }
}

impl<T> Lateinit<T> {
    pub fn unwrap(&mut self) -> T {
        self.val.get_or_insert(&self.init())
    }
}
