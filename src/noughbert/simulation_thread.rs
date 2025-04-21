use std::sync::mpsc::{Receiver, SyncSender};

use ego_tree::NodeId;

use crate::game::{
    game::{Game, Turn},
    value::Value,
};

use super::message::ExplorationRequest;

pub fn simulation_thread(
    rx: Receiver<ExplorationRequest>,
    tx: SyncSender<ExplorationRequest>,
    id: NodeId,
    mut game: Game,
    opt_for: Turn,
) {
    // eprintln!("Thread {:?}: Starting simulation", id);
    while game.board.check() == Value::None {
        // Check for incoming messages
        match rx.try_recv() {
            Ok(m) => match m {
                ExplorationRequest::Stop => {
                    // eprintln!("Thread {:?}: Stopping", id)
                }
                ExplorationRequest::Return { result: _ } => {}
            },
            Err(e) => match e {
                std::sync::mpsc::TryRecvError::Empty => {}
                std::sync::mpsc::TryRecvError::Disconnected => {
                    eprintln!(
                        "Thread {:?}: Stopping due to disconnect from main thread",
                        id
                    )
                }
            },
        }

        game.play(fastrand::choice(game.legal_moves().iter()).unwrap())
            .unwrap();
    }

    let val = if game.board.check() == opt_for.val() {
        1.0
    } else if game.board.check() == Value::Draw {
        0.0
    } else {
        -1.0
    };

    // eprintln!("Thread {:?}: Sending result {}", id, val);
    let _ = tx.send(ExplorationRequest::Return { result: val });

    // eprintln!("Thread {:?}: Finished", id)
}
