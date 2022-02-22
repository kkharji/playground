# Creating a chat server with async Rust and Tokio

Notes I manged to write from watching [Lily Mara stream].

## Notes

### std::future::Future
a `Future` is a thing that doesn't have a known value yet (at least until competition is done).
According to rust-docs, it represents an asynchronous computation. An
asynchronous value, that makes it possible for a thread to continue doing
useful work while it waits for the value to become available.

### await
Tells the compile to suspend the function until their is a value?. Though it
only blocks on thread level, not on task level.

Task is a unit of work in async work.

### tokio::BufReader
`tokio::BufReader` is helpful type for doing io operations. It job is to wrap
any type of reader, and it enable multiple read operations. like read entire
line.

For example: Echo server with and without `tokio::BufReader`

### #[tokio::main]
In rust, writing `async` in front of `fn` makes the function asynchronous.
However, as of now, rust can't have main function as async. It would raise an error
like: `main` function is not allowed to be `async`, or expected value, found
trait `future::Future`.

To fix this we need to annotate main function with `#[tokio::main]`

### tokio::spawn

`tokio::spawn` spawns a new asynchronous task and return [`JoinHandle`].

Spawning a task enables the task to execute concurrently to other tasks that
can be execute on the same thread or on other thread. The specifics depend on
the current [`Runtime`].

## tokio::select
Macro that run async function at the same time. It implicitly run await on functions needs awaiting


## tokio::select vs tokio::spawn
`tokio::select` is when you need something to operate on the same state and you have finite.
`tokio::spawn` when you don't need to share reference to  a variable



# Snippets

```toml
[dependencies]
tokio = { version = "1.17.0", features = ["full"]  } # don't have to be full, but for sake of getting started
```

## TCP Echo Server
Echo server is a server that wait for a client to connect, once the client
connects, it will take any message the client sent and send it back to the
client.

### Required imports
```rust
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};
```

### Snippet

```rust
#[tokio::main]
async fn main() {
    // Setup Tcp Listener and accept connections: ----------------
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    // Accept a new incoming connection // the stream, address of the stream.
    let (mut stream, _addr) = listener.accept().await.unwrap();
    // Create a buffer of specific size.
    let mut buffer = [0u8; 1024];
    // Read data into `buffer`. returns number of buffer read
    let bytes_read = stream.read(&mut buffer).await.unwrap();
    // write back to the stream upto the bytes read.
    stream.write_all(&buffer[..bytes_read]).await.unwrap();
}
```

### Fix: multiple read and write

As soon as the client connect, we need to drop to an infinit loop that will read then write data.

```rust
loop {
    let mut buffer = [0u8; 1024];
    // -----
}
```

### with `tokio::BufReader`

```rust
let (mut cstream, _addr) = listener.accept().await.unwrap(); // Accept a new incoming connection the stream, address of the stream.
let (reader, mut writer) = cstream.split(); // Split stream to reader and writer.
let mut breader = BufReader::new(reader);    // Create BufReader for reader.
let mut line = String::new(); // Create value placeholder to read into.

loop { // loop ensure that we keep trying to read from the cstream
    let bytes_read = breader.read_line(&mut line).await.unwrap(); // Read data into `line`. returns number of buffer read
    if bytes_read.eq(&0) { break; } // Ensure that cstream is still connected via checking bytes_read value
    writer.write_all(&line.as_bytes()).await.unwrap(); // Write back to the stream using line bytes
    line.clear(); // Clear the line
}
```

### Handle Multiple clients

```rust
// ---------
loop {
    let (mut cstream, _addr) = listener.accept().await.unwrap();
    // Spawns a new asynchronous task, returning a JoinHandle
    tokio::spawn(async move { /* body of the version using BufReader*/  });
}
```

# Chat Server
To allow clients to exchange messages, we need to have some types of broadcast
mechanism. `tokio` provide `broadcast` type as it allows multiple producers and
multiple consumers to send and receive on a single channel.

### Required imports
```rust
use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::TcpListener, sync::broadcast};
```

### Snippet

```rust
let (tx, _) = broadcast::channel(10); // Create broadcast channel with size
loop { // Start infinite loop 1
    let (mut cstream, client_addr) = listener.accept().await.unwrap(); // Accept a new incoming connection.
    let tx = tx.clone();         // Create new receiver,  Needs to be cloned to be used in send
    let mut rx = tx.subscribe(); // Spawns a new asynchronous task, returning a JoinHandle

    tokio::spawn(async move {
        let (reader, mut writer) = cstream.split(); // Split stream to reader and writer.
        let mut breader = BufReader::new(reader);   // Create BufReader for reader.
        let mut line = String::new();               // Create value placeholder to read into.
        loop { // Start infinite loop 2
            tokio::select! {                                // Do two tasks at the same time
                result = breader.read_line(&mut line) => {  // Read message and put into the channel.
                    if result.unwrap() ==  0 { break; }     // Check if read bytes == 0
                    tx.send((line.clone(), client_addr)).ok(); // Put cstream message to the channel
                    line.clear(); // Clear the line
                }
                result = rx.recv() => {                // Write message to cstream
                    let (msg, addr) = result.unwrap(); // Get message and cstream addr from channel
                    if client_addr != addr {           // Write message back when it from comes from different client
                        writer.write_all(&msg.as_bytes()).await.unwrap();
                    }
                }
            };
        }
    });
}
```

[Lily Mara stream]: https://www.youtube.com/watch?v=4DqP57BHaXI
