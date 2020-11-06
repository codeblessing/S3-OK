use rand::Rng;

/// Represents single, indivisible task, which takes `length` time to complete.
#[derive(PartialEq, Debug)]
pub struct Task {
    length: u64
}

impl Task {
    /// Creates new Task object with length 0.
    ///
    /// # Example
    ///
    /// ```
    /// let task = Task::new();
    /// assert_eq!(task.length(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            length: 0
        }
    }

    /// Creates new Task object with random length from range 1 - u64::MAX
    ///
    /// # Example
    ///
    /// ```
    /// let task = Task::random();
    /// assert!(task.length() > 0 && task.length() <= u64::MAX);
    /// ```
    pub fn random() -> Self {
        Self {
            length: rand::thread_rng().gen_range(1, u64::MAX)
        }
    }

    /// Sets up length of the task object.
    ///
    /// # Example
    ///
    /// ```
    /// let task = Task::new().with_length(30);
    /// assert_eq!(task.length(), 30);
    /// ```
    pub fn with_length(mut self, length: u64) -> Self {
        self.length = length;
        self
    }

    /// Returns the length of the task.
    ///
    /// # Example
    ///
    /// ```
    /// let task = Task::new().with_length(50);
    /// assert!(task.length() == 50);
    /// ```
    pub fn length(&self) -> u64 {
        self.length
    }
}