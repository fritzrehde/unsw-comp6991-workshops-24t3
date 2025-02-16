// todo:
// impl Trait for Type {}
// T vs impl Trait syntax (downside: multiple occurences don't have to be same type, doesn't work in type definitions)
// fn<T: Trait>(t: T) or fn<T>(t: T) where T: Trait (bound)
// trait Trait: A + B (any type implementig Trait must also implement A and B)
// monomorphization (static dispatch)
// trait objects
// associated vs generic type parameter (show Add example)
// default implementations

mod demo;

use std::{array::IntoIter, error::Error, ops::Add};

trait Trait: PartialEq + Eq + Sized {
    fn methods(&self) {
        // access to methods from PartialEq traits...
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}
