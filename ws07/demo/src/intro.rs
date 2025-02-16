// TODO:
// diff between function and closure
// unfortunately, e.g. FnOnce is all or nothing, can't make individual captured vars e.g. copy or &mut like in cpp
// closure is unique type that can't be written down
// explain move keyword
// recursive macro

fn square(x: &i32) -> i32 {
    x * x
}

fn main() {
    let v = vec![1, 2, 3];

    let mut sum = "".to_string();

    // let want_ownership = move || {
    //     println!("{}", sum);
    // };

    let square_imm = |x: &i32| -> i32 { x * x };
    let square_mut = |x: &i32| -> i32 {
        sum += &x.to_string();
        x * x
    };
    let square_once = |x: &i32| -> i32 {
        drop(sum);
        x * x
    };

    // If map requires FnMut, we can pass FnMut and Fn, but not FnOnce.
    let foo = v.iter().map(square).collect::<Vec<_>>();

    map_fn_ptr(v, square);
    // pass fn() to Fn => fn() impl Fn, but not FnMut and FnOnce
    map_fn_ptr(v, square_imm);
}

// If we annotate with FnOnce, user can pass FnOnce, FnMut, and Fn.
// => for fn designer, FnOnce is most restrictive, for fn user FnOnce is most flexible (you can pass anything).
fn map<I, F>(iterable: I, mut f: F) -> Vec<i32>
where
    I: IntoIterator<Item = i32>,
    F: FnMut(i32) -> i32,
{
    let mut output = vec![];
    for i in iterable.into_iter() {
        output.push(f(i));
    }
    output
}

// fn = function pointer: can't capture vars from env
fn map_fn_ptr<I>(iterable: I, f: impl Fn(&i32) -> i32) -> Vec<i32>
where
    I: IntoIterator<Item = i32>,
{
    let mut output = vec![];
    for i in iterable.into_iter() {
        output.push(f(&i));
    }
    output
}
