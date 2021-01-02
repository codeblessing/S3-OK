use crate::utils::{Case, Task};
use std::fs;
use std::io::Write;
use std::num::ParseIntError;
use std::str::FromStr;

impl ToString for Case {
    fn to_string(&self) -> String {
        return self
            .tasks()
            .iter()
            .fold(
                format!("{}\n{}\n", self.cores(), self.tasks().len()),
                |val, cur| format!("{}{}\n", val, cur.length()),
            )
            .trim_end()
            .to_owned();
    }
}

impl FromStr for Case {
    type Err = ParseIntError;

    fn from_str(serialized: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = serialized.lines().collect();

        let cores = lines[0].parse::<u64>()?;
        let mut tasks: Vec<Task> = Vec::new();
        for line in lines.iter().skip(2) {
            let length = line.parse::<u64>()?;
            tasks.push(Task::with_length(length));
        }

        let mut case = Case::new();
        case.with_cores(cores);
        case.add_tasks(tasks);

        Ok(case)
    }
}

impl Case {
    pub fn save_to_file<P>(&self, path: P) -> Result<(), std::io::Error>
    where
        P: Into<String>,
    {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(path.into())?;
        let case = self.to_string();

        file.write_all(case.as_bytes())?;

        Ok(())
    }

    pub fn read_from_file<P>(path: P) -> Result<Case, Box<dyn std::error::Error>>
    where
        P: Into<String>,
    {
        let serialized = fs::read_to_string(path.into())?.trim().to_owned();
        let case = Case::from_str(&serialized)?;

        Ok(case)
    }
}

#[cfg(test)]
mod test_case_io {
    use super::*;
    use crate::utils::Task;

    #[test]
    fn test_case_to_string() {
        let mut case = Case::new();
        case.add_task(Task::with_length(3));
        case.add_task(Task::with_length(6));
        case.add_task(Task::with_length(10));

        assert_eq!(case.to_string(), String::from("0\n3\n3\n6\n10"));
    }

    #[test]
    fn test_case_from_str() {
        let serialized = "2\n5\n4\n6\n9\n1\n3";
        let case = Case::from_str(serialized).unwrap_or(Case::new());

        let task_lengths: Vec<u64> = case.tasks().iter().map(|task| task.length()).collect();

        assert_eq!(case.cores(), 2);
        assert_eq!(case.tasks().len(), 5);
        assert_eq!(task_lengths, vec![4, 6, 9, 1, 3]);
    }
}