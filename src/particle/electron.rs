use ndarray::Ix2;

use crate::board::Board;
use crate::obstacle::Obstacle;
use crate::property::{AntiTrait, Antiness, Direction};

use super::{Particle, ParticleTrait};

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Default)]
pub struct Electron {
    anti: Antiness,
}

impl Electron {
    pub fn new(anti: Antiness) -> Self {
        Self { anti }
    }
}

impl AntiTrait for Electron {
    fn anti(&self) -> Antiness {
        self.anti
    }
}

impl ParticleTrait for Electron {
    fn charge(&self) -> i32 {
        match self.anti() {
            Antiness::Ordinary => -3,
            Antiness::Anti => 3,
        }
    }

    fn all_moves(&self, board: &Board, pos: Ix2) -> Vec<Board> {
        self.check_electric(board, pos)
            .iter()
            .filter_map(|direction| match direction {
                Direction::Right => self.one_move(board, pos, board.right_axis_indices(pos)),
                Direction::Down => self.one_move(board, pos, board.down_axis_indices(pos)),
                Direction::Left => self.one_move(board, pos, board.left_axis_indices(pos)),
                Direction::Up => self.one_move(board, pos, board.up_axis_indices(pos)),
            })
            .collect()
    }
}

impl Electron {
    fn check_electric(&self, board: &Board, pos: Ix2) -> Vec<Direction> {
        let mut ret_directions: Vec<Direction> = Vec::new();

        let x = self.charge()
            * (Self::next_charge(board, &mut board.right_axis_indices(pos))
                - Self::next_charge(board, &mut board.left_axis_indices(pos)));
        if x >= 0 {
            ret_directions.push(Direction::Left)
        }
        if x <= 0 {
            ret_directions.push(Direction::Right)
        }

        let y = self.charge()
            * (Self::next_charge(board, &mut board.down_axis_indices(pos))
                - Self::next_charge(board, &mut board.up_axis_indices(pos)));
        if y >= 0 {
            ret_directions.push(Direction::Up)
        }
        if y <= 0 {
            ret_directions.push(Direction::Down)
        }

        ret_directions
    }

    fn next_charge(board: &Board, path_indices: &mut impl Iterator<Item = Ix2>) -> i32 {
        path_indices
            .map(|i| board.particles().get(i))
            .find_map(|p| {
                let charge = p.unwrap().charge();
                if charge != 0 {
                    Some(charge)
                } else {
                    None
                }
            })
            .unwrap_or(0)
    }

    fn one_move(
        &self,
        board: &Board,
        pos: Ix2,
        mut path_indices: impl Iterator<Item = Ix2>,
    ) -> Option<Board> {
        let first = path_indices.next()?;
        match board.particles().get(first).unwrap() {
            Particle::Empty(_) => (),
            _ => return None,
        };
        match board.obstacles().get(first).unwrap() {
            Obstacle::Empty(_) => (),
            Obstacle::Block(_) => return None,
            Obstacle::Hole(_) => {
                let mut ret_board = board.clone();
                ret_board.remove_particle(pos);
                return Some(ret_board);
            }
        };

        let mut last = first;
        for next in path_indices {
            match board.particles().get(next).unwrap() {
                Particle::Empty(_) => (),
                _ => {
                    let mut ret_board = board.clone();
                    ret_board.move_particle(pos, last);
                    return Some(ret_board);
                }
            };
            match board.obstacles().get(next).unwrap() {
                Obstacle::Empty(_) => (),
                Obstacle::Block(_) => {
                    let mut ret_board = board.clone();
                    ret_board.move_particle(pos, last);
                    return Some(ret_board);
                }
                Obstacle::Hole(_) => {
                    let mut ret_board = board.clone();
                    ret_board.remove_particle(pos);
                    return Some(ret_board);
                }
            };

            last = next;
        }

        let mut ret_board = board.clone();
        ret_board.move_particle(pos, last);
        Some(ret_board)
    }
}
