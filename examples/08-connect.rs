use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::stream::{Stream, StreamExt};
use tokio::sync::mpsc;
use tokio::time::{DelayQueue, Duration};

#[derive(Debug)]
enum Event {
    Connected,
    TimerExpired,
    ChannelReceived,
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

#[derive(Debug)]
struct TcpConnectStream {
    pub io: tokio::io::PollEvented<mio::net::TcpStream>,
}

impl TcpConnectStream {
    pub fn connect(addr: std::net::SocketAddr) -> std::io::Result<TcpConnectStream> {
        let sys = mio::net::TcpStream::connect(&addr)?;
        let stream = TcpConnectStream::new(sys)?;
        Ok(stream)
    }

    fn new(connected: mio::net::TcpStream) -> std::io::Result<TcpConnectStream> {
        let io = tokio::io::PollEvented::new(connected)?;
        Ok(TcpConnectStream { io })
    }
}

impl Stream for TcpConnectStream {
    type Item = Result<Event, std::io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.io).poll_write_ready(cx) {
            Poll::Ready(Ok(_)) => {
                if let Some(e) = self.io.get_ref().take_error()? {
                    Poll::Ready(Some(Err(e)))
                } else {
                    Poll::Ready(Some(Ok(Event::Connected)))
                }
            }
            Poll::Ready(Err(e)) => Poll::Ready(Some(Err(e))),
            Poll::Pending => Poll::Pending,
        }
    }
}

// struct ReceiverStream {
//     rx: &mpsc::UnboundedReceiver<Event>,
// }

// impl Stream for ReceiverStream {
//     type Item = Result<Event, std::io::Error>;

//     fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
//         if let Poll::Ready(Some(_)) = Pin::new(&mut self.rx).poll_next(cx) {
//             Poll::Ready(Some(Ok(Event::ChannelReceived)))
//         } else {
//             Poll::Pending
//         }
//     }
// }

async fn connect(addr: std::net::SocketAddr, mut rx: mpsc::UnboundedReceiver<Event>) {
    // match rx.next().await {
    //     Some(_) => {}
    //     None => {}
    // }

    let val = loop {
        let mut timer = TimerStream::new();
        timer.insert(Event::TimerExpired, Duration::from_secs(3));

        //let mut timer = timer.merge(rx);

        // if let Some(Ok(Event::TimerExpired)) = timer.next().await {
        //     println!("Start timer expired");
        // } else {
        //     continue;
        // }

        // TCP connect stream.
        let connect = TcpConnectStream::connect(addr).unwrap();

        // Merge timer and connect stream for multiplexing.
        let mut stream = timer.merge(connect);
        if let Some(v) = stream.next().await {
            match v {
                Ok(_) => {
                    println!("Connect success!");
                    break 1;
                }
                Err(e) => {
                    println!("Connect Error {} Rretry!", e);
                }
            }
        }
    };
    println!("loop break {}", val);

    // Collision detect.

    // Create timer.
    // Create rx.
    // Create framed.

    // Connect success.  Start parsing packet.
    // loop {
    //     //let timer = tokio::stream::pending::<Result<Event, std::io::Error>>();
    //     println!("Start stream");
    //     let mut stream = TimerStream::new();
    //     stream.0.insert(Event::TimerExpired, Duration::from_secs(1));

    //     //let stream = stream.merge(rx);

    //     // let stream = TcpConnectStream::connect(addr)?;

    //     //let mut stream = timer.merge(stream);

    //     if let Some(v) = stream.next().await {
    //         println!("Get Some from stream");
    //         match v {
    //             Ok(Event::TimerExpired) => {
    //                 println!("Start Timer expired");
    //                 // Need to add.
    //                 continue;
    //             }
    //             Ok(Event::Connected) => {
    //                 println!("Connect success");
    //                 break;
    //             }
    //             Err(e) => {
    //                 println!("XXX Err {}", e);
    //                 continue;
    //             }
    //         }
    //     }

    //     println!("before sleep");
    //     std::thread::sleep(std::time::Duration::from_secs(5));
    //     println!("after sleep");
    // }
    // println!("XXX done");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (_tx, rx) = mpsc::unbounded_channel::<Event>();

    let ip = std::net::IpAddr::V4("192.168.55.2".parse().unwrap());
    let addr = std::net::SocketAddr::new(ip, 179);

    tokio::spawn(connect(addr, rx));

    loop {}
}
