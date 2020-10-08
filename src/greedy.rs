use crate::generator::{Case, Task};

struct Core {
    working_time: u128,
}

impl Core {
    fn new() -> Self {
        Self { working_time: 0 }
    }

    fn add(&mut self, time: u128) -> &mut Self {
        self.working_time += time;
        self
    }
}

pub fn schedule(tasks: &Case) -> u128 {
    let mut cpu: Vec<Core> = (0..tasks.cores()).map(|_| Core::new()).collect();

    tasks.tasks().iter().for_each(|task| {
        // For each task: find core with shortest working time and assign task to this core.
        cpu.iter_mut()
            .min_by(|x, y| x.working_time.cmp(&y.working_time))
            .unwrap()
            .add(task.length());
    });

    // Return the longest time
    cpu.iter()
        .max_by(|x, y| x.working_time.cmp(&y.working_time))
        .unwrap()
        .working_time
}

#[cfg(test)]
mod test_greedy_task_planning {
    use super::*;

    #[test]
    fn test_core_add_time() {
        assert_eq!(Core::new().add(15).working_time, 15);
    }

    #[test]
    fn test_greedy_schedule() {
        let case = Case::new()
            .with_cores(4)
            .with_tasks(10, |i| Task::new().with_length((i + 5) % 7 + 1));

        let result = schedule(&case);

        assert_eq!(result, 13);
    }
}
