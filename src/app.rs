use crate::{greedy, random};
use crate::serializer::Serializer;
use crate::simulated_annealing::{Reduction, SimulatedAnnealingParams, Solution};
use crate::utils::{Case, Schedule, Settings};
use std::{error::Error, fs::{self, OpenOptions}, path::Path};
pub struct App {}

impl App {
    pub fn process(file: &str) -> Result<Schedule, Box<dyn Error>> {
        let settings = Settings::get().unwrap().read()?;
        fs::create_dir_all(Path::new(&settings.log_file).parent().unwrap_or(Path::new("./logs")))?;
        let log_file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&settings.log_file)?;
        let mut serializer = Serializer::new(log_file);
        serializer.buffered(!settings.unbuffered);

        let case    = Case::read_from_file(file)?;
        //let initial = greedy::schedule(&case);
        let initial = random::schedule(&case);

        //println!("Greedy solution: {}", initial.makespan());
        let params = SimulatedAnnealingParams {
            initial_solution: initial,
            initial_temperature: 75.0,
            final_temperature: 0.01,
            reduction_rule: Reduction::Geometric(0.99),
            iterations_per_temperature: 20,
            max_simulation_time: 120,
            //max_simulation_time: settings.kill_time,
            max_changeless_iterations: 12000,
        };

        let solution = Solution::new(params).run(&mut serializer);

        Ok(solution)
    }
}
