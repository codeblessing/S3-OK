#![allow(dead_code)]
use crate::utils::{case::Case, core::Core};

pub struct Schedule {
    cores: Vec<Core>,
}

impl Schedule {
    pub fn max_time(&self) -> u128 {
        self.cores
            .iter()
            .max_by(|x, y| x.working_time().cmp(&y.working_time()))
            .unwrap()
            .working_time()
    }

    pub fn cores(&self) -> &Vec<Core> {
        &self.cores
    }
}

pub fn schedule(tasks: &Case) -> Schedule {
    let mut cpu: Vec<Core> = (0..tasks.cores()).map(|_| Core::new()).collect();

    tasks.tasks().iter().for_each(|task| {
        // For each task: find core with shortest working time and assign task to this core.
        cpu.iter_mut()
            .min_by(|x, y| x.working_time().cmp(&y.working_time()))
            .unwrap()
            .add(task.length());
    });

    Schedule { cores: cpu }
}

#[cfg(test)]
mod test_greedy_task_planning {
    use super::*;
    use crate::utils::task::Task;

    #[test]
    fn test_core_add_time() {
        assert_eq!(Core::new().add(15).working_time(), 15);
    }

    #[test]
    fn test_greedy_schedule() {
        let case = Case::new()
            .with_cores(4)
            .with_tasks(10, |i| Task::new().with_length((i + 5) % 7 + 1));

        let result = schedule(&case).max_time();

        assert_eq!(result, 13);
    }
}
