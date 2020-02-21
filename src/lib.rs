use std::pin::Pin;
use std::task::{Context, Poll};
use std::future::Future;

fn run_async_processor() -> impl Send {
    async move {
        let message_stream = None::<MessageStream>.unwrap();
        let mapped_stream = Map::new(message_stream, |_| {
            async { }
        });
        Buffered::new(mapped_stream)
            .await;
    }
}

struct MessageStream<'a> {
    consumer: &'a (),
}

impl<'a> Stream for MessageStream<'a> {
    type Item = &'a ();
}

struct Buffered<St>
    where
        St: Stream,
        St::Item: Future,
{
    stream: St,
    in_progress_queue:St::Item,
}

impl<St> Buffered<St> {
    fn new(stream: St) -> Buffered<St> {
        loop {}
    }
}

impl<St> Stream for Buffered<St>
    where
        St: Stream,
        St::Item: Future,
{
    type Item = <St::Item as Future>::Output;
}

impl<St> Future for Buffered<St> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {}
    }
}

struct Map<St, F> {
    stream: St,
    f: F,
}

impl<St, T, F> Map<St, F>
    where St: Stream,
          F: FnMut(St::Item) -> T,
{
    fn new(_: St, _: F) -> Map<St, F> {
        loop {}
    }

}

impl<St, F, T> Stream for Map<St, F>
    where St: Stream,
          F: FnMut(St::Item) -> T,
{
    type Item = T;
}

trait Stream {
    type Item;
}
