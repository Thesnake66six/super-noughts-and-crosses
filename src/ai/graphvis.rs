use std::{fmt::{self, Display}, fs, hash::{DefaultHasher, Hash, Hasher}, mem, path::Path, process::Command, ptr::hash};

use id_tree::{Node, NodeId, Tree};

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
            TreeWrapper::new(tree, tree.root_node_id().unwrap(), |tree, node_id| {
                let node = tree.get(node_id).unwrap();
                let node_data = node.data();
                let mut board = game.clone();
                let id: (usize, usize, usize) = unsafe { mem::transmute(node_id.clone()) };
                let id = id.2;
                let id = format!("Node ID: {:?}", id);
                let play = {
                    if node_id == tree.root_node_id().unwrap() {
                        format!(
                            "Starting Board: {}",
                            match game.turn {
                                Turn::Player1 => "Crosses".to_owned(),
                                Turn::Player2 => "Noughts".to_owned(),
                            }
                        )
                    } else {
                        match node_data.turn {
                            Turn::Player1 => {
                                format!("Crosses' turn: {:?}", &node_data.play)
                            }
                            Turn::Player2 => {
                                format!("Noughts' turn: {:?}", &node_data.play)
                            }
                        }
                    }
                };

                for x in tree.ancestors(node_id).unwrap().collect::<Vec<_>>().iter().rev().skip(1) {
                    // if board.board.check() != Value::None {
                    //     println!(
                    //         "{:?}",
                    //         node.ancestors().collect::<Vec<_>>().iter().rev().skip(1)
                    //     )
                    // }
                    board.play(&x.data().play).unwrap();
                }

                if !node_data.play.is_empty() {
                    board.play(&node_data.play).unwrap();
                }
                let repr = board.board.dbg_repr();

                let done = match board.board.check() {
                    Value::None => " ".to_owned(),
                    Value::Draw => "Draw".to_owned(),
                    Value::Player1 => "Crosses".to_owned(),
                    Value::Player2 => "Noughts".to_owned(),
                };

                let score = {
                    if node_id == tree.root_node_id().unwrap() {
                        format!(
                            "{} / {} = {}",
                            node_data.score(!opt_for),
                            node_data.playouts,
                            if node_id == tree.root_node_id().unwrap() {
                                String::new()
                            } else {
                                (node_data.score(!opt_for) / node_data.playouts).to_string()
                            },
                        )
                    } else {
                        format!(
                            "{} / {} = {}",
                            node_data.score(opt_for),
                            node_data.playouts,
                            if node_id == tree.root_node_id().unwrap() {
                                String::new()
                            } else {
                                (node_data.score(opt_for) / node_data.playouts).to_string()
                            },
                        )
                    }
                };

                let ucb1 = {
                    if node_id == tree.root_node_id().unwrap() {
                        String::new()
                    } else {
                        format!(
                            "{}",
                            node_data.ucb1(
                                exp,
                                match node.parent() {
                                    Some(n) => tree.get(n).unwrap().data().playouts,
                                    None => node_data.playouts,
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

pub struct TreeWrapper<'a, T, F> {
    tree: &'a Tree<T>,
    root: &'a NodeId,
    f: F,
}

impl<'a, T, F> TreeWrapper<'a, T, F>
where
    F: Fn(&Tree<T>, &NodeId) -> String,
{
    /// Creates a new TreeWrapper
    pub fn new(tree: &'a Tree<T>, root: &'a NodeId, f: F) -> Self {
        Self { tree, f, root }
    }
}

impl<'a, T, F: Fn(&Tree<T>, &NodeId) -> String> Display for TreeWrapper<'a, T, F> {
    fn fmt(&self, w: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(w, "digraph {{")?;
        writeln!(w, "node [shape=box]")?;
        for node in self.tree.traverse_post_order_ids(self.tree.root_node_id().unwrap()).unwrap() {
            let mut hasher = DefaultHasher::new();
            node.hash(&mut hasher);
            let id = hasher.finish();

            writeln!(
                w,
                "N{id} [ label = \"{}\"]",
                (self.f)(self.tree, &node)
                    .replace('\\', r#"\\"#)
                    .replace('\n', r#"\n"#)
                    .replace('\t', r#"\t"#)
                    .replace('\r', r#"\r"#)
                    .replace('\'', r#"\'"#)
                    .replace('"', r#"\""#)
            )?;
            for child in self.tree.children_ids(&node).unwrap() {
                let mut hasher = DefaultHasher::new();
                child.hash(&mut hasher);
                let c_id = hasher.finish();
                writeln!(w, "N{id} -> N{c_id}")?;
            }
            
        }
        writeln!(w, "}}")?;

        Ok(())
    }
}
