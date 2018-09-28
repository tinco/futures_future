#![feature(pin)]
#![feature(arbitrary_self_types)]
#![feature(futures_api)]

use futures::*;
use std::pin::Pin;
use std::task;

// Struct that has a reference to an old style future and implements the new style future
pub struct FuturesFuture<'a, F: 'a> {
    future: &'a mut F
}

impl<'a, F: Future<Item=T, Error=E>, T, E> std::future::Future for FuturesFuture<'a, F> {
    type Output = Result<T, E>;

    #[inline]
    fn poll(self: Pin<&mut Self>, _lw: &task::LocalWaker) -> task::Poll<Result<T,E>> {
        // TODO we should awake the localwaker if we're not ready.
        match Pin::get_mut(self).future.poll() {
            Ok(result) => match result {
                Async::Ready(item) => task::Poll::Ready(Ok(item)),
                Async::NotReady   => task::Poll::Pending,
            },
            Err(err) => task::Poll::Ready(Err(err)),
        }
    }
}

// Convert a mut ref to an old style future into a new style future
pub fn futures_future<F: Future>(future: &mut F) -> FuturesFuture<F> {
    FuturesFuture { future }
}
