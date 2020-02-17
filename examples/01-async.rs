use futures::executor::block_on;

async fn async_func() -> i32 {
    1
}

fn main() {
    let future = async_func();
    let ret = block_on(future);

    println!("{}", ret);
}
