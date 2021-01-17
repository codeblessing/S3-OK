// #![allow(dead_code)]
use crate::utils::{Case, Core, Schedule};

pub fn schedule(case: &Case) -> Schedule {
    let mut cpu: Vec<Core> = (0..case.cores()).map(|_| Core::new()).collect();

    let mut tasks = case.tasks().to_owned();
    tasks.sort_unstable_by(|a, b| b.length().cmp(&a.length()));

    tasks.iter().for_each(|&task| {
        cpu.iter_mut()
            .min_by(|x, y| x.working_time().cmp(&y.working_time()))
            .unwrap()
            .add_task(task);
    });

    let mut schedule = Schedule::new();

    for core in cpu {
        schedule.add_core(core);
    }

    schedule
}

#[cfg(test)]
mod test_greedy_task_planning {
    use super::*;
    use crate::utils::Task;

    #[test]
    fn test_greedy_schedule() {
        let mut case = Case::new().with_cores(4);

        for i in 1..11 {
            case.add_task(Task::with_length(i));
        }

        let schedule = schedule(&case);

        assert_eq!(schedule.makespan().unwrap(), 15);
    }
}
