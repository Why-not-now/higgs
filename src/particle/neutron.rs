use ndarray::Ix2;

use crate::board::Board;
use crate::obstacle::Obstacle;
use crate::particle::Particle;
use crate::property::Antiness;

use super::ParticleTrait;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Default)]
pub struct Neutron {
    anti: Antiness,
}

impl ParticleTrait for Neutron {
    fn all_moves(&self, board: &Board, pos: Ix2) -> Vec<Board> {
        vec![
            self.one_move(board, pos, board.right_axis_indices(pos)),
            self.one_move(board, pos, board.down_axis_indices(pos)),
            self.one_move(board, pos, board.left_axis_indices(pos)),
            self.one_move(board, pos, board.up_axis_indices(pos)),
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

#[cfg(test)]
mod tests {
    use crate::particle::Empty;

    use super::*;

    #[test]
    fn move_neutron_edge() {
        let width = 5;
        let height = 7;
        let mut board = Board::default(width, height, vec![].into());

        let x = 3;
        let y = 2;

        let start_index = Ix2(x, y);

        board.add_particle(Neutron::default().into(), start_index);

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
        let mut board = Board::default(width, height, vec![].into());

        let top_left_index = Ix2(0, 0);
        let bottom_right_index = Ix2(width - 1, height - 1);

        board.add_particle(Neutron::default().into(), top_left_index);
        board.add_particle(Neutron::default().into(), bottom_right_index);

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
        let mut board = Board::default(width, height, vec![].into());

        let x = 3;
        let y = 2;

        let start_index = Ix2(x, y);
        let up_one_index = Ix2(x, y - 1);
        let left_two_index = Ix2(x - 2, y);
        let right_three_index = Ix2(x + 3, y);
        let down_four_index = Ix2(x, y + 4);

        board.add_particle(Neutron::default().into(), start_index);
        board.add_particle(Neutron::default().into(), up_one_index);
        board.add_particle(Neutron::default().into(), left_two_index);
        board.add_particle(Neutron::default().into(), right_three_index);
        board.add_particle(Neutron::default().into(), down_four_index);

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
        let mut board = Board::default(width, height, vec![].into());

        let x = 3;
        let y = 2;

        let start_index = Ix2(x, y);
        let up_one_index = Ix2(x, y - 1);
        let left_two_index = Ix2(x - 2, y);
        let right_three_index = Ix2(x + 3, y);
        let down_four_index = Ix2(x, y + 4);

        board.add_particle(Neutron::default().into(), start_index);
        board.add_particle(Neutron::default().into(), up_one_index);
        board.add_particle(Neutron::default().into(), left_two_index);
        board.add_particle(Neutron::default().into(), right_three_index);
        board.add_particle(Neutron::default().into(), down_four_index);

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
