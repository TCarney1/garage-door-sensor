#![no_std]

mod door;
pub use door::DoorState;

#[allow(async_fn_in_trait)]
pub trait Source {
    type Event;
    fn current(&mut self) -> Self::Event;
    async fn changed(&mut self) -> Self::Event;
}

#[allow(async_fn_in_trait)]
pub trait Notifier<E> {
    async fn notify(&mut self, event: E);
}

pub async fn forward<S, N>(mut source: S, mut sink: N)
where
    S: Source,
    N: Notifier<S::Event>,
{
    sink.notify(source.current()).await;
    loop {
        let event = source.changed().await;
        sink.notify(event).await;
    }
}