use std::{fs, path::Path, process::Command};

use ego_tree::Tree;

use crate::{
    game::{
        game::{Game, Turn},
        value::Value,
    },
    styles::AUTOCOMPILE_GRAPHVIS_FILES,
};

use super::monte_carlo_node::MonteCarloNode;

pub fn output_graphvis_files(
    tree: &Tree<MonteCarloNode>,
    game: &Game,
    name: &str,
    exp: f32,
    opt_for: Turn,
) {
    eprintln!("Printing {name} to file");
    let _ = fs::write(Path::new(&format!("./outs/{name}.dot")), {
        let s = format!(
            "{}",
            graphvis_ego_tree::TreeWrapper::new(tree, tree.root().id(), |node| {
                let mut board = game.clone();
                let id = format!("Node ID: {:?}", node.id());
                let play = {
                    if node.id() == tree.root().id() {
                        format!(
                            "Starting Board: {}",
                            match game.turn {
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
                    if node.id() == tree.root().id() {
                        format!(
                            "{} / {} = {}",
                            node.value().score(!opt_for),
                            node.value().playouts,
                            if node.id() == tree.root().id() {
                                String::new()
                            } else {
                                (node.value().score(!opt_for) / node.value().playouts).to_string()
                            },
                        )
                    } else {
                        format!(
                            "{} / {} = {}",
                            node.value().score(opt_for),
                            node.value().playouts,
                            if node.id() == tree.root().id() {
                                String::new()
                            } else {
                                (node.value().score(opt_for) / node.value().playouts).to_string()
                            },
                        )
                    }
                };

                let ucb1 = {
                    if node.id() == tree.root().id() {
                        String::new()
                    } else {
                        format!(
                            "{}",
                            node.value().ucb1(
                                exp,
                                match node.parent() {
                                    Some(n) => n.value().playouts,
                                    None => node.value().playouts,
                                },
                                opt_for
                            )
                        )
                    }
                };

                format!("{id}\n{play}\n{repr}\n{done}\n{score}\nucb1 = {ucb1}")
            })
        );
        s.replace("\"]", "\" fontname = \"Consolas\"]")
    });

    // If needed, automatically compile the `.dot` files to `.svg` files
    if AUTOCOMPILE_GRAPHVIS_FILES {
        let _ = Command::new("dot")
            .args(["-T", "svg", "-O", &format!("./outs/{name}.dot")])
            .spawn();
    }
}
