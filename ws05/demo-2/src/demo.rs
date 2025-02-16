// todo:
// impl Trait for Type {}
// default implementations
// impl Trait syntax (downside: multiple occurences don't have to be same type, doesn't work in type definitions)
// fn<T: Trait>(t: T) or fn<T>(t: T) where T: Trait (bound)
// trait Trait: A + B (any type implementig Trait must also implement A and B)
// monomorphization (static dispatch)
// trait objects
// associated vs generic type parameter (show Add example)

use std::{error::Error, ops::Add};

#[derive(Debug, PartialEq, Default, Copy, Clone)]
struct Int<T: std::fmt::Debug> {
    inner: T,
}

struct U32Wrapper {
    inner: u32,
}

impl From<U32Wrapper> for u32 {
    fn from(value: U32Wrapper) -> Self {
        todo!()
    }
}

// these work:
// impl MyTrait for u32 {}
// impl ForeignTrait for MyType {}

impl Add<u32> for U32Wrapper {
    // associated type
    type Output = u32;

    fn add(self, rhs: u32) -> Self::Output {
        todo!()
    }
}

// impl Add<i32> for U32Wrapper {
//     // associated type
//     type Output = u32;

//     fn add(self, rhs: u32) -> Self::Output {
//         todo!()
//     }
// }

// impl Add<u32> for Int {
//     fn add(self, rhs: u32) -> Self::Output {
//         todo!()
//     }
// }

// impl Int {
//     // implement member functions.
// }

// impl Trait for Int {
//     // implement specific member functions == functions of the trait.
// }

// impl Ord for Int {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         todo!()
//     }
// }

// impl PartialOrd for Int {

// }

// trait Add<Lhs, Rhs, Output> {

//     fn add(self, rhs: Rhs) -> Output;
// }

fn foo2<F: std::fmt::Debug, T: IntoIterator<Item = F>>(iterable: T, iterable2: T) {
    for i in iterable {
        dbg!(i);
    }
}

// impl Type {}
// impl Trait for Type {}

fn foo(iterable: impl IntoIterator<Item = u32>, iterable2: impl IntoIterator<Item = u32>) {
    for i in iterable {
        dbg!(i);
    }
}

fn iterable(v: Vec<u32>) -> Box<dyn Iterator<Item = u32>> {
    if (true) {
        Box::new(v.into_iter().rev())
    } else {
        Box::new(v.into_iter())
    }
}

// fn addable<T: Add<Output = T>>(a: T, b: T) -> T {
//     a + b
// }

fn foo3<T>(iterable: T)
where
    T: IntoIterator<Item = u32> + std::fmt::Debug,
{
    for i in iterable {
        dbg!(i);
    }
}

fn main() {
    let a: u32 = 10u8.into();
    let b = a + a;

    println!("Hello, world!");

    // let a: Type = c.add(b);

    // foo(vec![1, 2, 3], [1, 2, 3]);
    // foo2(vec![1, 2, 3], [1, 2, 3]);

    for i in iterable(vec![1, 2, 3]) {
        dbg!(i);
    }

    // let v = vec![1, 2, 3];
    let v = vec![1, 2, 3];

    let arr = [1, 2, 3];
    // foo(v, arr);
    // foo2(v, arr);
    // let i = Int { inner: 10 };
    // dbg!(i);

    // v.iter() == (&v).into_iter()
    // v.iter_mut() == (&mut v).into_iter()
    // v.into_iter()

    let cat = Cat {};
    let v: Vec<&impl Animal> = vec![&cat, &Dog {}];

    for animal in v {
        animal.speak();
    }

    //
    //
    //
    // let test = (&v).into_iter();
    // let test: Box<dyn Iterator<Item = &u32>> = Box::new(v.iter());

    // OOP
    // let v: Vec<Box<dyn Animal>> = vec![Box::new(Cat {}), Box::new(Dog {})];
    let v: Vec<&dyn Animal> = vec![&Cat {}, &Dog {}];
    for i in v {
        i.speak();
    }
}

// use anyhow::Result;

fn bar() -> Result<u32, Box<dyn Error>> {
    // fn bar() -> Result<u32, String> {
    let a = "120".parse()?;
    Ok(a)
}

trait Animal: std::fmt::Debug {
    fn speak(&self);
}

// polymorphism

#[derive(Debug)]
struct Cat;
#[derive(Debug)]
struct Dog;

impl Animal for Cat {
    fn speak(&self) {
        println!("i'm a cat");
    }
}

impl Animal for Dog {
    fn speak(&self) {
        println!("i'm a dog");
    }
}
