use super::task::Task;

/// Represents single test case consisting of cores count and list of Tasks to be scheduled.
#[derive(PartialEq, Debug, Clone)]
pub struct Case {
    cores: u64,
    tasks: Vec<Task>,
}

impl Case {
    /// Creates new Case object with no cores and empty task list.
    pub fn new() -> Self {
        Case {
            cores: 0,
            tasks: Vec::new(),
        }
    }

    /// Sets core count for case.
    ///
    /// # Example
    ///
    /// ```
    /// let case = Case::new().with_cores(5);
    /// assert_eq!(case.cores(), 5);
    /// ```
    pub fn with_cores(&mut self, cores: u64) -> &mut Self {
        self.cores = cores;
        self
    }

    /// Adds `task` to case's task list.
    ///
    /// # Example
    ///
    /// ```
    /// let mut case = Case::new();
    /// case.add_task(Task::new().with_length(5));
    /// assert_eq!(case.tasks().len(), 1);
    /// assert_eq!(case.tasks()[0].length(), 5);
    /// ```
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    /// Adds vector of tasks to case's task list.
    ///
    /// # Example
    ///
    /// ```
    /// let mut tasks = Vec::new();
    /// for i in 1..6
    /// {
    ///     tasks.push(Task::new().with_length(i));
    /// }
    /// let mut case = Case::new();
    /// case.add_tasks(tasks);
    /// assert_eq!(case.tasks().len(), 5);
    /// ```
    pub fn add_tasks(&mut self, mut tasks: Vec<Task>) {
        self.tasks.append(&mut tasks);
    }

    /// Returns number of cores available.
    ///
    /// # Example
    ///
    /// ```
    /// let case = Case::new().with_cores(4).to_owned();
    /// assert_eq!(case.cores(), 4);
    /// ```
    pub fn cores(&self) -> u64 {
        self.cores
    }

    /// Returns immutable reference to case's task list.
    pub fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
}

#[cfg(test)]
mod test_case {
    use super::*;

    #[test]
    fn test_create_empty() {
        let case = Case::new();
        assert_eq!(case.cores, 0);
        assert!(case.tasks.is_empty());
    }

    #[test]
    fn test_create_with_cores() {
        let case = Case::new().with_cores(3).to_owned();
        assert_eq!(case.cores, 3);
    }

    #[test]
    fn test_add_task() {
        let mut case = Case::new();
        case.add_task(Task::with_length(3));
        assert_eq!(case.tasks.len(), 1);
        assert_eq!(case.tasks[0].length(), 3);
    }

    #[test]
    fn test_add_tasks() {
        let mut case = Case::new();
        let tasks: Vec<Task> = (0..9).map(|l| Task::with_length(l)).collect();

        case.add_tasks(tasks);
    }

    #[test]
    fn test_get_cores() {
        let case = Case::new().with_cores(5).to_owned();
        assert_eq!(case.cores(), 5);
    }

    #[test]
    fn test_get_tasks() {
        let mut case = Case::new();
        case.add_task(Task::with_length(3));
        case.add_task(Task::with_length(5));
        case.add_task(Task::with_length(8));

        let lengths: Vec<u64> = case.tasks().iter().map(|task| task.length()).collect();

        assert_eq!(lengths, vec![3, 5, 8]);
    }
}
