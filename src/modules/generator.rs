use crate::utils::Core;
use crate::utils::{Case, Schedule, Task};

use rand::thread_rng as random_generator;
use rand::Rng;

/// Generates tuple (case, schedule)
/// where `case` is test case with `cores` number of cores and random number of tasks
/// which can be scheduled with optimal solution equal to `optimal`.
/// `schedule` is this test case scheduled in optimal way.
///
/// # Panics
/// When cores == 0
pub fn generate(cores: u16, optimal: Option<u64>) -> (Case, Schedule) {
    if cores == 0 {
        panic!("Core count have to be positive.")
    }

    let mut random = random_generator();

    // If optimal value was given then use it,
    // otherwise randomly choose optimal solution length.
    let optimal = optimal.unwrap_or(random.gen_range(100..=10000));

    let mut case: Case = Case::new().with_cores(cores as u64);
    let mut schedule: Schedule = Schedule::new();

    // For each core
    for _ in 0..cores {
        // Create empty core.
        let mut core = Core::new();

        loop {
            // Until core is filled to `optimal` length add new task
            // with length no bigger than complement to `optimal`.
            let high = optimal - core.working_time() as u64;
            if high < 1 {
                break;
            }
            let length = random.gen_range(1..=high);
            core.add_task(Task::with_length(length));
        }

        schedule.add_core(core);
    }

    for core in schedule.cores() {
        case.add_tasks(core.get_tasks());
    }

    (case, schedule)
}

#[cfg(test)]
mod test_generator {
    use super::*;

    #[test]
    fn test_generate_random_case() {
        // Generates case with 5 cores and optimal solution 3400.
        let (_, schedule) = generate(5, Some(3400));

        assert_eq!(schedule.makespan().unwrap(), 3400);
    }
}
