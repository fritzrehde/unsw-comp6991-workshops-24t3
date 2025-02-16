// & vs &mut, ownership, lifetimes

// TODO:
// dangling reference/pointer
// lifetimes

struct Struct {
    ref_: i32,
}

// declare methods that act on structs
impl Struct {
    fn dbg(&self) {
        dbg!(self.ref_);
    }
}

// struct Wrapper<'a> {
//     struct_: Struct<'a>,
// }
fn print_str(s: &str) {
    s.len();
    println!("{}", s);
}

fn main() {
    let s = String::from("hello");

    // print_str(ptr + 3, end:8);
    // print_str(&s[3..99]);

    let s = Struct { ref_: 10 };
    Struct::dbg(&s);
    s.dbg();
    s.dbg();
    // //
    // let i_ref = dangle();

    // // use after free
    // *i_ref;

    let x = 1000; // x start
    let z = {
        // z start
        let mut y = 2; // y start
        max_of_refs(&x, &mut y) // y end
    };
    // z end
    dbg!(z);
    dbg!(x); // x end
}

// // lifetime: how long reference is valid for
// fn dangle<'foo>() -> &'foo i32 {
//     // i gets deallocated/freed/dropped at the end of its scope
//     let i = 5;
//     &i
// } // i gets dropped, i's lifetime ends here

fn max_of_refs<'a, 'b>(a: &'a i32, b: &mut i32) -> &'a i32 {
    // *b += 1;
    a
    // if *a > *b {
    //     a
    // } else {
    //     b
    // }
}

// fn foo() {
//     let a = 10;
//     bar(&a);
// }

// fn bar(a: &i32) -> &i32 {
//     let b = 20;
//     let ret = max_of_refs(a, &b);
//     ret
// }
