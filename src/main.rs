use futures::future::join_all;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("- start({:?})", std::thread::current().id());

    let mut handles = Vec::new();
    for i in 0..40 {
        let h = tokio::spawn(async move {
            println!("start({:?}) {}", std::thread::current().id(), i);
            tokio::time::delay_for(Duration::from_secs(i)).await;
            println!("end({:?}) {}", std::thread::current().id(), i);
        });
        handles.push(h)
    }
    println!("---- non blocked routine ----");
    println!("- end({:?})", std::thread::current().id());

    join_all(handles).await;

    Ok(())
}
