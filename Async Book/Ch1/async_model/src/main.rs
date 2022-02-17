/*
Asynchronous programming (Async from now on)
one of concurrent programming model
can run many concurrent tasks on few OS threads
allows highly performant implementations

Rust's async
    Futures execute only when polled.
    Dropping futures stops its execution
    Async is zero-cost: only pay for what is used
    No built-in runtime
    Both single- and multithreaded runtimes are available



*/

#![allow(unused)]

use std::thread;
use futures;

fn main() {
    println!("Hello, world!");
}

// thread - 2 threads are created
fn get_two_sites() {
    let thread_one = thread::spawn(|| download("http://www.foo.com"));
    let thread_two = thread::spawn(|| download("http://www.bar.com"));

    thread_one.join().expect("thread one panicked");
    thread_two.join().expect("thread two panicked");
}

// async - no threads are created, no heap allocations
async fn get_two_sites_async() {
    let future_one = download_async("http://www.foo.com");
    let future_two = download_async("http://www.bar.com");

    //futures::join!(future_one, future_two);
}

fn download(s: &str) {}
fn download_async(s: &str) {}
