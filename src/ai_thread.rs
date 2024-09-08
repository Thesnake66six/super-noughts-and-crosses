use std::{
    fs,
    path::Path,
    process::Command,
    sync::mpsc::{self, Receiver, SyncSender},
    time,
};

use crate::{
    game::{game::Turn, value::Value},
    noughbert::{
        message::{Message, Thoughts},
        monte_carlo::MonteCarloManager,
    },
    styles::{AUTOCOMPILE_GRAPHVIS_FILES, OUTPUT_GRAPHVIS_FILES},
};

pub fn noughbert(rx: Receiver<Message>, tx: SyncSender<Message>) {
    // Count the number of AI calls
    let mut runs = 0;

    // Clear and re-create the `./outs` folder
    if OUTPUT_GRAPHVIS_FILES {
        let _ = fs::remove_dir_all("./outs");
        let _ = fs::create_dir("./outs");
    }

    loop {
        // Recieve all messages, if a `Message::Start()` is recieved, begin simulation
        let message = rx.recv().unwrap();
        let mc_options = match message {
            Message::Start(x) => x,
            Message::Interrupt => continue,
            Message::GetThoughts(_) => continue,
            Message::Thoughts(_) => continue,
            Message::Move(_) => continue,
            Message::Return() => continue,
        };

        println!("Simulation requested");

        let mut noughbert = MonteCarloManager::new(mc_options.game, mc_options.opt_for);
        let start_time = time::Instant::now();
        let mut interrupt = false;
        let mut interrupt_return = true;

        // Make sure a move is never requested on a completed board state
        assert_eq!(noughbert.g.board.check(), Value::None);

        if noughbert.g.board.check() != Value::None {
            interrupt = true;
        }

        // Start new iteration within current bounds
        while start_time.elapsed() < mc_options.timeout
            && noughbert.sims < mc_options.max_sims
            && !interrupt
        {
            // Recieve all messages; Break if interrupted
            let message = rx.try_recv();
            match message {
                Ok(m) => match m {
                    Message::Start(_) => {}
                    Message::Interrupt => {
                        interrupt = true;
                        break;
                    }
                    Message::GetThoughts(t) => {
                        dbg!("Thoughts requested");
                        let root = noughbert.tree.root().value();
                        tx.send(Message::Thoughts(Thoughts {
                            sims: noughbert.sims,
                            score: root.score(t),
                        }))
                        .unwrap();
                    }
                    Message::Thoughts(_) => {}
                    Message::Move(_) => {}
                    Message::Return() => {
                        interrupt_return = true;
                        break;
                    }
                },
                Err(e) => match e {
                    mpsc::TryRecvError::Empty => {}
                    mpsc::TryRecvError::Disconnected => panic!("Thread disconnected"),
                },
            }
            // Run the MCTS algorithm once
            let x = noughbert.select(mc_options.exploration_factor, mc_options.opt_for);
            if x.is_none() {
                break;
            }
            let x = noughbert.expand(x.unwrap());
            let (x, val) = noughbert.simulate(x, mc_options.opt_for);
            noughbert.backpropogate(x, val);
            // Increment the number of simulations run
            noughbert.sims += 1;
        }
        // Print the reason for the cycle ending
        if interrupt {
            println!("Exited due to interrupt request");
            continue;
        } else if interrupt_return {
            println!("Exited due to return request");
        } else if noughbert.sims >= mc_options.max_sims {
            println!("Exited due to simulation cap");
        } else if start_time.elapsed() >= mc_options.timeout {
            println!("Exited due to timeout");
        } else {
            println!("Exited due to complete game tree");
        }
        println!(
            "Move selected after {} sims and {} seconds.",
            noughbert.sims,
            start_time.elapsed().as_secs_f32()
        );

        // Calculate the best play
        let best_play = noughbert.best(
            mc_options.policy,
            mc_options.opt_for,
            mc_options.exploration_factor,
        );

        // Send the best move calculated and increment the runs counter
        tx.send(Message::Move(best_play)).unwrap();
        runs += 1;

        // If needed, output the node `.svg` files
        if OUTPUT_GRAPHVIS_FILES {
            let _ = fs::write(Path::new(&format!("./outs/{runs}.dot")), {
                let s = format!(
                    "{}",
                    graphvis_ego_tree::TreeWrapper::new(
                        &noughbert.tree,
                        noughbert.tree.root().id(),
                        |node| {
                            let mut board = noughbert.g.clone();
                            let id = format!("Node ID: {:?}", node.id());
                            let play = {
                                if node.id() == noughbert.tree.root().id() {
                                    format!(
                                        "Starting Board: {}",
                                        match noughbert.g.turn {
                                            Turn::Player1 => "Crosses".to_owned(),
                                            Turn::Player2 => "Noughts".to_owned(),
                                        }
                                    )
                                } else {
                                    match node.value().turn {
                                        Turn::Player1 => {
                                            format!("Crosses' turn: {:?}", &node.value().play)
                                        }
                                        Turn::Player2 => {
                                            format!("Noughts' turn: {:?}", &node.value().play)
                                        }
                                    }
                                }
                            };

                            for x in node.ancestors().collect::<Vec<_>>().iter().rev().skip(1) {
                                if board.board.check() != Value::None {
                                    // println!(
                                    //     "{:?}",
                                    //     node.ancestors().collect::<Vec<_>>().iter().rev().skip(1)
                                    // )
                                }
                                board.play(&x.value().play).unwrap();
                            }
                            if !node.value().play.is_empty() {
                                board.play(&node.value().play).unwrap();
                            }
                            let repr = board.board.dbg_repr();

                            let done = match board.board.check() {
                                Value::None => " ".to_owned(),
                                Value::Draw => "Draw".to_owned(),
                                Value::Player1 => "Crosses".to_owned(),
                                Value::Player2 => "Noughts".to_owned(),
                            };

                            let score = {
                                if node.id() == noughbert.tree.root().id() {
                                    format!(
                                        "{} / {} = {}",
                                        node.value().score(!mc_options.opt_for),
                                        node.value().playouts,
                                        if node.id() == noughbert.tree.root().id() {
                                            String::new()
                                        } else {
                                            (node.value().score(!mc_options.opt_for)
                                                / node.value().playouts)
                                                .to_string()
                                        },
                                    )
                                } else {
                                    format!(
                                        "{} / {} = {}",
                                        node.value().score(mc_options.opt_for),
                                        node.value().playouts,
                                        if node.id() == noughbert.tree.root().id() {
                                            String::new()
                                        } else {
                                            (node.value().score(mc_options.opt_for)
                                                / node.value().playouts)
                                                .to_string()
                                        },
                                    )
                                }
                            };

                            let ucb1 = {
                                if node.id() == noughbert.tree.root().id() {
                                    String::new()
                                } else {
                                    format!(
                                        "{}",
                                        node.value().ucb1(
                                            mc_options.exploration_factor,
                                            match node.parent() {
                                                Some(n) => n.value().playouts,
                                                None => node.value().playouts,
                                            },
                                            mc_options.opt_for
                                        )
                                    )
                                }
                            };

                            format!("{id}\n{play}\n{repr}\n{done}\n{score}\nucb1 = {ucb1}")
                        }
                    )
                );
                s.replace("\"]", "\" fontname = \"Consolas\"]")
            });

            // If needed, automatically compile the `.dot` files to `.svg` files
            if AUTOCOMPILE_GRAPHVIS_FILES {
                let _ = Command::new("dot")
                    .args(["-T", "svg", "-O", &format!("./outs/{runs}.dot")])
                    .spawn();
            }
        }
    }
}
