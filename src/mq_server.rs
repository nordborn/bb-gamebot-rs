use crate::game_types::{Card, Game};
use crate::{beat_solver, beat_solver_cards, util};
use anyhow::{Context, Result};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

pub fn run_zmq(port: String, must_stop: Arc<AtomicBool>) -> Result<()> {
    info!("STARTING MQ ON PORT {}", port);
    let ctx: zmq::Context = zmq::Context::new();
    let router = ctx.socket(zmq::ROUTER)?;
    let addr = f!("tcp://*:{port}");
    router.connect(&addr).with_context(|| f!("{addr=}"))?;

    while !util::read_atomic_bool(&must_stop) {
        info!("waiting for msgs");
        match router.recv_multipart(0) {
            Err(err) => error!("{:?}", err),
            Ok(vecs) => {
                let msg = match process_req(&vecs) {
                    Err(err) => {
                        error!("{:?}", err);
                        f!("error: {err}")
                    }
                    Ok(card) => card.id,
                };
                let msg_id = &vecs[0];
                info!("send resp: id={:?}, msg={}", msg_id, msg);
                let _ = router
                    .send(msg_id, zmq::SNDMORE)
                    .map_err(|err| error!("{:?}", err));
                let _ = router
                    .send(&msg, 0)
                    .map_err(|err| error!("{:?}", err));
            }
        }
    }
    info!("STOPPING GRACEFULLY");
    Ok(())
}

fn process_req(vecs: &[Vec<u8>]) -> Result<Card> {
    let msg_id = &vecs[0];
    let body = std::str::from_utf8(&vecs[1])?;
    info!("got req: id={:?}, body={}", msg_id, body);
    solve(body)
}

fn solve(body: &str) -> Result<Card> {
    use beat_solver::beaters;
    use beat_solver_cards::{card_suites, lowest_power_card, shuffle_respecting_power};

    let game: Game = serde_json::from_str(body)?;
    let ai_cards = shuffle_respecting_power(&game.ai_cards);
    let human_cards = shuffle_respecting_power(&game.human_cards);
    let beater_suites =
        beaters(&card_suites(&human_cards), &card_suites(&ai_cards))?;
    // peek last beater - the nearest move
    match beater_suites.last() {
        None => anyhow::bail!("no last"),
        Some(&beater_suite) => Ok(lowest_power_card(&ai_cards.clone(), beater_suite)?),
    }
}
