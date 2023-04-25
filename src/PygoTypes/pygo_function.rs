use crate::PygoTypes::pygo_type::Type;

use std::rc::Rc;
pub struct MyFunc(Rc<dyn Fn(Type) -> Type>);

impl Clone for MyFunc {
    fn clone(&self) -> Self {
        MyFunc(Rc::clone(&self.0))
    }
}

impl MyFunc {
    pub fn new<F>(f: F) -> Self
    where
        F: 'static + Fn(Type) -> Type,
    {
        MyFunc(Rc::new(f))
    }

    pub fn call(&self, arg: Type) -> Type {
        (self.0)(arg)
    }
}


