// - stack vs heap
// ownership rules:
// - Each value in Rust has **one** owner.
// - There can only be one owner at a time.
// - When the owner goes out of scope, the value will be dropped.
// use after free, double free, dangling pointers, thread safety

// &str vs String
// moving
// Copy (stack-only data) vs Clone (show demo with Copy vs Clone use-after-move) => .clone()
// What is the default when copying? (deep vs shallow)
// demo function taking and returning ownership

// Show fancy iterator stuff (map, filter, collect, fold).
// entry API: python's defaultdict in rust (but on steroids)

// Copy vs Clone (deep copy)

use std::{collections::HashMap, hash, vec};

#[derive(Debug, Clone, Copy)]
struct Test {
    arr: [i32; 100000000],
    a: i32,
}

// fn main() {
//     let b = {
//         let a = 10;
//         a
//     };

//     let a: &str = "hello world";

//     let s = String::from("hello");

//     let s_ref: &str = &s;

//     let x = 1;
//     foo(x);
//     dbg!(x);

//     // dbg!(a);
//     let mut v = vec![1, 2];

//     // let v_ref = &mut v;
//     mutates_vec(&mut v);
//     dbg!(v);
// }

fn foo(mut x: i32) {
    x += 1;
    dbg!(x);
}

fn print_len_vector(v: &Vec<i32>) {
    println!("{}", v.len());
}

// & (immutable, shared)
// &mut (mutable, exclusive)

fn mutates_vec(v: &mut Vec<i32>) {
    v.push(10);
    // v
}

fn main() {
    // iter() vs iter_mut() vs into_iter()
    let mut v = vec![1, 2, 3];
    // dbg!(v);

    // let arr = [1, 2, 3];

    dbg!(&v);

    let v2: Vec<_> = dbg!(v.iter().map(|x| x * 2).filter(|x| *x % 2 == 0).collect());
    // v.iter_mut().filter(|x| **x % 2 == 0).for_each(|x| *x *= 2);
    // let v3: Vec<_> = v
    //     .into_iter()
    //     .map(|x| x * 2)
    //     .filter(|x| x % 2 == 0)
    //     .collect();
    // dbg!(v);

    // v.into_iter();
}
