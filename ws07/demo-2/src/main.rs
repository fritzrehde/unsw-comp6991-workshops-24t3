// TODO:
// diff between function and closure
// unfortunately, e.g. FnOnce is all or nothing, can't make individual captured vars e.g. copy or &mut like in cpp
// closure is unique type that can't be written down
// explain move keyword
// recursive macro

// Fn, FnMut, FnOnce

use std::thread;

// impl Fn, FnMut, FnOnce
fn square(x: &i32) -> i32 {
    x * x
}

fn main() {
    let v = vec![1, 2, 3];

    exec_fn(square);
    exec_fn(square);

    // env variable (not shell)
    let mut s = String::new();

    // impl Fn, impl FnMut, impl FnOnce
    let square1 = |x: &i32| -> i32 { x * x };
    let s_ref = &s;
    let square1_5 = move |x: &i32| -> i32 {
        // most restricted: we can't mutable borrow env ref, can't take ownership.
        println!("{}", s_ref);
        x * x
    };
    dbg!(s_ref);
    exec_fn(square1);
    exec_fn2(square1);

    // impl FnOnce
    let square3 = |x: &i32| -> i32 {
        drop(s);
        x * x
    };

    // exec_fn(square3);
    // exec_fn2(square3);

    // impl FnMut, impl FnOnce
    let square2 = |x: &i32| {
        s.push_str("hello");
        x * x
    };
    // exec_fn(square2);
    // exec_fn2(square2);

    let _ = v.iter().map(square).collect::<Vec<_>>();
    // closure: anonymous function
    let _ = v.iter().map(|x| x * x).collect::<Vec<_>>();
}

// perspectives:
// - function/library author: which Fn... do I use as param: best (least restrictive): Fn, worst (most restrictive): FnOnce
// - function/library user: which Fn... can I pass to fn as param: best: FnOnce, worst: Fn

fn exec_fn<F>(f: F)
where
    F: FnOnce(&i32) -> i32,
{
    f(&10);
    // f(&10);
}

fn exec_fn2(f: fn(&i32) -> i32) {
    f(&10);
    f(&10);
}

fn my_map<F>(v: Vec<i32>, mut f: F) -> Vec<i32>
where
    F: FnMut(i32) -> i32,
{
    let mut o = vec![];
    for i in v {
        o.push(f(i));
    }
    o
}
