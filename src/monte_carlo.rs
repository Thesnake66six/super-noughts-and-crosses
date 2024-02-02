use std::time::Duration;

use ego_tree::{NodeId, Tree};

use crate::{
    cell::{Cell, Value},
    game::{Game, Turn},
};

use graphvis_ego_tree::TreeWrapper;

use self::node::MonteCarloNode;

mod node;

pub enum MonteCarloPolicy {
    Robust,
    Maximum,
    Frail,
    Minimum,
    Random,
    Questionable,
}

pub enum Message {
    GetMove(MonteCarloSettings),
    Interrupt,
}

/// A struct to govern the settings of the AI
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
    pub fn new(g: Game) -> MonteCarloManager {
        let moves_count = &g.legal_moves().len();
        MonteCarloManager {
            g,
            tree: ego_tree::Tree::new(MonteCarloNode::new(vec![], *moves_count)),
            sims: 0,
        }
    }

    pub fn select(&mut self, exploration_factor: f32) -> Option<NodeId> {
        let mut current_node_id = self.tree.root().id();
        // print_ego_tree(&self.tree, |x| format!("{:?}", x.value().play));
        let mut plays = 0;
        let mut current_node = self.tree.get(current_node_id).unwrap();
        loop {
            let chn = current_node.children();
            let mut best_node_id: Vec<NodeId> = vec![];
            let mut best_score = 0.0;
            let moves = self.g.legal_moves().len();

            assert_eq!(moves, current_node.value().child_count);

            if current_node.children().count() < moves || current_node.value().child_count == 0 {
                break;
            }
            for node in chn {
                let val = node.value();
                let ucb1 = val.ucb1(exploration_factor, current_node.value().playouts);
                if ucb1 == best_score {
                    best_node_id.insert(best_node_id.len(), node.id());
                } else if ucb1 > best_score {
                    best_node_id = vec![node.id()];
                    best_score = ucb1
                    
                }
            }

            current_node_id = fastrand::choice(best_node_id).unwrap();
            current_node = self.tree.get(current_node_id).unwrap();

            self.g.play(&current_node.value().play).unwrap();
            plays += 1;
        }
        for _ in 0..plays {
            self.g.unplay().unwrap()
        }

        Some(current_node_id)
    }

    pub fn expand(&mut self, node_id: NodeId) -> NodeId {
        let node = self.tree.get(node_id).unwrap();

        let mut count = 0;

        for x in node.ancestors().collect::<Vec<_>>().iter().rev().skip(1) {
            self.g.play(&x.value().play).unwrap();
            count += 1;
        }

        if node.value().play != vec![] {
            self.g.play(&node.value().play).unwrap();
            count += 1;
        }

        let mut moves = self.g.legal_moves();
        
        moves.retain(|x| !node.children().any(|a| &a.value().play == x));

        if moves.is_empty() {
            for _ in 0..count {
                self.g.unplay().unwrap()
            }
            return node_id;
        }
        
        let play = fastrand::choice(moves).unwrap();
        
        self.g.play(&play).unwrap();
        count += 1;
        
        let complete = self.g.board.check() != Value::None;
        let moves_count = self.g.legal_moves().len();
        
        for _ in 0..count {
            self.g.unplay().unwrap()
        }

        let mut node_mut = self.tree.get_mut(node_id).unwrap();

        let out = node_mut
            .append(MonteCarloNode {
                play,
                playouts: 0.0,
                wins: 0.0,
                child_count: moves_count,
                complete,
            })
            .id();

        let node = self.tree.get(out).unwrap();

        if complete {
            for x in node
                .ancestors()
                .map(|x| x.id())
                .collect::<Vec<_>>()
                .iter()
            {
                let c = &self.update_complete(*x);
                if !c {
                    break;
                }
            }
        }
        out
    }

    pub fn simulate(&mut self, node_id: NodeId, opt_for: Turn) -> (NodeId, f32) {
        let node = self.tree.get(node_id).unwrap();

        let mut count = 0;
        for x in node.ancestors().collect::<Vec<_>>().iter().rev() {
            let x = &x.value();
            if x.play.is_empty() {
                continue;
            }
            self.g.play(&x.play).expect("Failed");
            count += 1;
        }

        while self.g.board.check() == Value::None {
            let _ = self
                .g
                .play(fastrand::choice(self.g.legal_moves().iter()).unwrap());
            count += 1;
        }

        let mut node_mut = self.tree.get_mut(node_id).unwrap();

        node_mut.value().playouts += 1.0;
        let val = if self.g.board.check() == opt_for.val() {
            1.0
        } else {
            0.0
        };
        node_mut.value().wins += val;

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
            anode.value().wins += val;
        }
    }

    pub fn best(&self, policy: MonteCarloPolicy) -> Vec<usize> {
        match policy {
            MonteCarloPolicy::Robust => {
                let node = self.tree.root();
                let mut best_score = 0.0;
                let mut best_id = node.id();

                for child in node.children().map(|x| x.id()) {
                    let cnode = self.tree.get(child).unwrap();
                    if cnode.value().playouts > best_score {
                        best_score = cnode.value().playouts;
                        best_id = cnode.id();
                    }
                }

                self.tree.get(best_id).unwrap().value().play.clone()
            }
            MonteCarloPolicy::Maximum => {
                let node = self.tree.root();
                let mut best_score = 0.0;
                let mut best_id = node.id();

                for child in node.children().map(|x| x.id()) {
                    let cnode = self.tree.get(child).unwrap();
                    if cnode.value().wins >= best_score {
                        best_score = cnode.value().wins;
                        best_id = cnode.id();
                    }
                }

                self.tree.get(best_id).unwrap().value().play.clone()
            }
            MonteCarloPolicy::Frail => todo!(),
            MonteCarloPolicy::Minimum => todo!(),
            MonteCarloPolicy::Random => todo!(),
            MonteCarloPolicy::Questionable => todo!(),
        }
    }

    pub fn update_complete(&mut self, node_id: NodeId) -> bool {
        let node = self.tree.get(node_id).unwrap();
        let mut complete = true;

        if node.value().complete {
            return true;
        }
        let chn = node.children().map(|x| x.id()).count();

        if chn < node.value().child_count {
            return false
        }

        for child in node.children().map(|x| x.id()) {
            let cnode = self.tree.get(child).unwrap();
            if !cnode.value().complete {
                complete = false;
            }
        }

        let mut node = self.tree.get_mut(node_id).unwrap();
        node.value().complete = complete;
        complete
    }
}
