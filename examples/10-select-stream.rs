use tokio::stream::{self, StreamExt};

#[tokio::main]
async fn main() {
    let mut stream1 = stream::iter(vec![1, 2, 3]);
    let mut stream2 = stream::iter(vec![4, 5, 6]);

    let next = tokio::select! {
        v = stream1.next() => {
            println!("stream1");
            v.unwrap()
        },
        v = stream2.next() => {
            println!("stream2");
            v.unwrap()
        },
    };

    println!("next {}", next);

    assert!(next == 1 || next == 4);
}
