//! Helper type for processing process output and exit status in non-blocking way
use crossbeam_channel::{unbounded, Receiver, SendError, Sender};
use std::io::{self, prelude::*, BufReader};
use std::process::{Child, ExitStatus};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

type ProcessHandle = JoinHandle<Result<(), SendError<Option<Output>>>>;
type OutputReceiver = Receiver<Option<Output>>;
type OutputSender = Sender<Option<Output>>;

struct ProcessHandlers {
    stdout: ProcessHandle,
    stderr: ProcessHandle,
    status: ProcessHandle,
}

/// Process struct for handleing a [`Child`] in non-blocking way
pub struct Process {
    inner: Arc<Mutex<Child>>,
    rx: OutputReceiver,
    handlers: ProcessHandlers,
    mtx: Sender<bool>,
}

impl Process {
    /// Create new process from [`Child`]
    pub fn new(mut process: Child) -> Process {
        let (tx, rx) = unbounded();
        let (mtx, mrx) = unbounded();

        let stdout = process.stdout.take().unwrap();
        let stderr = process.stderr.take().unwrap();

        let inner = Arc::new(Mutex::new(process));

        let handlers = ProcessHandlers {
            stdout: spawn_reader(true, stdout, tx.clone(), mrx.clone(), mtx.clone()),
            stderr: spawn_reader(false, stderr, tx.clone(), mrx.clone(), mtx.clone()),
            status: spawn_status_thread(inner.clone(), tx.clone(), mrx.clone(), mtx.clone()),
        };

        Process {
            inner,
            rx,
            mtx,
            handlers,
        }
    }

    /// Get iteratorable stream of outputs
    pub fn stream(&mut self) -> OutputStream {
        OutputStream {
            rx: &mut self.rx,
            exit: false,
        }
    }

    /// Block current thread until the process exist.
    pub fn wait(&self) -> io::Result<ExitStatus> {
        self.inner.lock().unwrap().wait()
    }

    /// Kill running process and close all channels
    pub fn kill(self) -> Option<()> {
        self.mtx.send(true).unwrap();
        self.handlers.stdout.join().unwrap().unwrap();
        self.handlers.stderr.join().unwrap().unwrap();
        self.handlers.status.join().unwrap().unwrap();
        let mut child = match self.inner.lock() {
            Ok(p) => p,
            Err(e) => e.into_inner(),
        };
        child.kill().unwrap();

        Some(())
    }
}

/// OutputStream iterator
pub struct OutputStream<'a> {
    rx: &'a mut OutputReceiver,
    exit: bool,
}

impl<'a> OutputStream<'a> {
    /// No blocking equivalent of next
    pub fn try_next(&mut self) -> Option<Output> {
        if let Ok(Some(output)) = self.rx.try_recv() {
            Some(output)
        } else {
            None
        }
    }
}

impl<'a> Iterator for OutputStream<'a> {
    type Item = Output;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exit {
            None
        } else if let Ok(Some(output)) = self.rx.recv() {
            self.exit = matches!(output, Output::Exit(..));
            Some(output)
        } else {
            None
        }
    }
}

/// Output produced by [`Process`] and [`OutputStream`]
#[derive(Debug)]
pub enum Output {
    /// Source stdout
    Out(String),
    /// Source stderr or internal io::Error
    Err(String),
    /// Exit status
    Exit(Result<Option<i32>, io::Error>),
}

fn spawn_reader<R: Read + Send + 'static>(
    is_stdout: bool,
    out: R,
    tx: OutputSender,
    mrx: Receiver<bool>,
    mtx: Sender<bool>,
) -> ProcessHandle {
    thread::spawn(move || {
        let stdout_reader = BufReader::new(out);
        let mut lines = stdout_reader.lines();
        loop {
            if let Ok(stop) = mrx.try_recv() {
                if stop {
                    mtx.send(stop).ok();
                    break;
                }
            }
            let output = match lines.next() {
                Some(Ok(line)) if is_stdout => Output::Out(line),
                Some(Ok(line)) => Output::Err(line),
                Some(Err(e)) => Output::Err(e.to_string()),
                _ => continue,
            };

            tx.send(Some(output))?;
        }
        Ok(())
    })
}

fn spawn_status_thread(
    shared_child: Arc<Mutex<Child>>,
    tx: Sender<Option<Output>>,
    mrx: Receiver<bool>,
    mtx: Sender<bool>,
) -> ProcessHandle {
    thread::spawn(move || {
        loop {
            if let Ok(stop) = mrx.try_recv() {
                if stop {
                    mtx.send(stop).ok();
                    break;
                }
            }
            if let Ok(ref mut child) = shared_child.try_lock() {
                match child.try_wait() {
                    Err(err) => {
                        tx.send(Some(Output::Exit(Err(err))))?;
                        break;
                    }
                    Ok(Some(status)) => {
                        tx.send(Some(Output::Exit(Ok(status.code()))))?;
                        break;
                    }
                    Ok(None) => continue,
                };
            };
        }

        Ok(())
    })
}

