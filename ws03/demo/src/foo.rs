// take whichever lifetime is shorter (in our case that is the lifetime of b).
fn max_of_refs(a: &i32, b: &i32) -> &i32 {
    if *a > *b {
        a
    } else {
        b
    }
}

fn foo() {
    let a = 10;
    bar(&a);
}

fn bar(a: &i32) -> &i32 {
    let b = 20;
    // ret's lifetime is now equal to the shorter of a and b's lifetimes, so b's.
    let ret = max_of_refs(a, &b);
    ret
}
