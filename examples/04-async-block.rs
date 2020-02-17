//
// async example 04.
//
// async block.
//
use futures::executor::block_on;

fn main() {
    let future = async { 1 };
    let ret = block_on(future);

    println!("{}", ret);
}
