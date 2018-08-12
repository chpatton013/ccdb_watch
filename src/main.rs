extern crate libc;
extern crate fswatch;

use fswatch::{ffi, Fsw, FswSession};

fn main() {
  println!("Initializing fswatch library");
  Fsw::init_library().expect("Could not start fswatch");

  println!("Creating a new session");
  let session = FswSession::default().unwrap();

  println!("Adding a new monitor path");
  session.add_path("./").unwrap();

  println!("Setting an event callback");
  session.set_callback(|events| {
    println!("{:#?}", events);
  }).unwrap();

  println!("Starting the monitor");
  session.start_monitor().unwrap();
}
