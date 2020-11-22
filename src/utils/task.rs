use rand::Rng;

/// Represents single, indivisible task, which takes `length` time to complete.
#[derive(PartialEq, Debug, Clone, Copy)]
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

    /// Creates new Task object with random length from range [min; max),
    /// i.e. inclusive of `min` and exclusive of `max`.
    ///
    /// # Example
    ///
    /// ```
    /// let task = Task::new().from_range(1, 64);
    /// assert!(task.length() > 0 && task.length() < 64);
    /// ```
    pub fn from_range(mut self, min: u64, max: u64) -> Self {
        self.length = rand::thread_rng().gen_range(min, max);
        self
    }

    /// Returns Task length.
    ///
    /// # Example
    ///
    /// ```
    /// let task = Task::new().with_length(5);
    /// assert_eq!(task.length(), 5);
    /// ```
    pub fn length(&self) -> u64
    {
        return self.length;
    }
}

#[cfg(test)]
mod test_task {
    use super::*;

    #[test]
    fn test_create_empty() {
        let task = Task::new();
        assert_eq!(task.length, 0);
    }

    #[test]
    fn test_create_with_length() {
        let task = Task::new().with_length(2);
        assert_eq!(task.length, 2);
    }

    #[test]
    fn test_create_from_range() {
        let task = Task::new().from_range(0, 64);
        assert!(task.length > 0 && task.length < 64);
    }

    #[test]
    fn test_get_length() {
        let task = Task::new().with_length(5);
        assert_eq!(task.length, task.length());
    }
}