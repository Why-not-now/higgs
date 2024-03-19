use higgs::board::Board;
use higgs::obstacle::{block::Block, hole::Hole};
use higgs::particle::{Electron, Neutron, Tau};
use higgs::solver::Solver;
use ndarray::Ix2;
use slotmap::new_key_type;

fn main() {
    new_key_type! { pub struct SolverKey; }

    let mut board = Board::default(7, 8, vec![Ix2(4, 2)].into());

    board.add_particle(Electron::default().into(), Ix2(3, 1));
    board.add_particle(Tau::default().into(), Ix2(2, 2));
    board.add_particle(Tau::default().into(), Ix2(3, 6));
    board.add_particle(Tau::default().into(), Ix2(1, 7));
    board.add_particle(Neutron::default().into(), Ix2(2, 0));
    board.add_obstacle(Block.into(), Ix2(6, 7));
    board.add_obstacle(Hole.into(), Ix2(4, 3));
    board.add_obstacle(Hole.into(), Ix2(5, 3));
    board.add_obstacle(Hole.into(), Ix2(6, 3));
    board.add_obstacle(Hole.into(), Ix2(2, 7));
    board.add_obstacle(Hole.into(), Ix2(5, 7));

    let mut solver: Solver<SolverKey> = Solver::new(board);

    let mut solutions = &solver.solve();
    let mut steps = Vec::new();
    while let Some(&step) = solutions.first() {
        // let particles: Vec<_> = solver
        //     .tree()
        //     .get_value(*step)
        //     .unwrap()
        //     .particles()
        //     .indexed_iter()
        //     .filter(|(_index, particle)| !matches!(particle, Particle::Empty(_)))
        //     .collect();
        steps.push(step);
        solutions = solver.tree().parents_of(step).unwrap()
    }

    for &step in steps.iter().rev() {
        solver.tree().get_value(step).unwrap().display_particles();
    }

    // for &next_keys in solver.next_states() {
    //     solver
    //         .tree()
    //         .get_value(next_keys)
    //         .unwrap()
    //         .display_particles();
    // }
}
