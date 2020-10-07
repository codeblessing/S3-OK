use rand::{self, Rng};
#[derive(PartialEq, Debug)]
pub struct Task {
    length: u128
}

impl Task {
    pub fn new() -> Self {
        Self {
            length: 0
        }
    }

    pub fn random() -> Self {
        Self {
            length: rand::thread_rng().gen_range(1, std::u128::MAX)
        }
    }

    pub fn with_length(mut self, length: u128) -> Self {
        self.length = length;
        self
    }
}
#[derive(PartialEq, Debug)]
pub struct Case {
    cores: u128,
    tasks: Vec<Task>
}

impl Case {
    pub fn new() -> Self {
        Case {
            cores: 0,
            tasks: Vec::new()
        }
    }

    pub fn with_cores(mut self, cores: u128) -> Self
    {
        self.cores = cores;
        self
    }

    pub fn with_random_cores(mut self) -> Self {
        self.cores = rand::thread_rng().gen_range(1, std::u128::MAX);
        self
    }

    pub fn with_tasks<F: Fn(u128) -> Task>(mut self, task_count: u128, task_generator: F) -> Self {
        self.tasks = (0..task_count).map(|x|task_generator(x)).collect();
        self
    }

    pub fn with_task_count(mut self, task_count: u128) -> Self {
        self.tasks = (0..task_count).map(|_| Task::random()).collect();
        self
    }

    pub fn with_random_tasks(mut self) -> Self {
        let count = rand::thread_rng().gen_range(1, std::u128::MAX);
        self.tasks = (0..count).map(|_| Task::random()).collect();
        self
    }
}

#[cfg(test)]
mod test_case_generator {
    use super::*;

    #[test]
    fn test_create_empty() {
        assert_eq!(Case::new(), Case { cores: 0, tasks: Vec::new() });
    }

    #[test]
    fn test_create_with_cores() {
        assert_eq!(Case::new().with_cores(6).cores, 6);
    }

    #[test]
    fn test_create_with_task_count() {
        assert_eq!(Case::new().with_task_count(42).tasks.len(), 42);
    }

    #[test]
    fn test_create_with_tasks() {
        let test_case = Case::new().with_tasks(5, |i| Task::new().with_length(i + 1));
        let lengths: Vec<u128> = test_case.tasks.iter().map(|task| task.length).collect();
        assert_eq!(test_case.tasks.len(), 5);
        assert_eq!(lengths, vec![1, 2, 3, 4, 5]);
    }
}