use futures::executor::block_on;
use futures::join;

fn main() {
    let future = async {
        let a = async { 1 };
        let b = async { 2 };

        assert_eq!(join!(a, b), (1, 2));
        println!("future is called");
    };

    block_on(future);
}
