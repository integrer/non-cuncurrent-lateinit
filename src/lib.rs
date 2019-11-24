use std::{
    cell::{Ref, RefCell, RefMut},
    option::Option,
    rc::Rc,
};

pub struct Lateinit<'a, T> {
    val: Rc<RefCell<Option<T>>>,
    init: &'a dyn Fn() -> T,
}

impl<'a, T> Lateinit<'a, T> {
    pub fn new(initializer: &'a dyn Fn() -> T) -> Lateinit<T> {
        Self {
            val: Rc::new(RefCell::new(None)),
            init: initializer,
        }
    }

    fn init(&self, ref_mut: &mut RefMut<Option<T>>) {
        if ref_mut.is_none() {
            **ref_mut = Some((self.init)())
        }
    }

    pub fn borrow(&self) -> Ref<T> {
        {
            self.init(&mut self.val.borrow_mut());
        }
        Ref::map(self.val.borrow(), |v| v.as_ref().unwrap())
    }

    pub fn borrow_mut(&mut self) -> RefMut<T> {
        let mut r = self.val.borrow_mut();
        self.init(&mut r);
        RefMut::map(r, |v| v.as_mut().unwrap())
    }
}
