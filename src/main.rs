use std::time::Duration;
use tokio::runtime::Runtime;
//use futures::join;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    println!("- start({:?})", std::thread::current().id());

    for i in 0..40 {
        rt.spawn(async move {
            println!("start({:?}) {}", std::thread::current().id(), i);
            tokio::time::delay_for(Duration::from_secs(i)).await;
            println!("end({:?}) {}", std::thread::current().id(), i);
        });
    }
    println!("---- non blocked routine ----");

    //rt.shutdown_on_idle();
    println!("- end({:?})", std::thread::current().id());

    loop {}
    Ok(())
}
