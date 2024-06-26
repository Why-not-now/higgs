use std::cmp::min;

use ndarray::{Array2, Ix2};
use sorted_vec::SortedSet;

use crate::container::{
    contents_positions, Component, Container, ContainerLUT, ContainerTrait, Contents,
};
use crate::obstacle::Obstacle;
use crate::ordered::OrdIx2;
use crate::particle::{Particle, ParticleTrait};
use crate::property::Direction;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct Board {
    width: usize,
    height: usize,
    goals: SortedSet<OrdIx2>,
    container_lut: ContainerLUT,
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

    pub fn new(width: usize, height: usize, goals: SortedSet<OrdIx2>) -> Self {
        Self {
            width,
            height,
            goals,
            container_lut: ContainerLUT::new(),
            particles: Array2::default([width, height]),
            obstacles: Array2::default([width, height]),
        }
    }

    pub fn is_solved(&self) -> bool {
        self.goals
            .iter()
            .all(|&i| !matches!(self.particles.get(*i).unwrap(), Particle::Empty(_)))
    }

    pub fn all_moves(&self) -> Vec<Board> {
        self.particles
            .indexed_iter()
            .filter(|&(_pos, particle)| !matches!(particle, Particle::Empty(_)))
            .flat_map(|(pos, particle)| particle.all_moves(self, Ix2(pos.0, pos.1)))
            .collect()
    }

    pub fn annihilate(&mut self, pos: Ix2, strength: usize) {
        let x = pos[0];
        let y = pos[1];

        let min_x = x.saturating_sub(strength);
        let min_y = y.saturating_sub(strength);
        let max_x = min(x + strength + 1, self.width);
        let max_y = min(y + strength + 1, self.height);

        for j in min_y..max_y {
            for i in min_x..max_x {
                if i.abs_diff(x) + j.abs_diff(y) <= strength {
                    self.remove_obstacle(Ix2(i, j));
                }
            }
        }
    }

    pub fn move_particle(&mut self, from_pos: Ix2, to_pos: Ix2) -> Option<Particle> {
        if from_pos == to_pos {
            return None;
        }
        self.particles.swap(from_pos, to_pos);

        Some(std::mem::take(
            self.particles
                .get_mut(from_pos)
                .expect("out of index should had be panicked by swap"),
        ))
    }

    pub fn add_particle(&mut self, particle: impl Into<Particle>, pos: Ix2) {
        self.particles[pos] = particle.into();
    }

    pub fn remove_particle(&mut self, pos: Ix2) -> Option<Particle> {
        Some(std::mem::take(&mut *self.particles.get_mut(pos)?))
    }

    pub fn add_obstacle(&mut self, obstacle: impl Into<Obstacle>, pos: Ix2) {
        self.obstacles[pos] = obstacle.into();
    }

    pub fn remove_obstacle(&mut self, pos: Ix2) -> Option<Obstacle> {
        Some(std::mem::take(&mut *self.obstacles.get_mut(pos)?))
    }

    pub fn add_container(&mut self, container: Container) {
        for component in container.contents().iter() {
            self.container_lut
                .insert(component.clone(), container.clone());
        }
    }

    pub fn remove_container(&mut self, contents: &Container) {
        for component in contents.contents().iter() {
            self.container_lut.remove(component);
        }
    }

    pub fn top_container(&self, component: &Component) -> Option<&Container> {
        let mut previous = component;
        let mut cloned: Component;
        let mut result: Option<&Container> = None;
        while let Some(next) = self.container_lut.get(previous) {
            result = Some(next);
            cloned = Component::from(next.contents().clone());
            previous = &cloned;
        }

        result
    }

    pub fn charge(&self, pos: Ix2) -> i32 {
        if let Some(container) = self.top_container(&Component::Particle(OrdIx2::from(pos))) {
            container.charge()
        } else {
            self.particles()
                .get(pos)
                .expect("Index is out of range")
                .charge()
        }
    }

    pub fn find<T>(
        &self,
        pos: Ix2,
        direction: Direction,
        container_fn: impl Fn(&Container) -> bool,
        particle_fn: impl Fn(&Particle) -> bool,
    ) -> Option<Component> {
        let mut previous = pos;
        while let Some(pos) = self.move_direction(direction, previous) {
            match self.top_container(&Component::Particle(OrdIx2::from(pos))) {
                Some(c) => {
                    if container_fn(c) {
                        return Some(Component::Container(c.contents().clone()));
                    }
                }
                None => {
                    if particle_fn(self.particles().get(pos).expect("Index is out of range")) {
                        return Some(Component::Particle(OrdIx2::from(pos)));
                    }
                }
            };

            previous = pos
        }

        None
    }

    pub fn find_value<T>(
        &self,
        pos: Ix2,
        direction: Direction,
        container_fn: impl Fn(&Container) -> Option<T>,
        particle_fn: impl Fn(&Particle) -> Option<T>,
    ) -> Option<(Component, T)> {
        let mut previous = pos;
        while let Some(pos) = self.move_direction(direction, previous) {
            let result = match self.top_container(&Component::Particle(OrdIx2::from(pos))) {
                Some(c) => container_fn(c).map(|v| (Component::Container(c.contents().clone()), v)),
                None => particle_fn(self.particles().get(pos).expect("Index is out of range"))
                    .map(|v| (Component::Particle(OrdIx2::from(pos)), v)),
            };

            if result.is_some() {
                return result;
            }

            previous = pos
        }

        None
    }

    pub fn find_charge_single(&self, pos: Ix2, direction: Direction) -> i32 {
        let mut previous = pos;
        while let Some(pos) = self.move_direction(direction, previous) {
            let charge = self.charge(pos);
            if charge != 0 {
                return charge;
            }
            previous = pos
        }

        0
    }

    pub fn find_charge_many(&self, contents: &Contents, direction: Direction) -> i32 {
        let mut visited = SortedSet::new();
        let mut total_charge = 0;

        for pos in contents_positions(contents).iter() {
            if let Some((component, charge)) = self.find_value(
                Ix2::from(**pos),
                direction,
                |c| {
                    let charge = c.charge();
                    if charge != 0 {
                        return Some(charge);
                    }
                    None
                },
                |p| {
                    let charge = p.charge();
                    if charge != 0 {
                        return Some(charge);
                    }
                    None
                },
            ) {
                if let (_, None) = visited.push(component) {
                    total_charge += charge;
                };
                break;
            };
        }

        total_charge
    }

    pub fn left(&self, pos: Ix2) -> Option<Ix2> {
        if pos[0] == 0 {
            return None;
        };
        Some(pos - Ix2(1, 0))
    }

    pub fn right(&self, pos: Ix2) -> Option<Ix2> {
        if pos[0] >= self.width - 1 {
            return None;
        };
        Some(pos + Ix2(1, 0))
    }

    pub fn up(&self, pos: Ix2) -> Option<Ix2> {
        if pos[1] == 0 {
            return None;
        };
        Some(pos - Ix2(0, 1))
    }

    pub fn down(&self, pos: Ix2) -> Option<Ix2> {
        if pos[1] >= self.height - 1 {
            return None;
        };
        Some(pos + Ix2(0, 1))
    }

    pub fn move_direction(&self, direction: Direction, pos: Ix2) -> Option<Ix2> {
        match direction {
            Direction::Right => self.right(pos),
            Direction::Down => self.down(pos),
            Direction::Left => self.left(pos),
            Direction::Up => self.up(pos),
        }
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
        if self.width == 0 || self.height == 0 {
            return;
        }
        println!("╔══{}═╗", "═╤══".repeat(self.width - 1));
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
            println!("║ {} ║", x.join(" │ "));
            if row < self.height - 1 {
                println!("╟──{}─╢", ("─┼──").repeat(self.width - 1));
            }
        }
        println!("╚══{}═╝", "═╧══".repeat(self.width - 1));
    }
}

#[cfg(test)]
mod tests {
    use crate::particle::{Electron, Empty};

    use super::*;

    #[test]
    fn board_move_particle() {
        let mut board = Board::new(5, 7, vec![Ix2(1, 0).into()].into());
        board.add_particle(Electron::default(), Ix2(3, 2));
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
