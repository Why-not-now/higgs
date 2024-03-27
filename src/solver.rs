use std::collections::HashMap;

use slotmap::Key;

use crate::{board::Board, tree::Tree};

pub struct Solver<K: Key> {
    tree: Tree<K, Board>,
    next_states: Vec<K>,
    board_to_key: HashMap<Board, K>,
}

impl<K: Key> Solver<K> {
    pub fn tree(&self) -> &Tree<K, Board> {
        &self.tree
    }

    pub fn next_states(&self) -> &[K] {
        &self.next_states
    }

    pub fn board_to_key(&self) -> &HashMap<Board, K> {
        &self.board_to_key
    }

    pub fn new(root: Board) -> Self {
        let tree = Tree::new(root.clone());
        let next_states = vec![tree.root()];
        let mut board_to_key = HashMap::new();
        board_to_key.insert(root, tree.root());

        Self {
            tree,
            next_states,
            board_to_key,
        }
    }

    pub fn solve(&mut self) -> Vec<K> {
        let mut solved = self.one_step();
        while !self.next_states.is_empty() && solved.is_empty() {
            solved = self.one_step();
        }

        solved
    }

    pub fn solve_n_steps(&mut self, steps: u32) -> Vec<K> {
        if steps == 0 {
            return Vec::new();
        }
        let mut solved = self.one_step();
        for _ in 0..steps - 1 {
            if self.next_states.is_empty() || solved.is_empty() {
                break;
            }
            solved = self.one_step();
        }

        solved
    }

    pub fn one_step(&mut self) -> Vec<K> {
        let mut solved_boards = Vec::new();
        let current_states = self.next_states.clone();
        self.next_states = Vec::new();

        for &current_key in &current_states {
            for next_board in self
                .tree
                .get_value(current_key)
                .expect("Board not found")
                .all_moves()
            {
                match self.board_to_key.get(&next_board) {
                    Some(next_key) => {
                        if self.next_states.contains(next_key) {
                            self.tree.connect(current_key, *next_key).unwrap();
                        }
                    }
                    None => {
                        let new_key = self
                            .tree
                            .insert_child_of(current_key, next_board.clone())
                            .expect("Parent not found");
                        if next_board.is_solved() {
                            solved_boards.push(new_key);
                        }
                        self.next_states.push(new_key);
                        self.board_to_key.insert(next_board, new_key);
                    }
                };
            }
        }

        solved_boards
    }
}
