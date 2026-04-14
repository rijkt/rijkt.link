+++
title = "tokio study notes"
date = 2025-08-08
[taxonomies]
tags = ["rust", "study notes"]
+++

Some notes on tokio and async rust. <!-- more --> I took these while working through the excellent [tokio tutorial](https://tokio.rs/tokio/tutorial).


# Overview

Tokio is an event-driven, non-blocking I/O platform for writing asynchronous applications with the Rust programming language. It excels at I/O tasks, where each individual task spends most of its time waiting for I/O. 

It provides the building blocks for networking applications. It comprises of:
- a multi-threaded runtime for executing async code
- an async version of the standard library
- an ecosystem of additional libraries

Not suitable for:
- parallelization of cpu tasks (use [rayon](https://docs.rs/rayon/latest/rayon/))
- singular http requests (use [reqwest](https://docs.rs/reqwest/latest/reqwest/))
- reading a lot of files (use a thread pool)
# Async in Rust
Asynchronous programming in rust is expressed through `async/await`. Rust compiles `async fn` into asynchronous routines, i.e. control is immediately returned to the thread. Rust's async operations are lazy, i.e. will only be executed once `await` is called.

The basics are documented in the book:
- [chapter 16 - concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [chapter 17 - async/await](https://doc.rust-lang.org/book/ch17-00-async-await.html)
- [send and async in the rustonomicon](https://doc.rust-lang.org/nomicon/send-and-sync.html) (also part of ch16)

# tokio
Builds on top of Rust primitives

```rust
#[tokio::main]
async fn main () {
	println!("hello there");
}
```

is transformed into 

```rust
fn main() {
	let mut rt = tokio::runtime::Runtime::new().unwrap();
	rt.block_on(async{
		println!("hello")
	})
}
```

## Tasks
A Tokio task is an asynchronous green thread (a thread scheduled by the runtime/VM, not by OS.) A task is created by passing an async block to `tokio::spawn`, e.g.:
```rust
tokio::spawn(async move {
	process(socket).await;
});
```

`move` here is used to move the closure's environment into it. This means `socket` will be moved. A task's type's lifetime must be `'static'`, i.e. it must not contain any references to data owned outside it. If data must be shared, a synchronization primitive such as `Arc` needs to be used.

`tokio::spawn` returns a `JoinHandle` which can be awaited to obtain a `Result`.

Tasks must implement the `Send` trait to move the tasks between threads. This is a rust concurrency thing. (see [chapter 16](https://doc.rust-lang.org/book/ch16-00-concurrency.html) in the book).

Tasks in Tokio are very lightweight. Under the hood, they require only a single allocation and 64 bytes of memory. Applications should feel free to spawn thousands, if not millions of tasks.

## Shared State

Two approaches:
- guard the shared state with a mutex (std)
- spawn a task to manage the sate and use message passing to operate on it

Generally speaking: use the first approach. The second comes into play with I/O. The usual contention considerations apply. 

Top-level mutexes in a task need to be handled specially, since they do not implement `Send`: Either wrap in a scope block or extract into a non-async function. Alternatively use a tokio async mutex (`tokio::sync::Mutex`), but this incurs a performance hit. 

### Handling contention
Use sharded mutexes.TIL: Java's `ConcurrentHashmap` is a sharded mutex.

## Channels
Why use channels?
- Tasks can't share resources freely (ownership)
- Using a mutex would introduce bottlenecks
- The alternative of creating connections per task is undesirable (**why?**)
=> Pattern: Message sending over channels

Types of channels:
- [mpsc](https://docs.rs/tokio/1/tokio/sync/mpsc/index.html): multi-producer, single-consumer channel. Many values can be sent.
- [oneshot](https://docs.rs/tokio/1/tokio/sync/oneshot/index.html): single-producer, single consumer channel. A single value can be sent.
- [broadcast](https://docs.rs/tokio/1/tokio/sync/broadcast/index.html): multi-producer, multi-consumer. Many values can be sent. Each receiver sees every value.
- [watch](https://docs.rs/tokio/1/tokio/sync/watch/index.html): multi-producer, multi-consumer. Many values can be sent, but no history is kept. Receivers only see the most recent value.

The standard library also has channels, but these are blocking. The `async-channel` crate has multi-producer, multi-consumer channels, where only one consumer sees each message.

# Further study

- https://tokio.rs/tokio/tutorial/io
- https://tokio.rs/tokio/tutorial/async
