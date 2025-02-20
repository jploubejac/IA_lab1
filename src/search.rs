use crate::board::*;
use crate::heuristics;
use crate::heuristics::*;
use crate::min_heap::*;
use std::collections::*;
use std::i64::MAX;
use std::io::BufRead;
use std::time::Duration;

/// Statistics of the search, used to evaluate the performance of the search algorithms.
/// Feel free to add more fields to this struct if you need them.
pub struct Stats {
    /// Numbers of states expanded during search
    pub expanded: usize,
    /// Total runtime spend in the search.
    ///
    /// ```rust
    /// let start_time: Instant = std::time::Instant::now();
    /// // do something
    /// let runtime: Duration = start_time.elapsed();
    /// ```
    pub runtime: Duration,
}
impl Stats {
    /// Creates a new `Stats` instance with the given expanded states count and runtime.
    pub fn new(expanded: usize, runtime: Duration) -> Stats {
        Stats { expanded, runtime }
    }
}
pub struct Djikstra_label{ //for gens distinguche pepole
    pub cost:u32,
    pub marked:bool
}

// cargo test -- --nocapture to get println stdout outputs
pub fn search(init_state: Board) -> (Option<Vec<Direction>>, Stats) {
    let start = std::time::Instant::now();
    // MinHeap provide allows to store the states to explore, with associated priority
    let mut heap: MinHeap<Board> = MinHeap::new();
    // the standard library provides a HashMap, that can be used to store the cost or other things
    let mut cost_map: HashMap<Board, u32> = HashMap::new();
    let mut direction_map: HashMap<Board, Direction> = HashMap::new();
    MinHeap::insert(&mut heap, init_state, 0);
    HashMap::insert(&mut cost_map, init_state, 0);

    let mut current_board = init_state;

    let mut expanded=0;
    // here is an example to measure the runtime and returns the statistics
    let runtime = start.elapsed();
    while (!heap.is_empty() && current_board != Board::GOAL) {
        current_board = match heap.pop() {
            None => break,
            Some (fiston) => fiston
        };
        if current_board==Board::GOAL {
            break;
        }

        let current_cost = match cost_map.get(&current_board){
            None => panic!("aaaaaaaaaaah"),
            Some (fiston_cost) => (*fiston_cost)
        };

        for direction in DIRECTIONS {

            let new_board = match Board::apply(&current_board, direction) {
                None => continue,
                Some (n_board) => n_board
            };
            let cost = match cost_map.get(&new_board) {
                None => {
                    cost_map.insert(new_board, 1+current_cost);
                    direction_map.insert(new_board, direction);
                    heap.insert(new_board, 1+current_cost);
                    expanded+=1;
                    1+current_cost
                },
                Some (cout) => (*cout),
            };

            if 1+current_cost < cost {
                cost_map.remove(&new_board);
                cost_map.insert(new_board, 1+current_cost);
                direction_map.remove(&new_board);
                direction_map.insert(new_board, direction);
                heap.insert(new_board, 1+current_cost);
                expanded+=1;
            }
        }
    }

    // example to construct a Stats instance
    let stats = Stats::new(expanded, runtime);

    if current_board != Board::GOAL {
        return (None,stats);
    }
    let mut path: Vec<Direction> = Vec::new();

    let mut index = 0;
    while (current_board != init_state) {
        
        let dir = match direction_map.get(&current_board){
            None => panic!("Rien du tout"),
            Some (direction) => (*direction)
        };
        current_board= match Board::apply(&current_board, dir.opposite()) {
            None => panic!("Error building the path"),
            Some (board) => board
        };

        path.insert(index, dir);
        index +=1;
    }
    path.reverse();

    println!("Runtime: {} ns",stats.runtime.as_nanos());
    println!("Expanded nodes: {}\n",stats.expanded);

    // return the results and associated stats
    (Some(path), stats)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_search() {
        use super::*;

        // validates that search oes return the optimal plan on the first 20 isntances

        for (expected_cost, init) in &INSTANCES[0..20] {
            let (path, stats) = search(*init);
            let path = path.expect("no plan");
            assert!(init.is_valid_plan(&path));
            assert_eq!(path.len(), *expected_cost as usize);
        }
    }
}
