use futures::future::join_all;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    println!("- start({:?})", std::thread::current().id());

    let mut handles = Vec::new();

    for i in 0..40 {
        handles.push(tokio::spawn(async move {
            println!("start({:?}) {}", std::thread::current().id(), i);
            tokio::time::delay_for(Duration::from_secs(i)).await;
            println!("end({:?}) {}", std::thread::current().id(), i);
        }));
    }
    println!("---- non blocked routine ----");

    join_all(handles).await;

    println!("- end({:?})", std::thread::current().id());

    Ok(())
}
