
use crate::ast::Type;


use std::collections::HashMap;

pub type StdLibFn = Box<dyn Fn(Type) -> Type>;

trait MyClone {
    fn my_clone(&self) -> Box<dyn Fn(Type) -> Type>;
}

impl<T> MyClone for T
where
    T: Fn(Type) -> Type + Clone + 'static,
{
    fn my_clone(&self) -> Box<dyn Fn(Type) -> Type> {
        Box::new(self.clone())
    }
}

pub fn standard_library() -> HashMap<String, StdLibFn> {
    let mut std_lib: HashMap<String, StdLibFn>  = HashMap::new();

    std_lib.insert(
        "print".to_string(),
        Box::new(|args: Type| {
            print(args);
            Type::Void
        }),
    );

    std_lib
}

pub fn print(value: Type) -> Type {
    println!("Print function {:?}", value);
	return Type::Void;
}