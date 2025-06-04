use std::{
    collections::HashMap,
    fs,
    sync::mpsc::{self, sync_channel},
    thread, time,
};

use id_tree::NodeId;

use crate::{
    ai::{
        graphvis::output_graphvis_files, monte_carlo::MonteCarloManager, noughbert_message::NoughbertMessage, simulation_thread::simulation_thread, thoughts::Thoughts
    },
    game::value::Value,
    styles::{GraphvisOutputLevel, OUTPUT_GRAPHVIS_FILES},
};

use super::{comms::Comms, noughbert_message::ExplorationRequest};

pub fn noughbert(main: Comms<NoughbertMessage>) {
    // Count the number of AI simulations
    let mut runs = 0;
    // let mut graphviz_prints = 0;

    // Clear and re-create the `./outs` folder
    if OUTPUT_GRAPHVIS_FILES != GraphvisOutputLevel::None {
        let _ = fs::remove_dir_all("./outs");
        let _ = fs::create_dir("./outs");
    }

    loop {
        // Recieve all messages, if a `Message::Start()` is recieved, begin simulation
        let message = main.recv().unwrap();
        let mc_options = match message {
            NoughbertMessage::Start(x) => x,
            NoughbertMessage::Interrupt => continue,
            NoughbertMessage::GetThoughts(_) => continue,
            NoughbertMessage::Thoughts(_) => continue,
            NoughbertMessage::Move(_) => continue,
            NoughbertMessage::Return() => continue,
        };

        println!("Simulation requested");

        let mut noughbert = MonteCarloManager::new(mc_options.game, mc_options.opt_for);
        let start_time = time::Instant::now();
        let mut threads: HashMap<usize, (NodeId, Comms<ExplorationRequest>)> = HashMap::new();
        let mut channel_counter = 10;
        let mut interrupt = false;
        let mut interrupt_return = true;
        let mut prints_this_run = 0;

        // Make sure a move is never requested on a completed board state
        // #TODO: Test code
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
            let message = main.try_recv();
            match message {
                Ok(m) => match m {
                    NoughbertMessage::Start(_) => {}
                    NoughbertMessage::Interrupt => {
                        interrupt = true;
                        break;
                    }
                    NoughbertMessage::GetThoughts(t) => {
                        let root = noughbert.tree.get(noughbert.tree.root_node_id().unwrap()).unwrap().data();
                        main.send(NoughbertMessage::Thoughts(Thoughts {
                            sims: noughbert.sims,
                            score: root.score(t),
                        }))
                        .unwrap();
                    }
                    NoughbertMessage::Thoughts(_) => {}
                    NoughbertMessage::Move(_) => {}
                    NoughbertMessage::Return() => {
                        interrupt_return = true;
                        break;
                    }
                },
                Err(e) => match e {
                    mpsc::TryRecvError::Empty => {}
                    mpsc::TryRecvError::Disconnected => panic!("Thread disconnected"),
                },
            }

            let mut polled_ids = Vec::new();
            // Poll all active threads
            for thread in threads.iter() {
                let id = &thread.1 .0;
                // eprintln!("Polling thread {:?}", id);
                // let mut node_mut = noughbert.tree.get_mut(id).unwrap();
                // if node_mut.value().playouts < 1.0 {
                //   eprintln!("Node {:?} has {} playouts!", node_mut.id(), node_mut.value().playouts)
                // }
                let tcomms = &thread.1 .1;

                match tcomms.try_recv() {
                    Ok(v) => match v {
                        ExplorationRequest::Stop => {}
                        ExplorationRequest::Return { result: v } => {
                            // eprintln!("Thread {:?} returned with value {}, bringing completed sims to {}", id, v, noughbert.sims + 1);
                            // node_mut.value().score += v;
                            noughbert.sims += 1;
                            noughbert.backpropogate_value(&id, v);
                            polled_ids.insert(polled_ids.len(), *thread.0);
                        }
                    },
                    Err(e) => match e {
                        mpsc::TryRecvError::Empty => {}
                        mpsc::TryRecvError::Disconnected => {
                            eprintln!("Thread crashed");
                            polled_ids.insert(polled_ids.len(), *thread.0);
                            // println!("Thread crashed")
                        }
                    },
                }
            }

            for id in polled_ids {
                threads.remove(&id);
            }

            if mc_options.threads == 1 {
                // Run the MCTS algorithm once
                let x = noughbert.select(mc_options.exploration_factor, mc_options.opt_for).cloned();
                if x.is_none() {
                    break;
                }
                let x = noughbert.expand(&x.unwrap());
                let (x, val) = noughbert.simulate(&x, mc_options.opt_for);
                noughbert.backpropogate_playouts(x, 1.0);
                noughbert.backpropogate_value(x, val);
                // Increment the number of simulations run
                noughbert.sims += 1;
            }

            // Fill the thread pool to the brim, else to the number of remaining sims needed
            let threads_to_spawn = (mc_options.threads - threads.len())
                .min(mc_options.max_sims - noughbert.sims_requested);
            // TODO: Should this be printed to the stderr in release?
            // if threads_to_spawn != 0 {
            //   eprintln!("Threads to spawn: Pool Space ({}) or Remaining Sims ({})", mc_options.threads - threads.len(), mc_options.max_sims - noughbert.sims_requested);
            //   eprintln!("Spawning {} threads this iteration:", threads_to_spawn);
            // }
            for _ in 0..threads_to_spawn {
                let (txi, rxi) = sync_channel::<ExplorationRequest>(channel_counter);
                let (txo, rxo) = sync_channel::<ExplorationRequest>(channel_counter + 1);
                // TODO: Should this really use wrapping add? I mean, if they port specifically this program to quantum chips...
                channel_counter = if channel_counter.wrapping_add(2) < 10 {
                    10
                } else {
                    channel_counter.wrapping_add(2)
                };

                let x = noughbert.select(mc_options.exploration_factor, mc_options.opt_for).cloned();
                if x.is_none() {
                    break;
                }
                let x = noughbert.expand(&x.unwrap());
                noughbert.backpropogate_playouts(&x, 1.0);
                // graphviz_prints += 1;
                prints_this_run += 1;
                let node = noughbert.tree.get(&x).unwrap();

                // Clone and prime the board based on the selected move
                let mut thread_game = noughbert.g.clone();
                for y in noughbert.tree.ancestors(&x).unwrap().collect::<Vec<_>>().iter().rev() {
                    if !y.data().play.is_empty() {
                        let y = &y.data();
                        thread_game.play(&y.play).unwrap();
                    }
                }
                thread_game.play(&node.data().play).unwrap();
                let thread_opt_for = mc_options.opt_for;

                // eprintln!(
                //   "Spawning thread on channel {} to run simulation {} on node {:?}",
                //   channel_counter.wrapping_sub(2),
                //   noughbert.sims_requested,
                //   x
                // );

                // Spawn the thread
                let x2 = x.clone();
                let _ = thread::Builder::new()
                    .name(format!("{:?}", &x))
                    .spawn(move || {
                        simulation_thread(Comms::new(rxo, txi), x.clone(), thread_game, thread_opt_for);
                    });

                threads.insert(channel_counter.wrapping_sub(2), (x2, Comms::new(rxi, txo)));

                noughbert.sims_requested += 1;
                if OUTPUT_GRAPHVIS_FILES == GraphvisOutputLevel::Full {
                    output_graphvis_files(
                        &noughbert.tree,
                        &noughbert.g,
                        &format!("Run{}Print{prints_this_run}", runs + 1),
                        mc_options.exploration_factor,
                        mc_options.opt_for,
                    );
                }
            }
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
        main.send(NoughbertMessage::Move(best_play)).unwrap();
        runs += 1;

        // If needed, output the node `.svg` files
        if OUTPUT_GRAPHVIS_FILES != GraphvisOutputLevel::None {
            output_graphvis_files(
                &noughbert.tree,
                &noughbert.g,
                &format!("Run{runs}Final"),
                mc_options.exploration_factor,
                mc_options.opt_for,
            );
        }
    }
}
