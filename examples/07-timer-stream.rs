//
// async example 07
//
// using DequeTime with stream.
//
use std::net::{IpAddr, SocketAddr};
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::net::TcpStream;
use tokio::stream::{Stream, StreamExt};
//use tokio::time::{DelayQueue, Duration};
use bytes::BytesMut;
//use std::io::Cursor;
use tokio::time::DelayQueue;
use tokio_util::codec::{Decoder, Encoder, Framed};

#[derive(Debug)]
pub enum Event {
    Expired(u32),
    Message,
}

struct Streams {
    timer: DelayQueue<u32>,
}

impl Stream for Streams {
    type Item = Result<Event, std::io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let Poll::Ready(Some(Ok(v))) = Pin::new(&mut self.timer).poll_expired(cx) {
            return Poll::Ready(Some(Ok(Event::Expired(v.into_inner()))));
        }
        Poll::Pending
    }
}

struct Bgp {}

pub fn from_bytes(_buf: &[u8]) -> Result<Event, std::io::Error> {
    // let _buflen = buf.len();
    // let mut _c = Cursor::new(_buf);

    return Ok(Event::Message);

    // if buflen < Message::HEADER_LENGTH as usize {
    //     return Err(format_err!("header is too short"));
    // }
    // c.set_position(16);
    // let length = c.read_u16::<NetworkEndian>()?;
    // if buflen < length as usize {
    //     return Err(format_err!("buffer is too short"));
    // }

    // let code = c.read_u8()?;
    // let mut c = Cursor::new(&buf[Message::HEADER_LENGTH as usize..length as usize]);
    // match code {
    //     Message::OPEN => {
    //         let b = OpenMessage::from_bytes(&mut c)?;
    //         return Ok(Message::Open(b));
    //     }
    //     Message::UPDATE => {
    //         let b = UpdateMessage::from_bytes(param, &mut c)?;
    //         return Ok(Message::Update(b));
    //     }
    //     Message::NOTIFICATION => {
    //         let b = NotificationMessage::from_bytes(&mut c)?;
    //         return Ok(Message::Notification(b));
    //     }
    //     Message::KEEPALIVE => return Ok(Message::Keepalive),
    //     Message::ROUTE_REFRESH => {
    //         let b = RouteRefreshMessage::from_bytes(&mut c)?;
    //         return Ok(Message::RouteRefresh(b));
    //     }
    //     _ => {
    //         let body_length = length - Message::HEADER_LENGTH;
    //         for _ in 0..body_length {
    //             c.read_u8()?;
    //         }
    //         return Ok(Message::Unknown {
    //             length: body_length as usize,
    //             code: code,
    //         });
    //     }
    // }
}

impl Decoder for Bgp {
    type Item = Event;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> std::io::Result<Option<Event>> {
        match from_bytes(src) {
            Ok(_) => Ok(Some(Event::Message)),
            Err(_) => Ok(None),
        }
    }
}

impl Encoder for Bgp {
    type Item = Event;
    type Error = std::io::Error;

    fn encode(&mut self, _item: Event, _dst: &mut BytesMut) -> Result<(), std::io::Error> {
        Ok(())
    }
}

struct Session {
    frames: Framed<TcpStream, Bgp>,
}

impl Stream for Session {
    type Item = Result<Event, std::io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let result: Option<_> = futures::ready!(Pin::new(&mut self.frames).poll_next(cx));
        Poll::Ready(match result {
            Some(Ok(_)) => Some(Ok(Event::Message)),

            Some(Err(e)) => Some(Err(e)),
            None => None,
        })
    }
}

#[tokio::main]
async fn main() {
    // let mut sess = Streams {
    //     timer: DelayQueue::<u32>::new(),
    // };

    // let key = sess.timer.insert(2u32, Duration::from_secs(2));
    // sess.timer.insert(1u32, Duration::from_secs(1));
    // sess.timer.remove(&key);

    // let one = tokio::stream::once(Ok(Event::Expired(100)));

    // let mut sess = one.merge(sess);

    let addr = IpAddr::V4("192.168.55.2".parse().unwrap());
    let sock = SocketAddr::new(addr, 179);
    let conn = TcpStream::connect(sock).await.unwrap();

    let mut sess = Session {
        frames: Framed::new(conn, Bgp {}),
    };

    loop {
        match sess.next().await {
            Some(_) => {
                println!("Some");
            }

            //match v {
            // Ok(Event::Expired(n)) => {
            //     println!("Event::Expired {}", n);
            // }
            // Err(_) => {}
            //},
            None => {
                println!("None");
                break;
            }
        }
    }
}
