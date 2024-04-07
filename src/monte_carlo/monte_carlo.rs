use core::panic;
use std::{fmt::Debug, time::Duration};

use ego_tree::{NodeId, Tree};

use crate::{
    cell::Value,
    game::{Game, Turn},
};

use super::node::MonteCarloNode;

#[derive(Debug, Clone, Copy)]
pub enum MonteCarloPolicy {
    Robust,
    Maximum,
    Frail,
    Minimum,
    Random,
    UCB1,
}

/// A struct to govern the settings of the AI
#[derive(Debug, Clone)]
pub struct MonteCarloSettings {
    pub game: Game,
    pub timeout: Duration,
    pub max_sims: usize,
    pub exploration_factor: f32,
    pub opt_for: Turn,
    pub carry_forward: bool,
    pub policy: MonteCarloPolicy,
}

#[derive(Debug)]
pub struct MonteCarloManager {
    pub g: Game,
    pub tree: Tree<MonteCarloNode>,
    pub sims: usize,
}

impl MonteCarloManager {
    pub fn new(g: Game, t: Turn) -> MonteCarloManager {
        let moves_count = &g.legal_moves().len();
        MonteCarloManager {
            g,
            tree: ego_tree::Tree::new(MonteCarloNode::new(vec![], *moves_count, !t)),
            sims: 0,
        }
    }

    pub fn select(&mut self, exploration_factor: f32, opt_for: Turn) -> Option<NodeId> {
        // print_ego_tree(&self.tree, |x| format!("{:?}", x.value().play));
        let mut plays = 0;
        let mut current_node_id = self.tree.root().id();
        let mut current_node = self.tree.get(current_node_id).unwrap();

        loop {
            let chn = current_node.children();
            let moves = self.g.legal_moves().len();

            if current_node.children().count() < moves
                || current_node.value().child_count == 0
                || self.g.board.check() != Value::None
            {
                break;
            }

            let mut best_node_ids: Vec<NodeId> = vec![];
            let mut best_score = f32::MIN;

            // assert_eq!(moves, current_node.value().child_count);

            for node in chn {
                let val = node.value();
                let ucb1 = val.ucb1(exploration_factor, current_node.value().playouts, opt_for);
                if ucb1 > best_score {
                    best_node_ids = vec![node.id()];
                    best_score = ucb1
                } else if ucb1 == best_score {
                    best_node_ids.insert(best_node_ids.len(), node.id());
                }
            }

            current_node_id = fastrand::choice(best_node_ids).unwrap();
            current_node = self.tree.get(current_node_id).unwrap();

            match self.g.play(&current_node.value().play) {
                Ok(_) => plays += 1,
                Err(_) => panic!(),
            }

            // eprintln!("Select Loop:");
            // eprintln!("Node {:?}", current_node_id);
            // eprintln!("Player: {:#?}, val: [N/A]", opt_for);
            // eprintln!("{:?}", self.g.board.check());
            // eprintln!("{}", self.g.board.dbg_repr());
        }
        for _ in 0..plays {
            match self.g.unplay() {
                Ok(_) => {}
                Err(_) => panic!(),
            }
        }

        Some(current_node_id)
    }

    pub fn expand(&mut self, node_id: NodeId) -> NodeId {
        let node = self.tree.get(node_id).unwrap();

        let mut count = 0;

        for x in node.ancestors().collect::<Vec<_>>().iter().rev().skip(1) {
            match self.g.play(&x.value().play) {
                Ok(_) => count += 1,
                Err(_) => panic!(),
            }
        }

        if !node.value().play.is_empty() {
            match self.g.play(&node.value().play) {
                Ok(_) => count += 1,
                Err(_) => panic!(),
            }
        }

        let mut moves = self.g.legal_moves();
        moves.retain(|x| !node.children().any(|a| &a.value().play == x));

        // eprintln!("Expansion:");
        // eprintln!("Node {:?}", node_id);
        // eprintln!("Player: {:#?}, val: [N/A]", opt_for);
        // eprintln!("Moves: {:?}", moves);
        // eprintln!("{:?}", self.g.board.check());
        // eprintln!("{}", self.g.board.dbg_repr());

        if moves.is_empty() || self.g.board.check() != Value::None {
            for _ in 0..count {
                match self.g.unplay() {
                    Ok(_) => {}
                    Err(_) => panic!(),
                }
            }
            return node_id;
        }

        let play = fastrand::choice(moves).unwrap();

        self.g.play(&play).unwrap();
        count += 1;

        let moves_count = self.g.legal_moves().len();

        for _ in 0..count {
            self.g.unplay().unwrap()
        }

        let new_turn = !self.tree.get(node_id).unwrap().value().turn;
        let mut node_mut = self.tree.get_mut(node_id).unwrap();

        let out = node_mut
            .append(MonteCarloNode {
                play,
                playouts: 0.0,
                score: 0.0,
                child_count: moves_count,
                turn: new_turn,
            })
            .id();

        out
    }

    pub fn simulate(&mut self, node_id: NodeId, opt_for: Turn) -> (NodeId, f32) {
        let node = self.tree.get(node_id).unwrap();

        let mut count = 0;
        for x in node.ancestors().collect::<Vec<_>>().iter().rev() {
            if !x.value().play.is_empty() {
                let x = &x.value();
                match self.g.play(&x.play) {
                    Ok(_) => count += 1,
                    Err(_) => panic!(),
                }
            }
        }

        match self.g.play(&node.value().play) {
            Ok(_) => count += 1,
            Err(_) => panic!(),
        }

        // eprintln!("Simulation (before loop):");
        // eprintln!("Node {:?}", node_id);
        // eprintln!("Player: {:#?}, val: [N/A]", opt_for);
        // eprintln!("{:?}", self.g.board.check());
        // eprintln!("{}", self.g.board.dbg_repr());

        while self.g.board.check() == Value::None {
            self.g
                .play(fastrand::choice(self.g.legal_moves().iter()).unwrap())
                .unwrap();
            count += 1;
            // eprintln!("Simulation (in loop):");
            // eprintln!("Node {:?}", node_id);
            // eprintln!("Player: {:#?}, val: [N/A]", opt_for);
            // eprintln!("{:?}", self.g.board.check());
            // eprintln!("{}", self.g.board.dbg_repr());
        }

        assert_ne!(self.g.board.check(), Value::None);

        let mut node_mut = self.tree.get_mut(node_id).unwrap();

        node_mut.value().playouts += 1.0;

        let val = if self.g.board.check() == opt_for.val() {
            1.0
        } else if self.g.board.check() == Value::Draw {
            0.0
        } else {
            -1.0
        };
        node_mut.value().score += val;

        // eprintln!("Simulation (after loop):");
        // eprintln!("Node {:?}", node_id);
        // eprintln!("Player: {:#?}, val: {}", opt_for, val);
        // eprintln!("{:?}", self.g.board.check());
        // eprintln!("{}", self.g.board.dbg_repr());

        for _ in 0..count {
            self.g.unplay().unwrap()
        }

        (node_mut.id(), val)
    }

    pub fn backpropogate(&mut self, node_id: NodeId, val: f32) {
        let node = self.tree.get(node_id).unwrap();

        for ancestor in node
            .ancestors()
            .map(|x| x.id())
            .collect::<Vec<_>>()
            .iter()
            .rev()
        {
            let mut anode = self.tree.get_mut(*ancestor).unwrap();
            anode.value().playouts += 1.0;
            anode.value().score += val;
        }
    }

    pub fn best(
        &mut self,
        policy: MonteCarloPolicy,
        opt_for: Turn,
        exploration_factor: f32,
    ) -> Option<Vec<usize>> {
        match policy {
            MonteCarloPolicy::Robust => {
                let node = self.tree.root();
                let mut best_score = 0.0;
                let mut best_id = None;

                for child in node.children().map(|x| x.id()) {
                    let cnode = self.tree.get(child).unwrap();
                    if cnode.value().playouts > best_score {
                        best_score = cnode.value().playouts;
                        best_id = Some(cnode.id());
                    }
                }

                if let Some(id) = best_id {
                    Some(self.tree.get(id).unwrap().value().play.clone())
                } else {
                    fastrand::choice(self.g.legal_moves())
                }
            }
            MonteCarloPolicy::Maximum => {
                let node = self.tree.root();
                let mut best_score = 0.0;
                let mut best_id = None;

                for child in node.children().map(|x| x.id()) {
                    let cnode = self.tree.get(child).unwrap();
                    if cnode.value().score(opt_for) >= best_score {
                        best_score = cnode.value().score(opt_for);
                        best_id = Some(cnode.id());
                    }
                }

                if let Some(id) = best_id {
                    Some(self.tree.get(id).unwrap().value().play.clone())
                } else {
                    None
                }
            }
            MonteCarloPolicy::Frail => todo!(),
            MonteCarloPolicy::Minimum => todo!(),
            MonteCarloPolicy::Random => todo!(),
            // Don't use ever
            MonteCarloPolicy::UCB1 => {
                let node = self.tree.root();
                let mut best_ucb1 = 0.0;
                let mut best_id = None;

                for child in node.children().map(|x| x.id()) {
                    let cnode = self.tree.get(child).unwrap();
                    let ucb1 = cnode.value().score(opt_for) / cnode.value().playouts;
                    if ucb1 >= best_ucb1 {
                        best_ucb1 = ucb1;
                        best_id = Some(cnode.id());
                    }
                }

                if let Some(id) = best_id {
                    Some(self.tree.get(id).unwrap().value().play.clone())
                } else {
                    None
                }
            }
        }
    }
}
