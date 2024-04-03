use ndarray::Ix2;

use crate::board::Board;
use crate::obstacle::Obstacle;
use crate::property::{AntiTrait, Antiness, Direction};

use super::{Particle, ParticleTrait};

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Default, PartialOrd, Ord)]
pub struct Tau {
    anti: Antiness,
}

impl Tau {
    pub fn new(anti: Antiness) -> Self {
        Self { anti }
    }
}

impl AntiTrait for Tau {
    fn anti(&self) -> Antiness {
        self.anti
    }
}

impl ParticleTrait for Tau {
    fn charge(&self) -> i32 {
        match self.anti() {
            Antiness::Ordinary => -3,
            Antiness::Anti => 3,
        }
    }

    fn all_moves(&self, board: &Board, pos: Ix2) -> Vec<Board> {
        self.check_electric(board, pos)
            .iter()
            .filter_map(|&direction| self.one_move(board, pos, direction))
            .collect()
    }
}

impl Tau {
    fn check_electric(&self, board: &Board, pos: Ix2) -> Vec<Direction> {
        let mut ret_directions: Vec<Direction> = Vec::new();

        let x = self.charge()
            * (Self::next_charge(board, pos, Direction::Right)
                - Self::next_charge(board, pos, Direction::Left));
        if x > 0 {
            ret_directions.push(Direction::Left)
        }
        if x < 0 {
            ret_directions.push(Direction::Right)
        }

        let y = self.charge()
            * (Self::next_charge(board, pos, Direction::Down)
                - Self::next_charge(board, pos, Direction::Up));
        if y > 0 {
            ret_directions.push(Direction::Up)
        }
        if y < 0 {
            ret_directions.push(Direction::Down)
        }

        ret_directions
    }

    fn next_charge(board: &Board, pos: Ix2, direction: Direction) -> i32 {
        let mut previous = pos;
        while let Some(pos) = board.move_direction(direction, previous) {
            let charge = board
                .particles()
                .get(pos)
                .expect("Particle not found.")
                .charge();
            if charge != 0 {
                return charge;
            }

            previous = pos
        }

        0
    }

    fn one_move(&self, board: &Board, pos: Ix2, direction: Direction) -> Option<Board> {
        let move_fn = |i| board.move_direction(direction, i);
        let next = move_fn(pos)?;
        match board.particles().get(next).unwrap() {
            Particle::Empty(_) => (),
            Particle::Tau(t) => match t.anti == self.anti {
                true => return None,
                false => {
                    let mut ret_board = board.clone();
                    ret_board.remove_particle(pos);
                    ret_board.remove_particle(next);
                    ret_board.annihilate(next, 3);
                    return Some(ret_board);
                }
            },
            _ => return None,
        };
        match board.obstacles().get(next).unwrap() {
            Obstacle::Empty(_) => (),
            Obstacle::Block(_) => return None,
            Obstacle::Hole(_) => {
                let mut ret_board = board.clone();
                ret_board.remove_particle(pos);
                return Some(ret_board);
            }
        };

        let mut previous = next;
        let mut ret_board = board.clone();
        while let Some(next) = move_fn(previous) {
            match board.particles().get(next).unwrap() {
                Particle::Empty(_) => (),
                Particle::Tau(t) => match t.anti == self.anti {
                    true => {
                        ret_board.move_particle(pos, previous);
                        return Some(ret_board);
                    }
                    false => {
                        ret_board.remove_particle(pos);
                        ret_board.remove_particle(next);
                        ret_board.annihilate(next, 3);
                        return Some(ret_board);
                    }
                },
                _ => {
                    ret_board.move_particle(pos, previous);
                    return Some(ret_board);
                }
            };
            match board.obstacles().get(next).unwrap() {
                Obstacle::Empty(_) => (),
                Obstacle::Block(_) => {
                    ret_board.move_particle(pos, previous);
                    return Some(ret_board);
                }
                Obstacle::Hole(_) => {
                    ret_board.remove_particle(pos);
                    return Some(ret_board);
                }
            };
            previous = next
        }
        ret_board.move_particle(pos, previous)?;
        Some(ret_board)
    }
}
