use ndarray::Ix2;

use crate::board::Board;
use crate::obstacle::Obstacle;
use crate::particle::Particle;
use crate::property::{Antiness, Step};

use super::ParticleTrait;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Default, PartialOrd, Ord)]
pub struct Neutron {
    anti: Antiness,
}

impl ParticleTrait for Neutron {
    fn all_moves(&self, board: &Board, pos: Ix2) -> Vec<Board> {
        vec![
            self.one_move(board, pos, |i| board.right(i)),
            self.one_move(board, pos, |i| board.down(i)),
            self.one_move(board, pos, |i| board.left(i)),
            self.one_move(board, pos, |i| board.up(i)),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl Neutron {
    fn one_move(
        &self,
        board: &Board,
        pos: Ix2,
        move_fn: impl Fn(Ix2) -> Option<Ix2>,
    ) -> Option<Board> {
        let next = move_fn(pos)?;
        let mut step = self.one_step(board, next);
        let mut previous = match step {
            Step::Continue => next,
            Step::Shift => return None,
            Step::Annihilate(strength) => {
                let mut ret_board = board.clone();
                ret_board.remove_particle(pos);
                ret_board.annihilate(next, strength);
                return Some(ret_board);
            }
            Step::Remove => {
                let mut ret_board = board.clone();
                ret_board.remove_particle(pos);
                return Some(ret_board);
            }
        };
        while let Some(next) = move_fn(previous) {
            step = self.one_step(board, next);
            previous = match step {
                Step::Continue => next,
                Step::Shift => {
                    let mut ret_board = board.clone();
                    ret_board.move_particle(pos, previous);
                    return Some(ret_board);
                }
                Step::Annihilate(_) => {
                    let mut ret_board = board.clone();
                    ret_board.move_particle(pos, next);
                    return Some(ret_board);
                }
                Step::Remove => {
                    let mut ret_board = board.clone();
                    ret_board.remove_particle(pos);
                    return Some(ret_board);
                }
            }
        }
        let mut ret_board = board.clone();
        ret_board.move_particle(pos, previous)?;
        Some(ret_board)
    }

    pub fn one_step(&self, board: &Board, next: Ix2) -> Step {
        match board.particles().get(next).unwrap() {
            Particle::Empty(_) => (),
            Particle::Neutron(n) => match n.anti == self.anti {
                true => return Step::Shift,
                false => return Step::Annihilate(3),
            },
            _ => return Step::Shift,
        };
        match board.obstacles().get(next).unwrap() {
            Obstacle::Empty(_) => (),
            Obstacle::Block(_) => return Step::Shift,
            Obstacle::Hole(_) => return Step::Remove,
        };
        Step::Continue
    }
}

#[cfg(test)]
mod tests {
    use crate::particle::Empty;

    use super::*;

    #[test]
    fn move_neutron_edge() {
        let width = 5;
        let height = 7;
        let mut board = Board::new(width, height, vec![].into());

        let x = 3;
        let y = 2;

        let start_index = Ix2(x, y);

        board.add_particle(Neutron::default(), start_index);

        let new_boards = board
            .particles()
            .get(start_index)
            .unwrap()
            .all_moves(&board, start_index);
        for new_b in &new_boards {
            assert_eq!(new_b.particles().get(start_index), Some(&Empty.into()));
        }

        let right = &new_boards[0];
        let down = &new_boards[1];
        let left = &new_boards[2];
        let up = &new_boards[3];

        assert_eq!(
            right.particles().get(Ix2(width - 1, y)),
            Some(&Neutron::default().into())
        );
        assert_eq!(
            down.particles().get(Ix2(x, height - 1)),
            Some(&Neutron::default().into())
        );
        assert_eq!(
            left.particles().get(Ix2(0, y)),
            Some(&Neutron::default().into())
        );
        assert_eq!(
            up.particles().get(Ix2(x, 0)),
            Some(&Neutron::default().into())
        );
    }

    #[test]
    fn move_neutron_edge_flatten() {
        let width = 5;
        let height = 7;
        let mut board = Board::new(width, height, vec![].into());

        let top_left_index = Ix2(0, 0);
        let bottom_right_index = Ix2(width - 1, height - 1);

        board.add_particle(Neutron::default(), top_left_index);
        board.add_particle(Neutron::default(), bottom_right_index);

        assert_eq!(
            board
                .particles()
                .get(top_left_index)
                .unwrap()
                .all_moves(&board, top_left_index)
                .len(),
            2
        );
        assert_eq!(
            board
                .particles()
                .get(bottom_right_index)
                .unwrap()
                .all_moves(&board, bottom_right_index)
                .len(),
            2
        );
    }

    #[test]
    fn move_neutron_collide() {
        let width = 7;
        let height = 9;
        let mut board = Board::new(width, height, vec![].into());

        let x = 3;
        let y = 2;

        let start_index = Ix2(x, y);
        let up_one_index = Ix2(x, y - 1);
        let left_two_index = Ix2(x - 2, y);
        let right_three_index = Ix2(x + 3, y);
        let down_four_index = Ix2(x, y + 4);

        board.add_particle(Neutron::default(), start_index);
        board.add_particle(Neutron::default(), up_one_index);
        board.add_particle(Neutron::default(), left_two_index);
        board.add_particle(Neutron::default(), right_three_index);
        board.add_particle(Neutron::default(), down_four_index);

        let new_boards = board
            .particles()
            .get(start_index)
            .unwrap()
            .all_moves(&board, start_index);

        assert_eq!(new_boards.len(), 3);

        let right = &new_boards[0];
        let down = &new_boards[1];
        let left = &new_boards[2];

        assert_eq!(
            right.particles().get(Ix2(x + 2, y)),
            Some(&Neutron::default().into())
        );
        assert_eq!(
            down.particles().get(Ix2(x, y + 3)),
            Some(&Neutron::default().into())
        );
        assert_eq!(
            left.particles().get(Ix2(x - 1, y)),
            Some(&Neutron::default().into())
        );
    }

    #[test]
    fn move_neutron_obstacle() {
        let width = 7;
        let height = 9;
        let mut board = Board::new(width, height, vec![].into());

        let x = 3;
        let y = 2;

        let start_index = Ix2(x, y);
        let up_one_index = Ix2(x, y - 1);
        let left_two_index = Ix2(x - 2, y);
        let right_three_index = Ix2(x + 3, y);
        let down_four_index = Ix2(x, y + 4);

        board.add_particle(Neutron::default(), start_index);
        board.add_particle(Neutron::default(), up_one_index);
        board.add_particle(Neutron::default(), left_two_index);
        board.add_particle(Neutron::default(), right_three_index);
        board.add_particle(Neutron::default(), down_four_index);

        let new_boards = board
            .particles()
            .get(start_index)
            .unwrap()
            .all_moves(&board, start_index);

        assert_eq!(new_boards.len(), 3);

        let right = &new_boards[0];
        let down = &new_boards[1];
        let left = &new_boards[2];

        assert_eq!(
            right.particles().get(Ix2(x + 2, y)),
            Some(&Neutron::default().into())
        );
        assert_eq!(
            down.particles().get(Ix2(x, y + 3)),
            Some(&Neutron::default().into())
        );
        assert_eq!(
            left.particles().get(Ix2(x - 1, y)),
            Some(&Neutron::default().into())
        );
    }
}
