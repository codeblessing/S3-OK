// #![allow(dead_code)]
use crate::utils::{Case, Core, Schedule};
use rand::Rng;

pub fn schedule(case: &Case) -> Schedule {
    let mut cpu: Vec<Core> = (0..case.cores()).map(|_| Core::new()).collect();

    let mut rng = rand::thread_rng();
    let tasks = case.tasks().to_owned();

    tasks.iter().for_each(|&task| {
        let core_idx = rng.gen_range(0..cpu.len());
        cpu[core_idx].add_task(task);
    });

    let mut schedule = Schedule::new();

    for core in cpu {
        schedule.add_core(core);
    }

    schedule
}
