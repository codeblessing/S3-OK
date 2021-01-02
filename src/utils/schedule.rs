use std::error::Error;

use crate::utils::Core;
use serde::Serialize;
use serde_json;
#[derive(Clone, PartialEq, Debug, Serialize)]
pub struct Schedule {
    cores: Vec<Core>,
}

impl Schedule {
    pub fn new() -> Self {
        Self { cores: Vec::new() }
    }

    pub fn add_core(&mut self, core: Core) {
        self.cores.push(core);
    }

    pub fn cores(&self) -> &Vec<Core> {
        &self.cores
    }

    pub fn makespan(&self) -> u128 {
        self.cores
            .iter()
            .max_by(|x, y| x.working_time().cmp(&y.working_time()))
            .unwrap()
            .working_time()
    }

    pub fn serialize(&self) -> Result<String, Box<dyn Error>> {
        let serialized = serde_json::to_string(self)?;
        Ok(serialized)
    }
}

#[cfg(test)]
mod test_schedule {
    use super::*;
    use crate::utils::Task;

    #[test]
    fn test_create_empty() {
        let schedule = Schedule::new();
        assert!(schedule.cores.is_empty());
    }

    #[test]
    fn test_add_core() {
        let mut schedule = Schedule::new();
        schedule.add_core(Core::new());
        assert_eq!(schedule.cores.len(), 1);
    }

    #[test]
    fn test_get_cores() {
        let mut schedule = Schedule::new();
        schedule.add_core(Core::new());
        schedule.add_core(Core::new());
        schedule.add_core(Core::new());

        let cores = schedule.cores();

        assert_eq!(cores.len(), 3);
    }

    #[test]
    fn test_makespan() {
        let mut schedule = Schedule::new();
        let mut first_core = Core::new();
        first_core.add_task(Task::with_length(7));
        let mut second_core = Core::new();
        second_core.add_task(Task::with_length(8));
        let mut third_core = Core::new();
        third_core.add_task(Task::with_length(5));

        schedule.add_core(first_core);
        schedule.add_core(second_core);
        schedule.add_core(third_core);

        let makespan = schedule.makespan();

        assert_eq!(makespan, 8);
    }
}
