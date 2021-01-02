use crate::utils::task::Task;
use serde::{Serialize, Deserialize};

/// Represent single core (processor) on which task times are scheduled.
#[derive(Clone, PartialEq, Debug, Serialize)]
pub struct Core {
    #[serde(rename = "tasks")]
    timeline: Vec<Task>,
    #[serde(skip)]
    working_time: u128,
}

impl Core {
    /// Creates new Core object with empty timeline (no task scheduled on it).
    pub fn new() -> Self {
        Self {
            timeline: Vec::new(),
            working_time: 0,
        }
    }

    /// Schedules `task` on core.
    pub fn add_task(&mut self, task: Task) {
        self.working_time += task.length() as u128;
        self.timeline.push(task);
    }

    /// Returns immutable reference to core's timeline.
    pub fn timeline(&self) -> &Vec<Task> {
        &self.timeline
    }

    /// Returns total length of core's schedule.
    pub fn working_time(&self) -> u128 {
        self.working_time
    }
}

impl<T> From<T> for Core where T: Into<Vec<Task>> {
    fn from(tasks: T) -> Self {
        let tasks: Vec<Task> = tasks.into();
        let time: u128 = tasks.iter().map(|task| task.length() as u128).sum();
        Self {
            timeline: tasks,
            working_time: time
        }
    }
}

#[cfg(test)]
mod test_core {
    use super::*;

    #[test]
    fn test_create_empty() {
        let core = Core::new();
        assert_eq!(core.timeline, Vec::new());
        assert_eq!(core.working_time, 0);
    }

    #[test]
    fn test_add_task() {
        let mut core = Core::new();
        core.add_task(Task::with_length(2));
        assert_eq!(core.timeline.len(), 1);
        assert_eq!(core.working_time, 2);
    }

    #[test]
    fn test_get_timeline() {
        let mut core = Core::new();
        core.add_task(Task::with_length(3));
        core.add_task(Task::with_length(5));
        core.add_task(Task::with_length(7));

        let lengths: Vec<u64> = core.timeline().iter().map(|task| task.length()).collect();
        assert_eq!(lengths, vec![3, 5, 7]);
    }

    #[test]
    fn test_get_working_time() {
        let mut core = Core::new();
        core.add_task(Task::with_length(3));
        core.add_task(Task::with_length(5));

        assert_eq!(core.working_time(), 8);
    }
}
