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
        serializer.buffered(!settings.unbuffered);

        let case = Case::read_from_file(file)?;
        let initial = greedy::schedule(&case);

        println!("Greedy solution: {}", initial.makespan());
        let solution = Solution::new()
            .with_initial_solution(initial)
            .with_iterations_per_temperature(100)
            .with_reduction_rule(Reduction::Linear(0.5))
            .with_temperature(200.0)
            .run(&mut serializer);

        Ok(solution)
    }
}
