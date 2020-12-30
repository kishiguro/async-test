//
// async example 02.
//
// calling async func from another async func.
//
use futures::executor::block_on;

async fn async_func() -> i32 {
    1
}

async fn async_await() -> i32 {
    async_func().await
}

fn main() {
    let future = async_await();
    let ret = block_on(future);

    println!("{}", ret);
}
