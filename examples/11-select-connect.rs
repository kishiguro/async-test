use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::stream::{Stream, StreamExt};
use tokio::sync::mpsc;
use tokio::time::{DelayQueue, Duration};

#[derive(Debug)]
enum Event {
    //Connected,
    TimerExpired,
    //ChannelReceived,
}

#[derive(Debug)]
struct TimerStream(DelayQueue<Event>);

impl TimerStream {
    pub fn new() -> Self {
        TimerStream(DelayQueue::new())
    }
    pub fn insert(&mut self, value: Event, timeout: Duration) -> tokio::time::delay_queue::Key {
        self.0.insert(value, timeout)
    }
}

impl Stream for TimerStream {
    type Item = Result<Event, std::io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let Poll::Ready(Some(Ok(v))) = Pin::new(&mut self.0).poll_expired(cx) {
            return Poll::Ready(Some(Ok(v.into_inner())));
        }
        Poll::Pending
    }
}

use Event::*;

async fn connect(addr: std::net::SocketAddr, mut rx: mpsc::UnboundedReceiver<Event>) {
    let sock = loop {
        let mut timer = TimerStream::new();
        timer.insert(TimerExpired, Duration::from_secs(3));

        tokio::select! {
            Some(Ok(TimerExpired)) = timer.next() => {
                println!("Start timer expired");
            },
            Some(_) = rx.next() => {
                println!("RX");
            },
        };

        tokio::select! {
            _ = timer.next() => {
                println!("XXX timer should not happen");
                continue;
            },
            v = tokio::net::TcpStream::connect(addr) => {
                match v {
                    Ok(v) => {
                        break v;
                    }
                    Err(e) => {
                        println!("Connect Error {:?}", e);
                        continue;
                    }
                }
            }
            _ = rx.next() => {
                println!("RX");
                continue;
            },
        }
    };
    // SockStream.
    println!("sock {:?}", sock);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (_tx, rx) = mpsc::unbounded_channel::<Event>();

    let ip = std::net::IpAddr::V4("192.168.55.2".parse().unwrap());
    let addr = std::net::SocketAddr::new(ip, 179);

    tokio::spawn(connect(addr, rx));

    loop {}
}
