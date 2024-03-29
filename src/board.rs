use ndarray::{Array2, Ix2};

use crate::obstacle::Obstacle;
use crate::particle::{Particle, ParticleTrait};
use crate::unordered::UnorderedVec;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct Board {
    width: usize,
    height: usize,
    goals: UnorderedVec<Ix2>,
    particles: Array2<Particle>,
    obstacles: Array2<Obstacle>,
}

impl Board {
    pub fn particles(&self) -> &Array2<Particle> {
        &self.particles
    }

    pub fn obstacles(&self) -> &Array2<Obstacle> {
        &self.obstacles
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn new(
        width: usize,
        height: usize,
        goals: UnorderedVec<Ix2>,
        particles: Array2<Particle>,
        obstacles: Array2<Obstacle>,
    ) -> Self {
        Self {
            width,
            height,
            goals,
            particles,
            obstacles,
        }
    }

    pub fn default(width: usize, height: usize, goals: UnorderedVec<Ix2>) -> Self {
        Self {
            width,
            height,
            goals,
            particles: Array2::default([width, height]),
            obstacles: Array2::default([width, height]),
        }
    }

    pub fn is_solved(&self) -> bool {
        self.goals
            .iter()
            .all(|&i| !matches!(self.particles.get(i).unwrap(), Particle::Empty(_)))
    }

    pub fn all_moves(&self) -> Vec<Board> {
        self.particles
            .indexed_iter()
            .filter(|&(_pos, particle)| !matches!(particle, Particle::Empty(_)))
            .flat_map(|(pos, particle)| particle.all_moves(self, Ix2(pos.0, pos.1)))
            .collect()
    }

    pub fn move_particle(&mut self, from_pos: Ix2, to_pos: Ix2) {
        self.particles[to_pos] = *match self.particles.get(from_pos) {
            Some(particle) => particle,
            None => return,
        };
        self.particles[from_pos] = Particle::default();
    }

    pub fn add_particle(&mut self, particle: Particle, pos: Ix2) {
        self.particles[pos] = particle;
    }

    pub fn remove_particle(&mut self, pos: Ix2) {
        self.particles[pos] = Particle::default();
    }

    pub fn add_obstacle(&mut self, obstacle: Obstacle, pos: Ix2) {
        self.obstacles[pos] = obstacle;
    }

    pub fn remove_obstacle(&mut self, pos: Ix2) {
        self.obstacles[pos] = Obstacle::default();
    }

    pub fn left_axis_indices(&self, pos: Ix2) -> impl Iterator<Item = Ix2> {
        let x = pos[0];
        let y = pos[1];

        (0..x).rev().map(move |x| Ix2(x, y))
    }

    pub fn right_axis_indices(&self, pos: Ix2) -> impl Iterator<Item = Ix2> {
        let x = pos[0];
        let y = pos[1];

        ((x + 1)..self.width).map(move |x| Ix2(x, y))
    }

    pub fn up_axis_indices(&self, pos: Ix2) -> impl Iterator<Item = Ix2> {
        let x = pos[0];
        let y = pos[1];

        (0..y).rev().map(move |y| Ix2(x, y))
    }

    pub fn down_axis_indices(&self, pos: Ix2) -> impl Iterator<Item = Ix2> {
        let x = pos[0];
        let y = pos[1];

        ((y + 1)..self.height).map(move |y| Ix2(x, y))
    }

    pub fn display_particles(&self) {
        println!("╔═{}╗", "╤═".repeat(self.width - 1));
        for (row, content) in self.particles().columns().into_iter().enumerate() {
            let x: Vec<_> = content
                .iter()
                .map(|p| match p {
                    Particle::Empty(_) => " ",
                    Particle::Electron(_) => "e",
                    Particle::Muon(_) => "𝜇",
                    Particle::Tau(_) => "𝜏",
                    Particle::Neutron(_) => "n",
                })
                .collect();
            println!("║{}║", x.join("│"));
            if row < self.height - 1 {
                println!("╟─{}╢", ("┼─").repeat(self.width - 1));
            }
        }
        println!("╚═{}╝", "╧═".repeat(self.width - 1));
    }
}

#[cfg(test)]
mod tests {
    use crate::particle::{Electron, Empty};

    use super::*;

    #[test]
    fn board_move_particle() {
        let mut board = Board::default(5, 7, vec![Ix2(1, 0)].into());
        board.add_particle(Electron::default().into(), Ix2(3, 2));
        assert_eq!(
            board.particles().get([3, 2]),
            Some(&Electron::default().into())
        );
        assert!(!board.is_solved());

        board.move_particle(Ix2(3, 2), Ix2(1, 0));
        assert_eq!(board.particles().get([3, 2]), Some(&Empty.into()));
        assert_eq!(
            board.particles().get([1, 0]),
            Some(&Electron::default().into())
        );
        assert!(board.is_solved());

        board.remove_particle(Ix2(1, 0));
        assert_eq!(board.particles().get([1, 0]), Some(&Empty.into()));
    }
}
