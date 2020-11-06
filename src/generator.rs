use crate::greedy;
use crate::utils::{case::Case, task::Task};

use rand::Rng;

impl Case {
    /// Generates test case for PC||max problem with optimal solution.
    ///
    /// # Returns
    /// Tuple (case: Case, solution: u128) where
    /// case is generated test case and solution is optimal time.
    pub fn generate() -> (Self, u128) {
        let task_max_time : u32 = 1000;
        
        let mut rng = rand::thread_rng();
        let core_count = rng.gen_range(1, 1000);
        let task_count = rng.gen_range(core_count, 10000);
        let mut case = Case::new()
            .with_cores(core_count)
            .with_tasks(task_count - core_count, |_| {
                Task::new().with_length(rand::thread_rng().gen_range(1, task_max_time as u64))
            });

        let schedule = greedy::schedule(&case);

        let max_time = schedule.max_time() + rng.gen_range(1, task_max_time) as u128;

        schedule.cores().iter().for_each(|core| {
            case.add_task(Task::new().with_length((max_time - core.working_time()) as u64));
        });

        (case, max_time)
    }
}
