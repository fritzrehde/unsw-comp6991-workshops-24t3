// TODO:
// async rust
// concurrency (dealing with multiple things at once) vs parallelism (doing multiple things at once)
// send, sync traits (compare to how Copy is a building block every uses/depends on given simple rules): !send,!sync=Rc, send,!sync=Cell, !send,sync=MutexGuard
// thread::spawn vs thread::scope
// demo how two &mut doesn't compile
// Why do we need Arc (why not just Rc)?
// sharing state across threads: Arc<Mutex> (try Rc mutex, explain why doesn't work), rwlock
// common mistakes with mutex:
// 1. always need to unlock, otherwise deadlock (rust fix: mutex gets released/unlocked automatically at end of scope)
// 2. use mutex, but don't actually lock and unlock, so race condition still exists (rust fix: you don't even get mutable access without the mutex)
// channels: do not communicate by sharing memory, share memory by communicating (shared memory synchronisation vs message passing)
// demo we can still deadlock
// global mutable state: LazyLock

use std::{
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    println!("Hello, world!");

    // let rc = Rc::new(10);
    // let rc2 = Rc::clone(&rc);
    // // Rc::clone(&self)
    // thread::spawn(move || {
    //     loop {
    //         Rc::clone(&rc);
    //     }
    // });
    // thread::spawn(move || {
    //     loop {
    //         Rc::clone(&rc2);
    //     }
    // });

    // foo();

    let v = vec!["1", "20", "hello"];
    let test = v.iter().map(|s| s.parse::<u32>());
    let test = v.iter().map(|s| s.parse::<u32>()).filter(|opt| opt.is_ok());
    let collected: Vec<u32> = v
        .iter()
        .map(|s| s.parse::<u32>())
        .filter(|opt| opt.is_ok())
        .map(|res| res.expect("safe"))
        .collect();
    let test = v.iter().filter_map(|s| s.parse::<u32>().ok());

    for (i, element) in v.iter().enumerate() {
        //
    }

    let opt = Some(10);
    match opt {
        Some(10 | 20) => println!("fritz was right"),
        Some(v @ (10 | 20)) => println!("mitchell was right"),
        Some(v @ 10) | Some(v @ 20) => todo!(),
        Some(v) => {
            // i can use variable v here.
        }
        None => todo!(),
    };
    // map, filter, filter_map, find
}

fn generic_fn<T>(t: Vec<T>) {
    // what can I do with t now?
    for i in t {
        //
    }
}

enum Number {
    f64(f64),
    u32(u32),
}

trait NumberTrait {
    type Err;
    fn from_str(s: &str) -> Result<u32, Self::Err> {
        todo!()
    }
}
// impl NumberTrait for f64 {}
// impl NumberTrait for u32 {}

impl Number {
    fn fn_on_number(&self) {
        //
    }
}

fn foo2() {
    let num = Number::u32(10);
    num.fn_on_number();

    let v = vec![10, 10.0];
    let v = vec![Number::u32(10), Number::f64(10.0)];

    let v = vec![Number::u32(10), Number::f64(10.0)];
    generic_fn
}

// enum Option {
//     Some(i),
//     None,
// }

// enum Result {
//     Ok(i),
//     Err(e),
// }

// fn unwrap(opt: Option) -> i32 {
//     match opt {
//         Option::Some(i) => i,
//         Option::None => panic!("found none"),
//     }
// }

fn foo() {
    let rc = Arc::new(String::new());
    let rc2 = Arc::clone(&rc);
    let h1 = thread::spawn(move || loop {
        dbg!(&rc);
    });
    let h2 = thread::spawn(move || loop {
        dbg!(&rc2);
    });
    let res = h1.join().unwrap();
    let res = h2.join().unwrap();
}

fn foo2() {
    let rc = Arc::new(String::new());
    let rc2 = Arc::clone(&rc);
    thread::scope(|scope| {
        scope.spawn(move || loop {
            dbg!(&rc);
        });
        scope.spawn(move || loop {
            dbg!(&rc2);
        });
        // auto joins here
    });
}

fn foo3() {
    let rc = Arc::new(Mutex::new(String::new()));
    let rc2 = Arc::clone(&rc);
    thread::scope(|scope| {
        scope.spawn(move || loop {
            let mut s_mutex = rc.lock().expect("i don't care about poisoning");
            s_mutex.push_str("hello");
        });
        scope.spawn(move || loop {
            let mut s_mutex = rc2.lock().unwrap();
            s_mutex.push_str("hello");
        });
        // auto joins here
    });
}
