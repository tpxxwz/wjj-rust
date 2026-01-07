// Streams 就是 std::iter::Iterator 的异步版本

use crate::tokio::_07_async_in_depth::Delay;
use async_stream::stream;
use futures::{Stream, stream};
use mini_redis::client;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use tokio_stream::StreamExt;
// ❌ 这是不支持的语法
// async fn run(mut stream: impl Stream<Item = i32>) {
//     async for item in stream {
//         println!("{}", item);
//     }
// }

// Stream 很像Future
// pub trait Stream {
//     type Item;
//
//     fn poll_next(
//         self: Pin<&mut Self>,
//         cx: &mut Context<'_>
//     ) -> Poll<Option<Self::Item>>;
//
//     fn size_hint(&self) -> (usize, Option<usize>) {
//         (0, None)
//     }
// }

#[tokio::test]
async fn stream_iter() {
    // let mut stream = stream::iter(&[1, 2, 3]);
    let mut stream = tokio_stream::iter(&[1, 2, 3]);

    while let Some(v) = stream.next().await {
        println!("GOT = {:?}", v);
    }
}

async fn publish() -> mini_redis::Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Publish some data
    client.publish("numbers", "1".into()).await?;
    client.publish("numbers", "two".into()).await?;
    client.publish("numbers", "3".into()).await?;
    client.publish("numbers", "four".into()).await?;
    client.publish("numbers", "five".into()).await?;
    client.publish("numbers", "6".into()).await?;
    Ok(())
}

async fn subscribe() -> mini_redis::Result<()> {
    let client = client::connect("127.0.0.1:6379").await?;
    let subscriber = client.subscribe(vec!["numbers".to_string()]).await?;

    // let messages = subscriber.into_stream();

    // Adapters

    // let messages = subscriber
    //     .into_stream()
    //     .take(3);

    // let messages = subscriber
    //     .into_stream()
    //     .filter(|msg| match msg {
    //         Ok(msg) if msg.content.len() == 1 => true,
    //         _ => false,
    //     })
    //     .take(3);

    let messages = subscriber
        .into_stream()
        .filter(|msg| match msg {
            Ok(msg) if msg.content.len() == 1 => true,
            _ => false,
        })
        .map(|msg| msg.unwrap().content)
        .take(3);

    tokio::pin!(messages);

    while let Some(msg) = messages.next().await {
        println!("got = {:?}", msg);
    }

    Ok(())
}

#[tokio::test]
async fn mini_redis_client_server() -> mini_redis::Result<()> {
    tokio::spawn(async { publish().await });

    subscribe().await?;

    println!("DONE");

    Ok(())
}

struct Interval {
    rem: usize,
    delay: Delay,
}

impl Interval {
    fn new() -> Self {
        Self {
            rem: 3,
            delay: Delay {
                when: Instant::now(),
            },
        }
    }
}

impl Stream for Interval {
    type Item = ();

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<()>> {
        if self.rem == 0 {
            // No more delays
            return Poll::Ready(None);
        }

        match Pin::new(&mut self.delay).poll(cx) {
            Poll::Ready(_) => {
                let when = self.delay.when + Duration::from_millis(10);
                self.delay = Delay { when };
                self.rem -= 1;
                Poll::Ready(Some(()))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

// The async-stream crate is available as a temporary solution.
// This crate provides a stream! macro that transforms the input into a stream
async fn async_stream() -> impl Stream<Item = ()> {
    stream! {
        let mut when = Instant::now();
        for _ in 0..3 {
            let delay = Delay { when };
            delay.await;
            yield ();
            when += Duration::from_millis(10);
        }
    }
}
