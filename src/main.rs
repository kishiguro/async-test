use std::sync::Arc;
use std::sync::Mutex;

use futures::future::join_all;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    println!("- start({:?})", std::thread::current().id());

    let counter = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    for i in 0..4 {
        let counter = Arc::clone(&counter);
        handles.push(tokio::spawn(async move {
            println!("start({:?}) {}", std::thread::current().id(), i);
            tokio::time::delay_for(Duration::from_secs(i)).await;
            println!("end({:?}) {}", std::thread::current().id(), i);

            let mut num = counter.lock().unwrap();
            if i == 3 {
                (*num).pop();
            } else {
                (*num).push(0);
            }
        }));
    }
    println!("---- non blocked routine ----");

    join_all(handles).await;

    let mut num = counter.lock().unwrap();
    (*num).push(199);
    println!("{:?}", (*num));

    println!("- end({:?})", std::thread::current().id());

    Ok(())
}
