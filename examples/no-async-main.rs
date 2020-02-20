//! A simple client that does some work and triggers a callback asynchronously
//! with an API can be called from no-async code and uses async tokio inside
//!
//! To run the example:
//!
//!     cargo run --example no-async-main
use core::future::Future;
use std::sync::{Arc, Mutex};
use tokio::runtime::{Handle, Runtime};
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;
use Command::*;

#[derive(Debug)]
enum Command {
    Increment,
    Shutdown,
    // Other commands can be added here
}

#[derive(Debug)]
enum Response {
    IncrementCompleted(u64),
}

struct RuntimeContext {
    runtime: Runtime,
    cmd_handler: Option<JoinHandle<()>>,
    tasks: Vec<i32>,
}
struct Commander {
    context: Arc<Mutex<RuntimeContext>>,
}

#[derive(Clone)]
struct Sender {
    handle: Handle,
    cmd_tx: mpsc::Sender<(Command, oneshot::Sender<Response>)>,
}

impl Commander {
    pub fn new() -> Commander {
        let runtime = Runtime::new().expect("new tokio Runtime");
        Commander {
            context: Arc::new(Mutex::new(RuntimeContext {
                runtime,
                cmd_handler: None,
                tasks: Vec::new(),
            })),
        }
    }
    pub fn connect(&self, ready_callback: impl Fn(Sender) -> () + Send + 'static) {
        let mut context = self.context.lock().unwrap();

        let (cmd_tx, mut cmd_rx) = mpsc::channel::<(Command, oneshot::Sender<Response>)>(100);
        // Spawn a task to manage the counter
        let handle = context.runtime.spawn(async move {
            let mut counter: u64 = 0;
            while let Some((cmd, response)) = cmd_rx.recv().await {
                match cmd {
                    Shutdown => {
                        break;
                    }
                    Increment => {
                        counter += 1;
                        response
                            .send(Response::IncrementCompleted(counter))
                            .unwrap();
                    }
                }
            }
            println!("completed async task, originally spawned in connect");
        });
        context.cmd_handler = Some(handle);
        ready_callback(Sender::new(cmd_tx, context.runtime.handle().clone()));
    }
}

impl Sender {
    pub fn new(cmd_tx: mpsc::Sender<(Command, oneshot::Sender<Response>)>, handle: Handle) -> Self {
        Sender { handle, cmd_tx }
    }

    pub fn send_command(&mut self, cmd: Command, f: impl Fn(Response) -> () + Send + 'static) {
        let (resp_tx, resp_rx) = oneshot::channel::<Response>();
        let mut cmd_tx = self.cmd_tx.clone();
        self.handle.spawn(async move {
            cmd_tx.send((cmd, resp_tx)).await.ok().unwrap();
            let res: Response = resp_rx.await.unwrap();

            println!("  => {:?}", res);
            f(res);
        });
    }
} // impl Sender

fn main() {
    let c = Commander::new();
    c.connect(move |mut sender| {
        println!("Yay!");
        sender.send_command(Increment, |response| {
            println!("  ==> received response {:?}", response);
        })
    });

    let mut input = String::new();
    println!("press return to quit");
    std::io::stdin()
        .read_line(&mut input)
        .expect("stdio read_line");
}
