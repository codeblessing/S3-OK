use crate::greedy;
use crate::utils::{Case, Schedule, Task};

use rand::Rng;

impl Case {
    /// Generates test case with optimal solution for P||Cmax problem.
    pub fn generate(
        min_cores: u64,
        max_cores: u64,
        min_time: u64,
        max_time: u64,
        max_task_count: u64,
    ) -> (Self, Schedule) {
        let mut rng = rand::thread_rng();

        let core_count = rng.gen_range(min_cores, max_cores);
        let task_count = rng.gen_range(core_count, max_task_count);
        let mut case = Case::generate_random(core_count, task_count, min_time, max_time);

        let partial = greedy::schedule(&case);

        let makespan = partial.makespan() + rng.gen_range(min_time, max_time) as u128;

        let mut schedule = Schedule::new();

        for i in 0..core_count as usize {
            let mut core = partial.cores()[i].to_owned();
            let length = (makespan - core.working_time()) as u64;
            case.add_task(Task::new().with_length(length));
            core.add_task(Task::new().with_length(length));

            schedule.add_core(core);
        }

        (case, schedule)
    }

    fn generate_random(core_count: u64, task_count: u64, min_time: u64, max_time: u64) -> Self {
        let mut rng = rand::thread_rng();
        let mut case = Case::new().with_cores(core_count).to_owned();

        for _ in 0..task_count {
            let length = rng.gen_range(min_time, max_time);
            case.add_task(Task::new().with_length(length));
        }

        case
    }
}

#[cfg(test)]
mod test_generator {
    use super::*;

    #[test]
    fn test_generate_random_cores() {
        let case = Case::generate_random(10, 15, 0, 25);

        assert_eq!(case.cores(), 10);
    }

    #[test]
    fn test_generate_random_task_count() {
        let case = Case::generate_random(1, 15, 1, 10);

        assert_eq!(case.tasks().len(), 15);
    }

    #[test]
    fn test_generate_random_task_lengths() {
        let case = Case::generate_random(1, 5, 1, 10);

        for task in case.tasks() {
            assert!(task.length() > 0 && task.length() < 10);
        }
    }
}
