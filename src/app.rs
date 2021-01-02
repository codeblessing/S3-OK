use crate::greedy;
use crate::serializer::Serializer;
use crate::simulated_annealing::{Reduction, SimulatedAnnealingParams, Solution};
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
        let params = SimulatedAnnealingParams {
            initial_solution: initial,
            initial_temperature: 100.0,
            final_temperature: 0.0,
            reduction_rule: Reduction::SlowDecrease(0.5),
            iterations_per_temperature: 100,
            max_simulation_time: settings.kill_time,
            max_changeless_iterations: 500,
        };

        let solution = Solution::new(params).run(&mut serializer);

        Ok(solution)
    }
}
