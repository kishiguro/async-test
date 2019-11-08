use tokio::runtime::Runtime;
use std::time::Duration;
//use futures::join;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    println!("- start({:?})", std::thread::current().id());

    for i in 0..40 {
        rt.spawn(async move {

            println!("start({:?}) {}", std::thread::current().id(), i);
            // std::thread::sleep(Duration::from_secs(10));

            let when = tokio::clock::now() + Duration::from_secs(i);
            tokio::timer::delay(when).await;

            println!("end({:?}) {}", std::thread::current().id(), i);

            // producer message to channel.
        });
    }
    // rt.spawn(async {
    //     let delay_queue: tokio::timer::DelayQueue<String> = tokio::timer::DelayQueue::new();
    //     delay_queue.poll_next(rt);
    // });
    println!("---- non blocked routine ----");

    rt.shutdown_on_idle();
    println!("- end({:?})", std::thread::current().id());
    Ok(())
}
