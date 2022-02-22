use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    // Setup Tcp Listener and accept connections: ----------------
    let listener = TcpListener::bind("::1:8080").await.unwrap();
    // Create broadcast channel with number of items it can store inside it's internal state.
    let (tx, _) = broadcast::channel(10);
    // Start infinite loop 1
    loop {
        // Accept a new incoming connection the stream, address of the stream.
        let (mut cstreem, caddr) = listener.accept().await.unwrap();
        // Needs to be cloned to be used in send
        let tx = tx.clone();
        // Create new receiver
        let mut rx = tx.subscribe();
        // Spawns a new asynchronous task, returning a JoinHandle
        tokio::spawn(async move {
            // Split stream to reader and writer.
            let (reader, mut writer) = cstreem.split();
            // Create BufReader for reader.
            let mut breader = BufReader::new(reader);
            // Create value placeholder to read into.
            let mut line: String = Default::default();
            // Start infinite loop 2
            loop {
                // Do two tasks at the same time
                tokio::select! {
                    // Read message and put into the channel.
                    result = breader.read_line(&mut line) => {
                        // Check if read bytes == 0
                        if result.unwrap() ==  0 { break; }
                        // Put client message to the channel
                        tx.send((line.clone(), caddr)).unwrap();
                        // Clear the line
                        line.clear();
                    }
                    // Write message to client
                    result = rx.recv() => {
                        // Get message and client addr from channel
                        let (msg, addr) = result.unwrap();
                        // Write message back when it comes from different address
                        if caddr != addr { writer.write_all(&msg.as_bytes()).await.unwrap(); }
                    }
                };
            }
        });
    }
}
