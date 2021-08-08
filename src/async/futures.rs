//! Futures for the async singleton.

use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

/// Future returned by `AsyncSingleton::use_singleton`.
pub struct UseSingletonFuture<Func, Fut, Inner, Ret> {
    func: Box<Func>,
    state: UseSingletonFutureState,
    _phantom: PhantomData<(Fut, Inner, Ret)>,
}

/// State for the future returned by `AsyncSingleton::use_singleton`.
enum UseSingletonFutureState {
    AcquireLock,
    CallFunction,
}

impl<Func, Fut, Inner, Ret> Future for UseSingletonFuture<Func, Fut, Inner, Ret>
where
    Func: FnOnce(&Inner) -> Fut,
    Fut: Future<Output = Ret>,
{
    type Output = Ret;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            match self.state {
                UseSingletonFutureState::AcquireLock => todo!(),
                UseSingletonFutureState::CallFunction => {
                    let func = &self.func;
                    let fut = func(&F);
                    match fut.poll(cx) {
                        Poll::Pending => break Poll::Pending,
                        Poll::Ready(ret) => break Poll::Ready(ret),
                    }
                },
            }
        }
    }
}

fn abc<Func, Fut, Ret>(func: Func) -> Fut
where
    Func: FnOnce() -> Fut,
    Fut: Future<Output = Ret>
{
    func()
}

async fn test() -> i32 {
    abc(|| async { 42 }).await
}
