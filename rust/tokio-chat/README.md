# Creating a chat server with async Rust and Tokio

[Source - YouTube]

- Asynchronous non-blocking code
- Allows multiple TCP client to connect.

## Async Main function

In rust, writing `async` in front of `fn` makes the function asynchronous.
However, as of now, rust can't have main function as async. It would raise an error
like: `main` function is not allowed to be `async`, or expected value, found
trait `future::Future`.

## `std::future::Future`

a `Future` is a thing that doesn't have a known value yet (at least until competition is done).
According to rust-docs, it represents an asynchronous computation. An
asynchronous value, that makes it possible for a thread to continue doing
useful work while it waits for the value to become available.

## `await` keyword

Tells the compile to suspend the function until their is a value?. Though it
only blocks on thread level, not on task level.

Task is a unit of work in async work.


## TCP Echo Server

Echo server is a server that wait for a client to connect, once the client
connects, it will take any message the client sent and send it back to the
client.

```rust
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};
\#[tokio::main]
async fn main() {
    // Setup Tcp Listener and accept connections: ----------------
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    // Accept a new incoming connection
    // the stream, address of the stream.
    let (mut stream, _addr) = listener.accept().await.unwrap();

    // Read and write data: ---------------------------------------

    // Create a buffer of specific size.
    let mut buffer = [0u8; 1024];

    // Read data into `buffer`. returns number of buffer read
    let bytes_read = stream.read(&mut buffer).await.unwrap();

    // write back to the stream upto the bytes read.
    stream.write_all(&buffer[..bytes_read]).await.unwrap();
}
```

### Support multiple read and write

As soon as the client connect, we need to drop to an infinit loop that will read then write data.

```rust
loop {
    let mut buffer = [0u8; 1024];
    // -----
}
```

## `tokio::BufReader`

`tokio::BufReader` is helpful type for doing io operations. It job is to wrap
any type of reader, and it enable muliple read operations. like read entire
line.

```rust
\#[tokio::main]
async fn main() {
    // Setup Tcp Listener and accept connections: ----------------
    let listener = TcpListener::bind("::1:8080").await.unwrap();

    // Accept a new incoming connection the stream, address of the stream.
    let (mut cstream, _addr) = listener.accept().await.unwrap();

    // Split stream to reader and writer.
    let (reader, mut writer) = cstream.split();

    // Create BufReader for reader.
    let mut breader = BufReader::new(reader);

    // Create value placeholder to read into.
    let mut line = String::new();

    // loop ensure that we keep trying to read from the cstream
    loop {
        // Read data into `line`. returns number of buffer read
        let bytes_read = breader.read_line(&mut line).await.unwrap();

        // Ensure that cstream is still connected through checking bytes_read length
        if bytes_read.eq(&0) {
            break;
        }

        // Write back to the stream using line bytes
        writer.write_all(&line.as_bytes()).await.unwrap();

        // Clear the line
        line.clear();
    }
}
```

## Support multiple clients

### `tokio::task::spawn` or `tokio::spawn`

`tokio::spawn` spawns a new asynchronous task and return [`JoinHandle`].

Spawning a task enables the task to execute concurrently to other tasks that
can be execute on the same thread or on other thread. The specifics depend on
the current [`Runtime`].

### Async blocks

Unlike many language, rust support async blocks which can be thought of as isolate async task.

### All together
```rust
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

\#[tokio::main]
async fn main() {
    // Setup Tcp Listener and accept connections: ----------------
    let listener = TcpListener::bind("::1:8080").await.unwrap();

    loop {
        // Accept a new incoming connection the stream, address of the stream.
        let (mut cstream, _addr) = listener.accept().await.unwrap();

        // Spawns a new asynchronous task, returning a JoinHandle
        tokio::spawn(async move {
            // body of the version using BufReader
        });
    }
}
```

# Chat Server

To allow clients to exchange messages, we need to have some types of broadcast
mechanism. `tokio` provide `broadcast` type as it allows multiple producesser and
multiple consumers to send and receive on a single channel.

## `tokio::select`
Macro that run async function at the same time. It implicitly run await on functions needs awaiting

## All Togther
```rust
use std::net::SocketAddr;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

\#[tokio::main]
async fn main() {
    // Setup Tcp Listener and accept connections: ----------------
    let listener = TcpListener::bind("::1:8080").await.unwrap();
    // Create broadcast channel with number of items it can store inside it's internal state.
    let (tx, _) = broadcast::channel::<(String, SocketAddr)>(10);
    // Start infinite loop 1
    loop {
        // Accept a new incoming connection the stream, address of the stream.
        let (mut cstream, client_addr) = listener.accept().await.unwrap();
        // Needs to be cloned to be used in send
        let tx = tx.clone();
        // Create new receiver
        let mut rx = tx.subscribe();
        // Spawns a new asynchronous task, returning a JoinHandle
        tokio::spawn(async move {
            // Split stream to reader and writer.
            let (reader, mut writer) = cstream.split();
            // Create BufReader for reader.
            let mut breader = BufReader::new(reader);
            // Create value placeholder to read into.
            let mut line = String::new();
            // Start infinite loop 2
            loop {
                // Do two tasks at the same time
                tokio::select! {
                    // Read message and put into the channel.
                    result = breader.read_line(&mut line) => {
                        // Check if read bytes == 0
                        if result.unwrap() ==  0 { break; }
                        // Put cstream message to the channel
                        tx.send((line.clone(), client_addr)).unwrap();
                        // Clear the line
                        line.clear();
                    }
                    // Write message to cstream
                    result = rx.recv() => {
                        // Get message and cstream addr from channel
                        let (msg, addr) = result.unwrap();
                        // Write message back when it comes from different address
                        if client_addr != addr {
                            writer.write_all(&msg.as_bytes()).await.unwrap();
                        }
                    }
                };
            }
        });
    }
}
```

## `tokio::select!` vs `tokio::spawn`
`tokio::select` is when you need something to operate on the same state and you have finite.
`tokio::spawn` when you don't need to share reference to  a variable


[Source - YouTube]: https://www.youtube.com/watch?v=4DqP57BHaXI
