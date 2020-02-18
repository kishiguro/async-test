#![feature(async_closure)]

use futures::executor::block_on;
use futures::join;
use std::{thread, time};

static WAIT: bool = false;

async fn async_func() -> u32 {
    if WAIT {
        thread::sleep(time::Duration::from_secs(1));
    }
    1
}

// To call join!, it must be in async.
async fn futures() {
    let a = async_func();
    let b = async { 2 };
    let c = (async || 3)();

    let result = join!(a, b, c);
    println!("{} {} {}", result.0, result.1, result.2);
    assert_eq!(result, (1, 2, 3));
}

fn main() {
    let future = futures();
    block_on(future);
}
