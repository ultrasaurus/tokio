//! A simple client that does some work and triggers a callback asynchronously
//! with an API can be called from no-async code and uses async tokio inside
//!
//! To run the example:
//!
//!     cargo run --example no-async-main
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
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
  cmd_handler: JoinHandle<()>,
  tasks: Vec<JoinHandle<()>>,
}

#[derive(Clone)]
struct Commander {
  context: Arc<Mutex<RuntimeContext>>,
  cmd_tx: mpsc::Sender<(Command, oneshot::Sender<Response>)>,
}

impl Commander {
  pub fn new() -> Commander {
    let runtime = Runtime::new().expect("new tokio Runtime");
    let (cmd_tx, mut cmd_rx) = mpsc::channel::<(Command, oneshot::Sender<Response>)>(100);
    // Spawn a task to manage the counter
    let cmd_handler = runtime.spawn(async move {
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
    Commander {
      cmd_tx,
      context: Arc::new(Mutex::new(RuntimeContext {
        runtime,
        cmd_handler,
        tasks: Vec::new(),
      })),
    }
  }

  pub fn send_command(&self, cmd: Command, f: impl Fn(Commander, Response) -> () + Send + 'static) {
    println!("send_command, cmd_tx = {:?}", self.cmd_tx);
    let (resp_tx, resp_rx) = oneshot::channel::<Response>();
    let mut cmd_tx = self.cmd_tx.clone();

    let mut context = self.context.lock().unwrap();
    let commander_for_callback = self.clone();
    let handle = context.runtime.spawn(async move {
      cmd_tx.send((cmd, resp_tx)).await.ok().unwrap();
      let res: Response = resp_rx.await.unwrap();

      println!("  => {:?}", res);
      f(commander_for_callback, res);
    });
    context.tasks.push(handle);
  }
} // impl Sender

fn main() {
  let c = Commander::new();
  c.send_command(Increment, |cmder, response| {
    println!("  ==> received response {:?}", response);
    cmder.send_command(Increment, move |_, response| {
      println!("  ==> received response {:?}", response);
    });
  });
  c.send_command(Increment, |_, response| {
    println!("  ==> received response {:?}", response);
  });
  let mut input = String::new();
  println!("press return to quit");
  std::io::stdin()
    .read_line(&mut input)
    .expect("stdio read_line");
}
