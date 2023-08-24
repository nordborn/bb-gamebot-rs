use std::fmt::Debug;
use crate::game_types::{Card, Game};
use crate::{beat_solver, beat_solver_cards, util};
use anyhow::{Context, Result};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

pub fn run_zmq(port: String, must_stop: Arc<AtomicBool>) -> Result<()> {
    fn wrap<T>(t: T) -> String
        where T: Debug {
        format!("run_zmq: {:?}", t)
    }
    println!("STARTING MQ ON PORT {}", port);
    let ctx: zmq::Context = zmq::Context::new();
    let router = ctx.socket(zmq::ROUTER).with_context(|| wrap("router"))?;
    let addr = format!("tcp://*:{}", port);
    router.connect(&addr).with_context(|| wrap("connect"))?;

    while !util::read_atomic_bool(&must_stop) {
        println!("run_zmq: waiting for msgs");
        let req = router.recv_multipart(0);
        match req {
            Err(err) => eprintln!("run_zmq: BAD INPUT: {}", err),
            Ok(vecs) => match process_req(&vecs) {
                Err(err) => eprintln!("run_zmq: {}", err),
                Ok(card) => {
                    let msg_id = &vecs[0];
                    let msg = card.id;
                    let _ = router
                        .send(msg_id, zmq::SNDMORE.clone())
                        .map_err(|err| eprintln!("run_zmq: send msg_id: {:?}", err));
                    let _ = router
                        .send(&msg, 0)
                        .map_err(|err| eprintln!("run_zmq: send data: {:?}", err));
                }
            },
        }
    }
    println!("STOPPING GRACEFULLY");
    Ok(())
}

fn process_req(vecs: &Vec<Vec<u8>>) -> Result<Card> {
    let msg_id = &vecs[0];
    let body = std::str::from_utf8(&vecs[1]).with_context(|| "process_req: body")?;
    println!("got req: id={:?}, body={}", msg_id, body);
    Ok(solve(body).with_context(|| "process_req")?)
}

fn solve(body: &str) -> Result<Card> {
    use beat_solver_cards::{card_suites, lowest_power_card, shuffle_respecting_power};
    use beat_solver::beaters;

    let game: Game = serde_json::from_str(body).with_context(|| "solve: game from_str")?;
    let ai_cards = shuffle_respecting_power(&game.ai_cards);
    let human_cards = shuffle_respecting_power(&game.human_cards);
    let beater_suites = beaters(&card_suites(&human_cards), &card_suites(&ai_cards))
        .with_context(|| "solve")?;
    // peek last beater - the nearest move
    match beater_suites.last() {
        None => anyhow::bail!("solve: no last"),
        Some(&beater_suite) => match lowest_power_card(&ai_cards.clone(), beater_suite) {
            None => anyhow::bail!("solve: no lowest card with suite {:?}", beater_suite),
            Some(card) => Ok(card),
        },
    }
}
