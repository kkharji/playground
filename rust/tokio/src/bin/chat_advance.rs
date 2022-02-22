// Alternative implementation. based on https://github.com/tokio-rs/tokio/blob/master/examples/chat.rs
#![allow(dead_code, unused_imports)]

use futures::sink::SinkExt; // Requires: futures = "0.3.21", used in Framed::send
use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use std::{env, io};

use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use tokio::{select, spawn};

use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::EnvFilter;

/// Shorthand for the transmit half of the message channel.
type Tx = mpsc::UnboundedSender<String>;

/// Shorthand for the receive half of the message channel.
type Rx = mpsc::UnboundedReceiver<String>;

/// Data that is shared between all peers in the chat server.
struct State {
    peers: HashMap<SocketAddr, Tx>,
}

/// Shorthand for State wrapped with Arc and Mux
type SharedState = Arc<Mutex<State>>;

impl State {
    /// Initialize new state, Wrapped with Arc and Mutex
    fn new() -> SharedState {
        Arc::new(Mutex::new({
            Self {
                peers: HashMap::new(),
            }
        }))
    }

    /// Sends a `LinesCodec` encoded message to every peer, expect for the sender
    /// Whenever a it is called, it iterates over the `peers` and send a copy of the message
    async fn broadcast(&mut self, sender: SocketAddr, message: &str) {
        self.peers.iter_mut().for_each(|(addr, tx)| {
            if *addr != sender {
                tx.send(message.into()).ok();
            }
        })
    }
}

/// The state for each connected client.
struct Peer {
    /// The TCP socket wrapped with the `Lines` codec.
    ///
    /// This handles sending and receiving data on the socket. When using
    /// `Lines`, we can work at the line level instead of having to manage the
    /// raw byte operations.
    lines: Framed<TcpStream, LinesCodec>,

    /// Receive half of the message channel. used to receive messages from peers.
    /// When a message is received off of this `Rx`, it will be written to the socket.
    rx: Rx,
}

impl Peer {
    async fn new(state: SharedState, lines: Framed<TcpStream, LinesCodec>) -> io::Result<Self> {
        let addr = lines.get_ref().peer_addr()?; // Get the client socket address.
        let (tx, rx) = mpsc::unbounded_channel(); // Create a channel for this peer.
        state.lock().await.peers.insert(addr, tx); // Add an entry for this `Peer` in the shared state map.
        Ok(Self { lines, rx })
    }
}

async fn process(
    state: SharedState,
    stream: TcpStream,
    addr: SocketAddr,
) -> Result<(), Box<dyn Error>> {
    let mut lines = Framed::new(stream, LinesCodec::new());

    // Prompt client to enter value.
    lines.send("Please enter your username:").await?;

    // Read the first line from the `LineCodec` stream to get the username.
    let username = match lines.next().await {
        Some(Ok(line)) => line,
        _ => {
            tracing::error!("Failed to get username from {}. Client disconnected.", addr);
            return Ok(());
        }
    };

    // Register our peer with state which inernally sets some channels
    let mut peer = Peer::new(state.clone(), lines).await?;

    // A client has connected, notify other clients
    {
        let msg = format!("{username}: joined");
        tracing::info!("{msg}");
        state.lock().await.broadcast(addr, &msg).await;
    }

    // Process incoming messages until our stream is exhausted by a disconnected
    loop {
        select! {
            // A message was received from a peer. Send it to the current user.
            Some(msg) = peer.rx.recv() => peer.lines.send(&msg).await?,
            result = peer.lines.next() => match result {
                // A message was received from the current user, we should brodcast
                Some(Ok(msg)) => state.lock().await.broadcast(addr, &format!("{username}: {msg}")).await,
                // An Error occurred
                Some(Err(err)) => tracing::error!("{}: Error while processing messages; {:?}", username, err),
                // No more messages
                None =>  break,
            }
        }
    }

    // If this section is reached it mean that the client was disconnected!
    // Notify everyone
    {
        let mut state = state.lock().await;
        state.peers.remove(&addr);
        let msg = format!("{username} has left the chat");
        tracing::info!("{msg}");
        state.broadcast(addr, &msg).await;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Configure tracing_subscriber
    tracing_subscriber::fmt()
        // Filter what traces are displayed based on RUST_LOG var: `RUST_LOG=tokio=trace`
        .with_env_filter(EnvFilter::from_default_env().add_directive("chat=info".parse()?))
        // Log events when `tracing` spans are created, entered, existed, or closed.
        .with_span_events(FmtSpan::FULL)
        // Set this subscriber as the default, to collect all traces emitted by the programmer.
        .init();

    // Create shared state. This is how all the peers communicate
    let state = State::new();

    let listener = TcpListener::bind("::1:8080").await?;
    tracing::info!("server running on localhost::8080");
    loop {
        // Asynchronously wait for an inbound TcpStream.
        let (stream, addr) = listener.accept().await?;
        // Clone a handle to the `SharedState` for the new connection.
        let state = state.clone();
        // Spawn our handler to run asynchronously
        spawn(async move {
            tracing::debug!("accepted connection");
            if let Err(e) = process(state, stream, addr).await {
                tracing::info!("an error occurred; error = {:?}", e);
            }
        });
    }
}
