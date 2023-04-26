use crate::PygoTypes::pygo_type::Type;

use std::rc::Rc;
pub struct Function{
	pub func: Rc<dyn Fn(Type) -> Type>
}

impl Clone for Function {
    fn clone(&self) -> Self {
        Function{
			func: Rc::clone(&self.func)
		}
    }
}

impl Function {
    pub fn new<F>(f: F) -> Self
    where
        F: 'static + Fn(Type) -> Type,
    {
        Function{
			func: Rc::new(f)
		}
    }

    pub fn call(&self, arg: Type) -> Type {
        (self.func)(arg)
    }
}


