//
// async example 05.
//
// async closure.  This require nightly rust.
//
// $ rustup default nightly
//
#![feature(async_closure)]

use futures::executor::block_on;

fn main() {
    let future = (async || 1)();
    let ret = block_on(future);

    println!("{}", ret);
}
