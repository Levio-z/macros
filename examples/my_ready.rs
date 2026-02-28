use std::{
    pin::Pin,
    task::{Context, Poll},
};

use template::my_ready;

#[tokio::main]
async fn main() {
    let fut = MyFut::new(42);
    println!("Final result: {:?}", fut.await);
}

fn _poll_fut(cx: &mut Context<'_>) -> Poll<usize> {
    let mut fut = MyFut::new(42);
    let fut = Pin::new(&mut fut);
    let ret = my_ready!(fut.poll(cx));
    Poll::Ready(ret)
}

struct MyFut {
    polled: bool,
    value: usize,
}
impl MyFut {
    fn new(value: usize) -> Self {
        Self {
            polled: false,
            value,
        }
    }
}
impl Future for MyFut {
    type Output = usize;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.polled {
            println!("polling ready");
            Poll::Ready(self.value)
        } else {
            println!("polling pending");
            self.polled = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
