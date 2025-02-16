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
    arr: [i32; 100000],
    a: i32,
}

// impl Copy for Test {}
fn count(s: &str) -> HashMap<char, i32> {
    let mut count = HashMap::new();
    // for
    count
}

fn main() {
    let mut hashmap: HashMap<&str, i32> = HashMap::new();
    hashmap.insert("test", 10);
    hashmap.insert("another", 20);

    // let n = 4;
    // let mut array = [1, 2, 3];
    // let array = [0; 1000];
    // array.push(5);

    // let mut test = Test { a: 10 };
    // let mut test2 = test;
    // test.a = 15;
    // test2.a = 16;
    // println!("{}, {}", test.a, test2.a);

    // dbg!(test);

    let mut vector;
    {
        vector = vec![1, 2, 3];
    }

    let sum_: i32 = vector
        .iter()
        .map(|i| i * 2)
        .filter(|i| i % 2 == 0)
        .max()
        .unwrap();

    // vector.push(5);
    // dbg!(vector);

    // for e in vector.iter() {
    //     dbg!(e);
    //     vector.push(5);
    // }

    let vector_ref = &mut vector; // vector_ref lifetime starts here
    println!("{}", vector.len()); // <unnamed ref> lifetime starts and ends here.
    push_element_to_vec(vector_ref); // vector_ref lifetime ends here

    // std::env::args();
    // println!("Hello, world!");
}

fn push_element_to_vec(v: &Vec<i32>) {
    // I need &mut
    // v.push(5)
    // End of scope: every owned value gets dropped.
}
