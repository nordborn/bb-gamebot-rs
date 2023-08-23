mod beat_solver;
mod beat_solver_cards;
mod game_types;
mod mq_server;
mod util;

use anyhow::Result;
use signal_hook::consts::{SIGINT, SIGTERM};
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::channel;
use std::sync::Arc;

use crate::mq_server::run_zmq;
use crate::util::get_env;

// extern crate rand;
// extern crate serde;

fn main() -> Result<()> {
    println!("RUN GAME SOLVER");
    let port: String = get_env("ZMQ_PORT")?;
    let (tx_stopped, rx_stopped) = channel();

    let must_stop = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(SIGTERM, Arc::clone(&must_stop))?;
    signal_hook::flag::register(SIGINT, Arc::clone(&must_stop))?;

    std::thread::spawn(move || {
        match run_zmq(port, must_stop) {
            Ok(()) => (),
            Err(err) => println!("ERR: {:?}", err),
        }
        println!("STOPPED");
        tx_stopped.send(true)
    });

    rx_stopped.recv()?;
    Ok(())
}
