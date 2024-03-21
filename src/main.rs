use higgs::board::Board;
use higgs::obstacle::{block::Block, hole::Hole};
use higgs::particle::{Electron, Neutron, Tau};
use higgs::solver::Solver;
use ndarray::Ix2;
use slotmap::new_key_type;

fn main() {
    new_key_type! { pub struct SolverKey; }

    let mut board = Board::default(7, 8, vec![Ix2(4, 2).into()].into());

    board.add_particle(Electron::default(), Ix2(3, 1));
    board.add_particle(Tau::default(), Ix2(2, 2));
    board.add_particle(Tau::default(), Ix2(3, 6));
    board.add_particle(Tau::default(), Ix2(1, 7));
    board.add_particle(Neutron::default(), Ix2(2, 0));
    board.add_obstacle(Block, Ix2(6, 7));
    board.add_obstacle(Hole, Ix2(4, 3));
    board.add_obstacle(Hole, Ix2(5, 3));
    board.add_obstacle(Hole, Ix2(6, 3));
    board.add_obstacle(Hole, Ix2(2, 7));
    board.add_obstacle(Hole, Ix2(5, 7));

    let mut solver: Solver<SolverKey> = Solver::new(board);

    let mut solutions = &solver.solve();
    let mut steps = Vec::new();
    while let Some(&step) = solutions.first() {
        steps.push(step);
        solutions = solver.tree().parents_of(step).unwrap()
    }

    for &step in steps.iter().rev() {
        solver.tree().get_value(step).unwrap().display_particles();
    }
}
