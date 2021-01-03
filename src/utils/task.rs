use rand::Rng;
use serde::Serialize;
use std::ops::Range;

/// Represents single, indivisible task, which takes `length` time to complete.
#[derive(PartialEq, Debug, Clone, Copy, Serialize)]
pub struct Task (u64);

impl Task {
    /// Sets up length of the task object.
    ///
    /// # Example
    ///
    /// ```
    /// // This is the same as `Task(30)`:
    /// let task = Task::with_length(30);
    /// assert_eq!(task.length(), 30);
    /// ```
    pub fn with_length(length: u64) -> Self {
        Self(length)
    }

    /// Creates new Task object with random length from range [min; max),
    /// i.e. inclusive of `min` and exclusive of `max`.
    ///
    /// # Example
    ///
    /// ```
    /// let task = Task::from_range(1..64);
    /// assert!(task.length() > 0 && task.length() < 64);
    /// ```
    pub fn from_range(range: Range<u64>) -> Self {
        Self(rand::thread_rng().gen_range(range))
    }

    /// Returns Task length.
    pub fn length(&self) -> u64
    {
        return self.0;
    }
}

#[cfg(test)]
mod test_task {
    use super::*;

    #[test]
    fn test_create_with_length() {
        let task = Task::with_length(2);
        assert_eq!(task.length(), 2);
    }

    #[test]
    fn test_create_from_range() {
        let task = Task::from_range(1..64);
        assert!(task.length() > 0 && task.length() < 64);
    }

    #[test]
    fn test_get_length() {
        let task = Task::with_length(5);
        assert_eq!(task.0, task.length());
    }

    #[test]
    fn test_serialize() {
        let serialized = serde_json::to_string(&Task(84)).unwrap();
        println!("{}", serialized);
    }
}
