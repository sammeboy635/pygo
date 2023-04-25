use std::fmt::Debug;
use std::rc::Rc;

#[derive(Clone, Debug)]
enum Type {
    Int(i32),
}

use std::ops;
impl ops::Add for Type {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Type::Int(a), Type::Int(b)) => Type::Int(a + b),
            _ => unimplemented!(),
        }
    }
}
struct MyFunc(Rc<dyn Fn(Type) -> Type>);

impl Clone for MyFunc {
    fn clone(&self) -> Self {
        MyFunc(Rc::clone(&self.0))
    }
}

impl MyFunc {
    fn new<F>(f: F) -> Self
    where
        F: 'static + Fn(Type) -> Type,
    {
        MyFunc(Rc::new(f))
    }

    fn call(&self, arg: Type) -> Type {
        (self.0)(arg)
    }
}

fn print(t: Type) -> Type {
    println!("{:?}", t);
    t
}

fn add(t: Type, a: Type) -> Type{
	t+a
}

#[test]
fn main2() {
    let f = MyFunc::new(print);
    let g = f.clone();

    let x = Type::Int(32);
    let y = f.call(x.clone());
    let z = g.call(x.clone());
}