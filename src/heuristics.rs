use std::env::consts::EXE_SUFFIX;

use crate::board::*;

/// A heuristic function to estimate the cost of reaching the goal state from a given board.
///
/// ```rust
/// let board = Board::new([[8, 7, 3], [2, 0, 5], [1, 4, 6]]);
/// let h = Heuristic::Manhattan.estimate(&board);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Heuristic {
    /// The blind heuristic always returns 0.
    Blind,
    /// The Hamming heuristic, which counts the number of misplaced tiles.
    Hamming,
    /// The Manhattan heuristic, which computes the sum of the Manhattan distances of each tile to its goal position.
    Manhattan,
}

impl Heuristic {
    pub fn estimate(&self, board: &Board) -> u32 {
        match self {
            // blind heuristic always returns 0
            Heuristic::Blind => 0,
            Heuristic::Hamming => {
                let mut dist=0;
                for i in (0..3) {
                    for j in (0..3) {
                        let value = if j==2 && i==2 {0} else { (i*3 + j + 1) };
                        if usize::from(board.value_at(i, j)) != value {
                            dist+=1;
                        }
                    }
                }
                return dist;
            }
            Heuristic::Manhattan => {
                let mut dist=0;
                for i in (0..3) {
                    for j in (0..3) {
                        let expected = (i*3 + j + 1);
                        let mut provided =usize::from(board.value_at(i, j));
                        
                        provided = if (provided == 0) {9} else {provided};

                        let diff = (expected).abs_diff(provided);
                        
                        dist += diff / 3 + diff % 3 ;
                    }
                }
                
                return dist as u32;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_heuristic() {
        use super::*;
        let board = Board::new([[8, 7, 3], [2, 0, 5], [1, 4, 6]]);
        assert_eq!(Heuristic::Blind.estimate(&board), 0);
        assert_eq!(Heuristic::Hamming.estimate(&board), 8);
        assert_eq!(Heuristic::Manhattan.estimate(&board), 16);
    }
}
