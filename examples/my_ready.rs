use std::{
    pin::Pin,
    task::{Context, Poll},
};

use anyhow::Result;
use futures::Future;

#[tokio::main]
async fn main() -> Result<()> {
    let mut cx = Context::from_waker(futures::task::noop_waker_ref());
    let ret = poll_fut(&mut cx);
    println!("{:?}", ret);

    let fut = MyFut::new(100);
    println!("{:?}", fut.await);
    Ok(())
}

fn poll_fut(cx: &mut Context<'_>) -> Poll<usize> {
    let mut fut = MyFut::new(42);
    let fut = Pin::new(&mut fut);
    my_ready!(fut.poll(cx))
}

struct MyFut {
    polled: bool,
    v: usize,
}

impl MyFut {
    pub fn new(v: usize) -> Self {
        Self { polled: false, v }
    }
}

impl Future for MyFut {
    type Output = usize;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.polled {
            Poll::Ready(self.v)
        } else {
            self.polled = true;
            // wake up the waker
            cx.waker().wake_by_ref(); // 一定要把waker唤醒，否则会一直Pending阻塞
            Poll::Pending
        }
    }
}

// my_ready! => Poll::Ready / Poll::Pending
#[macro_export]
macro_rules! my_ready {
    ($expr:expr) => {
        match $expr {
            std::task::Poll::Ready(v) => std::task::Poll::Ready(v),
            std::task::Poll::Pending => return std::task::Poll::Pending,
        }
    };
}
