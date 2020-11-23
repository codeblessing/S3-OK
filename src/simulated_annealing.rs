use std::cell::RefCell;
use crate::utils::Schedule;

/// Temperature reduction rule used by evaluation algorithm.\
/// Linear(α): `T = T - α`, in that one α have to be positive\
/// Geometric(α): `T = T * α`, in that one α have to be from range (0;1)\
/// Slow Decrease(β): `T = T / (1 + βT)`
enum Reduction {
    Linear(f64),
    Geometric(f64),
    SlowDecrease(f64)
}

/// Simulated Annealing implementation.
struct Solution {
    initial_solution: Schedule,
    reduction_rule: Reduction,
    current_temperature: RefCell<f64>,
    final_temperature: f64,
    iteration_count: u16,
}