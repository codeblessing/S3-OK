use crate::utils::Schedule;
use std::cell::RefCell;

/// Temperature reduction rule used by evaluation algorithm.\
/// Linear(α): `T = T - α`, in that one α have to be positive\
/// Geometric(α): `T = T * α`, in that one α have to be from range (0;1)\
/// Slow Decrease(β): `T = T / (1 + βT)`
#[derive(PartialEq, Debug)]
pub enum Reduction {
    Linear(f64),
    Geometric(f64),
    SlowDecrease(f64),
}

/// Simulated Annealing implementation.
struct Solution {
    initial_solution: Schedule,
    reduction_rule: Reduction,
    current_temperature: RefCell<f64>,
    final_temperature: f64,
    iteration_count: u16,
}

impl Solution {
    pub fn new() -> Self {
        Self {
            initial_solution: Schedule::new(),
            reduction_rule: Reduction::Linear(0.0),
            current_temperature: RefCell::new(0.0),
            final_temperature: 0.0,
            iteration_count: 0,
        }
    }

    pub fn with_reduction_rule(mut self, rule: Reduction) -> Self {
        self.reduction_rule = rule;
        self
    }

    pub fn with_temperature(mut self, temperature: f64) -> Self {
        self.current_temperature = RefCell::new(temperature);
        self
    }

    pub fn with_final_temperature(mut self, temperature: f64) -> Self {
        self.final_temperature = temperature;
        self
    }

    pub fn with_iterations_per_temperature(mut self, iteration_count: u16) -> Self {
        self.iteration_count = iteration_count;
        self
    }
}

#[cfg(test)]
mod test_simulated_annealing {
    use super::*;

    #[test]
    fn test_create_empty() {
        let solution = Solution::new();

        assert_eq!(solution.reduction_rule, Reduction::Linear(0.0));
        assert_eq!(*solution.current_temperature.borrow(), 0.0);
        assert_eq!(solution.final_temperature, 0.0);
        assert_eq!(solution.iteration_count, 0);
    }

    #[test]
    fn test_set_reduction_rule() {
        let solution = Solution::new().with_reduction_rule(Reduction::Geometric(0.5));

        assert_eq!(solution.reduction_rule, Reduction::Geometric(0.5));
    }

    #[test]
    fn test_set_initial_temperature() {
        let solution = Solution::new().with_temperature(85.0);

        assert_eq!(*solution.current_temperature.borrow(), 85.0);
    }

    #[test]
    fn test_set_final_temperature() {
        let solution = Solution::new().with_final_temperature(30.0);

        assert_eq!(solution.final_temperature, 30.0);
    }

    #[test]
    fn test_set_iteration_count_per_temperature() {
        let solution = Solution::new().with_iterations_per_temperature(100);

        assert_eq!(solution.iteration_count, 100);
    }

    
}
