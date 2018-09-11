# Futures Future

The Rust crate that converts the old style futures crate `futures::Future` into the new
nightly async/await style `std::future::Future` so you can easily try out the new
syntax.

## Example

```
#![feature(async_await)]
#![feature(futures_api)]
#![feature(await_macro)]

use futures::*;
use futures::sync::oneshot;
use futures_future::*;

pub async fn and_its_done() {
    let (signal_setup_done, mut setup_done) = oneshot::channel::<bool>();
    let _ = signal_setup_done.send(true);
    let f = futures_future(&mut setup_done);
    await!(f);
}

```
