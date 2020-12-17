use crate::greedy;
use crate::serializer::Serializer;
use crate::simulated_annealing::{Reduction, Solution};
use crate::utils::{Case, Schedule, Settings};
use std::{error::Error, fs::OpenOptions};
pub struct App {}

impl App {
    pub fn process(file: &str) -> Result<Schedule, Box<dyn Error>> {
        let settings = Settings::get().unwrap().read()?;
        let log_file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&settings.log_file)?;
        let mut serializer = Serializer::new(log_file);
        let case = Case::read_from_file(file)?;
        let initial = greedy::schedule(&case);

        let solution = Solution::new()
            .with_initial_solution(initial)
            .with_iterations_per_temperature(100)
            .with_reduction_rule(Reduction::Linear(5.0))
            .with_temperature(80.0)
            .run(&mut serializer);

        Ok(solution)
    }
}
