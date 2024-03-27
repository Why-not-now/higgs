#![allow(dead_code)]

pub mod board;
pub mod container;
pub mod object;
pub mod obstacle;
pub mod ordered;
pub mod particle;
pub mod property;
pub mod solver;
pub mod tree;
pub mod unordered;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
