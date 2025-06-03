use core::panic;

// use ego_tree::{NodeId, Tree};
use id_tree::{Node, NodeId, Tree, TreeBuilder};

use crate::{common::Move, game::{
    game::{Game, Turn},
    value::Value,
}};

use super::{monte_carlo_node::MonteCarloNode, monte_carlo_policy::MonteCarloPolicy};

#[derive(Debug)]
/// The Monte Carlo manager struct
pub struct MonteCarloManager {
    /// The game that is being evaluated
    pub g: Game,
    /// The game tree
    pub tree: Tree<MonteCarloNode>,
    /// The number of simulations that have been run
    pub sims: usize,
    /// The number of simulations that have been started
    pub sims_requested: usize,
}

impl MonteCarloManager {
    /// Constructor function
    pub fn new(g: Game, t: Turn) -> MonteCarloManager {
        let moves_count = &g.legal_moves().len();
        MonteCarloManager {
            g,
            // tree: Tree::new(MonteCarloNode::new(vec![], *moves_count, !t)),
            tree: TreeBuilder::new().with_root(Node::new(MonteCarloNode::new(vec![], *moves_count, !t))).build(),
            sims: 0,
            sims_requested: 0,
        }
    }

    /// Selects the next move for simulation
    pub fn select(&mut self, exploration_factor: f32, opt_for: Turn) -> Option<&NodeId> {
        let mut plays = 0;
        let mut current_node_id = self.tree.root_node_id().unwrap();
        let mut current_node = self.tree.get(current_node_id).unwrap();

        loop {
            let chn = current_node.children();
            let moves = self.g.legal_moves().len();

            // Break if a terminal node or node that has not been fully expanded is selected
            if current_node.children().len() < moves
                || current_node.data().child_count == 0
                || self.g.board.check() != Value::None
            {
                break;
            }

            let mut best_node_ids: Vec<&NodeId> = vec![];
            let mut best_score = f32::MIN;

            // Calculate the child(ren) with the highest UCB1 value
            for node_id in chn {
                let node = self.tree.get(node_id).unwrap();
                let val = node.data();
                let ucb1 = val.ucb1(exploration_factor, current_node.data().playouts, opt_for);
                if ucb1 > best_score {
                    best_node_ids = vec![node_id];
                    best_score = ucb1;
                } else if ucb1 == best_score {
                    best_node_ids.insert(best_node_ids.len(), node_id);
                } else if ucb1 <= f32::MIN {
                    eprintln!("Node {:?} has UCB1 of {ucb1}", node_id)
                }
            }

            // Choose a random next best node
            current_node_id = fastrand::choice(best_node_ids).unwrap(); // This panics if node has no children with UCB1 higher than f32::MIN
            current_node = self.tree.get(current_node_id).unwrap();

            match self.g.play(&current_node.data().play) {
                Ok(()) => plays += 1,
                Err(_) => panic!(),
            }
        }

        // Unplay each move made
        for _ in 0..plays {
            match self.g.unplay() {
                Ok(()) => {}
                Err(_) => panic!(),
            }
        }

        // Return the selected node
        Some(current_node_id)
    }

    /// Adds a child node (where applicable) to the selected node
    pub fn expand(&mut self, node_id: &NodeId) -> NodeId {
        let node = self.tree.get(node_id).unwrap();

        let mut count = 0;

        // Play each move preceding the selected node
        for x in self.tree.ancestors(node_id).unwrap().collect::<Vec<_>>().iter().rev().skip(1) {
            match self.g.play(&x.data().play) {
                Ok(()) => count += 1,
                Err(_) => panic!(),
            }
        }

        // Play the move of the selected node
        if !node.data().play.is_empty() {
            match self.g.play(&node.data().play) {
                Ok(()) => count += 1,
                Err(_) => panic!(),
            }
        }

        // Filter off each move that has not yet been expanded
        let mut moves = self.g.legal_moves();
        moves.retain(|x| !node.children().iter().any(|a| &self.tree.get(a).unwrap().data().play == x));

        // If all moves are expanded, or the node is terminal, return the node
        if moves.is_empty() || self.g.board.check() != Value::None {
            for _ in 0..count {
                match self.g.unplay() {
                    Ok(()) => {}
                    Err(_) => panic!(),
                }
            }
            return node_id.clone();
        }

        // Choose a random remaining move and play it
        let play = fastrand::choice(moves).unwrap();
        self.g.play(&play).unwrap();
        count += 1;

        let moves_count = self.g.legal_moves().len();

        for _ in 0..count {
            self.g.unplay().unwrap();
        }

        let new_turn = !self.tree.get(node_id).unwrap().data().turn;
        let mut node_mut = self.tree.get_mut(node_id).unwrap();

        // Append the new child and return it
        // let out = node_mut
        //     .append(MonteCarloNode {
        //         play,
        //         playouts: 0.0,
        //         score: 0.0,
        //         child_count: moves_count,
        //         turn: new_turn,
        //     })
        //     .id();

        self.tree.insert(Node::new(MonteCarloNode {
                play,
                playouts: 0.0,
                score: 0.0,
                child_count: moves_count,
                turn: new_turn,
            }), id_tree::InsertBehavior::UnderNode(node_id)
        ).unwrap()
    }

    /// Runs a playout on the selected node
    pub fn simulate<'a>(&mut self, node_id: &'a NodeId, opt_for: Turn) -> (&'a NodeId, f32) {
        let node = self.tree.get(&node_id).unwrap();

        // Play each move preceding the selected node
        let mut count = 0;
        for x in self.tree.ancestors(&node_id).unwrap().collect::<Vec<_>>().iter().rev() {
            if !x.data().play.is_empty() {
                let x = &x.data();
                match self.g.play(&x.play) {
                    Ok(()) => count += 1,
                    Err(_) => panic!(),
                }
            }
        }

        // Play the move of the selected node
        match self.g.play(&node.data().play) {
            Ok(()) => count += 1,
            Err(_) => panic!(),
        }

        // Repeatedly play moves until a terminal state is reached
        while self.g.board.check() == Value::None {
            self.g
                .play(fastrand::choice(self.g.legal_moves().iter()).unwrap())
                .unwrap();
            count += 1;
        }

        let mut node_mut = self.tree.get_mut(&node_id).unwrap();

        // Adjust the value of the node based on the playout result

        let val = if self.g.board.check() == opt_for.val() {
            1.0
        } else if self.g.board.check() == Value::Draw {
            0.0
        } else {
            -1.0
        };

        // Unplay all moves made
        for _ in 0..count {
            self.g.unplay().unwrap();
        }

        // Return the node, and the simulation result
        (node_id, val)
    }

    /// Propagates the value up the tree
    pub fn backpropogate_value(&mut self, node_id: &NodeId, val: f32) {
        // Apply result to the leaf node
        let node_mut = self.tree.get_mut(node_id).unwrap();
        node_mut.data_mut().score += val;

        // Loop over each parent node of the selected node
        let ancestors: Vec<_> = self.tree.ancestor_ids(node_id).unwrap().cloned().collect();
        for ancestor in ancestors.iter().rev() {
            // Adjust the value of the parent node
            let anode = self.tree.get_mut(ancestor).unwrap();
            anode.data_mut().score += val;
        }
    }

    /// Propagates the value up the tree
    pub fn backpropogate_playouts(&mut self, node_id: &NodeId, val: f32) {
        // Apply result to the leaf node

        let node_mut = self.tree.get_mut(node_id).unwrap();
        node_mut.data_mut().playouts += val;

        // Loop over each parent node of the selected node
        let ancestors: Vec<_> = self.tree.ancestor_ids(node_id).unwrap().cloned().collect();
        for ancestor in ancestors.iter().rev() {
            // Adjust the value of the parent node
            let anode = self.tree.get_mut(ancestor).unwrap();
            anode.data_mut().playouts += val;
        }
    }

    /// Return the best move based on the selected policy
    pub fn best(
        &mut self,
        policy: MonteCarloPolicy,
        opt_for: Turn,
        exploration_factor: f32,
    ) -> Option<Move> {
        match policy {
            MonteCarloPolicy::Robust => {
                let node = self.tree.root_node_id().unwrap();
                let mut best_score = 0.0;
                let mut best_id = None;

                for child in self.tree.children_ids(node).unwrap() {
                    let cnode = self.tree.get(child).unwrap();
                    if cnode.data().playouts > best_score {
                        best_score = cnode.data().playouts;
                        best_id = Some(child);
                    }
                }

                if let Some(id) = best_id {
                    Some(self.tree.get(id).unwrap().data().play.clone())
                } else {
                    fastrand::choice(self.g.legal_moves())
                }
            }
            MonteCarloPolicy::Maximum => {
                let node = self.tree.root_node_id().unwrap();
                let mut best_score = 0.0;
                let mut best_id = None;

                for child in self.tree.children_ids(node).unwrap() {
                    let cnode = self.tree.get(child).unwrap();
                    if cnode.data().score(opt_for) >= best_score {
                        best_score = cnode.data().score(opt_for);
                        best_id = Some(child);
                    }
                }

                if let Some(id) = best_id {
                    Some(self.tree.get(id).unwrap().data().play.clone())
                } else {
                    fastrand::choice(self.g.legal_moves())
                }
            }
            MonteCarloPolicy::Frail => unimplemented!(),
            MonteCarloPolicy::Minimum => unimplemented!(),
            MonteCarloPolicy::Random => unimplemented!(),
            // Don't use ever
            MonteCarloPolicy::UCB1 => {
                let node = self.tree.root_node_id().unwrap();
                let mut best_ucb1 = 0.0;
                let mut best_id = None;

                for child in self.tree.children_ids(node).unwrap() {
                    let cnode = self.tree.get(child).unwrap();
                    let ucb1 = cnode.data().score(opt_for) / cnode.data().playouts;
                    if ucb1 >= best_ucb1 {
                        best_ucb1 = ucb1;
                        best_id = Some(child);
                    }
                }

                if let Some(id) = best_id {
                    Some(self.tree.get(id).unwrap().data().play.clone())
                } else {
                    None
                }
            }
        }
    }
}
