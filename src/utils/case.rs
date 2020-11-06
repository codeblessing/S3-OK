use super::task::Task;
use rand::Rng;
use std::fs;
use std::io::Write;

/// Represents single test case consisting of cores count and list of Tasks to be scheduled.
#[derive(PartialEq, Debug)]
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
    pub fn with_cores(mut self, cores: u64) -> Self {
        self.cores = cores;
        self
    }

    /// Sets random core count for case from range 1 - u64::MAX.
    ///
    /// # Example
    ///
    /// ```
    /// let case = Case::new().with_random_cores();
    /// assert!(case.cores() > 0 && case.cores() <= u64::MAX);
    /// ```
    pub fn with_random_cores(mut self) -> Self {
        self.cores = rand::thread_rng().gen_range(1, u64::MAX);
        self
    }

    /// Sets task list for case. Tasks are created using `task_generator` closure.
    /// `task_generator` takes one argument - index of generated task.
    ///
    /// # Example
    ///
    /// ```
    /// let case = Case::new().with_cores(5).with_tasks(20, |i| { Task::new().with_length(i + 1) });
    /// assert_eq!(case.cores(), 5);
    /// assert_eq!(case.tasks().len(), 20);
    /// assert_eq!(case.tasks()[1].length(), 2);
    /// ```
    pub fn with_tasks<F: Fn(u64) -> Task>(mut self, task_count: u64, task_generator: F) -> Self {
        self.tasks = (0..task_count).map(|x| task_generator(x)).collect();
        self
    }

    /// Sets list of random tasks for case.
    ///
    /// # Example
    ///
    /// ```
    /// let case = Case::new().with_cores(5).with_task_count(20);
    /// assert_eq!(case.cores(), 5);
    /// assert_eq!(case.tasks().len(), 20);
    /// ```
    pub fn with_task_count(mut self, task_count: u64) -> Self {
        self.tasks = (0..task_count).map(|_| Task::random()).collect();
        self
    }

    /// Sets task list of random length with random tasks.
    pub fn with_random_tasks(mut self) -> Self {
        let count = rand::thread_rng().gen_range(1, u64::MAX);
        self.tasks = (0..count).map(|_| Task::random()).collect();
        self
    }

    /// Adds task to case's task list.
    ///
    /// # Example
    ///
    /// ```
    /// let mut case = Case::new();
    /// case.add_task(Task::random());
    /// assert_eq!(case.tasks().len(), 1);
    /// ```
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    /// Returns number of cores.
    pub fn cores(&self) -> u64 {
        self.cores
    }

    /// Returns immutable reference to list of tasks.
    pub fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    /// Returns lower bound of possible case's solutions
    /// (equals total time divided by nuber of cores).
    /// If lower bound is fraction it's rounded upward.
    pub fn lower_bound(&self) -> u128 {
        let total_length: u128 = self.tasks().iter().map(|task| task.length() as u128).sum();
        let remainder = total_length % self.cores as u128;
        match remainder {
            0 => total_length / self.cores as u128,
            _ => (total_length / self.cores as u128) + 1,
        }
    }

    /// Saves Case object as string to file in form:
    /// core count
    /// task count
    /// task 1 length
    /// task 2 length
    /// ...
    /// task n length
    pub fn save<T: ToString>(&self, path: T) -> std::io::Result<()> {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path.to_string())?;

        let content = self.tasks().iter().fold(
            format!("{}\n{}\n", self.cores(), self.tasks().len()),
            |acc, elem| format!("{}{}\n", acc, elem.length()),
        );

        file.write_all(content.as_bytes())?;

        Ok(())
    }

    /// Reads Case object from file in form:
    ///
    /// core count \
    /// task count \
    /// task 1 length\
    /// task 2 length\
    /// ...\
    /// task n length
    pub fn read<T: ToString>(path: T) -> Result<Case, std::io::Error> {
        let content = fs::read_to_string(path.to_string())?.trim().to_owned();

        let core_count: u64 = content
            .lines()
            .take(1)
            .map(|val| val.parse::<u64>().ok().unwrap_or(0))
            .next()
            .unwrap();

        let vals: Vec<Task> = content
            .lines()
            .skip(2)
            .filter_map(|line| match line.parse::<u64>().ok() {
                Some(val) => Some(Task::new().with_length(val)),
                None => None,
            })
            .collect();

        Ok(Self {
            cores: core_count,
            tasks: vals,
        })
    }
}
