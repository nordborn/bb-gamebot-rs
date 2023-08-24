mod beat_solver;
mod beat_solver_cards;
mod game_types;
mod mq_server;
mod util;

use anyhow::Result;
use signal_hook::consts::{SIGINT, SIGTERM};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::mq_server::run_zmq;

fn main() -> Result<()> {
    println!("RUN GAME SOLVER");
    let port: String = util::get_env("ZMQ_PORT")?;
    let must_stop = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(SIGTERM, Arc::clone(&must_stop))?;
    signal_hook::flag::register(SIGINT, Arc::clone(&must_stop))?;

    let must_stop_spawn = Arc::clone(&must_stop);
    thread::spawn(move || {
        let _ = run_zmq(port, must_stop_spawn).map_err(|err| eprintln!("ERR: {:?}", err));
    });

    while !util::read_atomic_bool(&must_stop){
        thread::sleep(Duration::from_secs(1));
    }

    println!("STOPPING WITH DELAY");
    thread::sleep(Duration::from_secs(10));
    println!("STOPPED");
    Ok(())
}
