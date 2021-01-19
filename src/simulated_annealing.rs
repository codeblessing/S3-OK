use crate::{
    serializer::{Record, Serializer},
    utils::{Core, Schedule},
};
use rand::{seq::IteratorRandom, Rng};
use std::time::Instant;

/// Temperature reduction rule used by evaluation algorithm.\
/// Linear(α): `T = T - α`, in that one α have to be positive\
/// Geometric(α): `T = T * α`, in that one α have to be from range (0;1)\
/// Slow Decrease(β): `T = T / (1 + βT)`
#[derive(PartialEq, Debug)]
pub enum Reduction {
    Linear(f64),
    Geometric(f64),
    SlowDecrease(f64),
}

pub struct SimulatedAnnealingParams {
    pub(crate) initial_solution: Schedule,
    pub(crate) initial_temperature: f64,
    pub(crate) final_temperature: f64,
    pub(crate) reduction_rule: Reduction,
    pub(crate) iterations_per_temperature: u16,
    pub(crate) max_changeless_iterations: u16,
    pub(crate) max_simulation_time: u16,
}

/// Simulated Annealing implementation.
pub struct Solution {
    params: SimulatedAnnealingParams,
}

impl Solution {
    pub fn new(params: SimulatedAnnealingParams) -> Self {
        Self { params }
    }

    pub fn run<T: std::io::Write>(&mut self, serializer: &mut Serializer<T>) -> Schedule {
        let mut rng = rand::thread_rng();

        let mut current_solution = self.params.initial_solution.clone();
        let mut best_solution = self.params.initial_solution.clone();

        let mut current_temperature = self.params.initial_temperature;
        let mut iteration: u64 = 1;

        let mut changeless_iterations = 0u16;

        let timer = Instant::now();
        while !self.should_terminate(current_temperature, &timer, changeless_iterations) {
            for _ in 0..self.params.iterations_per_temperature {
                // Generate neighborhood and choose one of neighbors.
                let neighbors = gen_neighbours(&current_solution, 20, current_temperature);
                // FOR DEBUG PURPOSES:
                // if iteration % 50 == 0 {
                //     println!("---");
                //     for schedule in &neighbors {
                //         println!("{}", schedule.makespan());
                //     }
                // }
                let neighbor = neighbors.iter().choose(&mut rng).unwrap().to_owned();
                
                // let delta = neighbor.makespan() as f64 - best_solution.makespan() as f64;
                // it actually outputs better results
                let delta = neighbor.makespan() as f64 - current_solution.makespan() as f64;

                if delta < 0.0 {
                    current_solution = neighbor;
                    changeless_iterations = 0;
                } else {
                    if rng.gen::<f64>() < (-1.0 * delta / current_temperature).exp() {
                        current_solution = neighbor;
                        changeless_iterations = 0;
                    } else {
                        changeless_iterations += 1;
                    }
                }
                if current_solution.makespan() < best_solution.makespan() {
                    best_solution = current_solution.clone();
                }

                serializer.add_record(Record::new(iteration, current_solution.makespan()));
                iteration += 1;
            }
            current_temperature = self.reduce_temperature(current_temperature);
        }
        serializer.save("---\n").unwrap();
        best_solution
    }

    fn should_terminate(
        &self,
        temperature: f64,
        timer: &Instant,
        changeless_iterations: u16,
    ) -> bool {
        if temperature <= self.params.final_temperature
            || timer.elapsed().as_secs() > self.params.max_simulation_time.into()
            || changeless_iterations >= self.params.max_changeless_iterations
        {
            return true;
        }
        false
    }

    fn reduce_temperature(&self, temperature: f64) -> f64 {
        match self.params.reduction_rule {
            Reduction::Linear(alpha) => self.linear_decrease(alpha, temperature),
            Reduction::Geometric(alpha) => self.geometric_decrease(alpha, temperature),
            Reduction::SlowDecrease(beta) => self.slow_decrease(beta, temperature),
        }
    }

    fn linear_decrease(&self, alpha: f64, temperature: f64) -> f64 {
        temperature - alpha
    }

    fn geometric_decrease(&self, alpha: f64, temperature: f64) -> f64 {
        temperature * alpha
    }

    fn slow_decrease(&self, beta: f64, temperature: f64) -> f64 {
        temperature / (1.0 + beta * temperature)
    }
}

pub fn gen_neighbours(schedule: &Schedule, count: u8, temp: f64) -> Vec<Schedule> {
    let mut neighbours: Vec<Schedule> = Vec::new();
    for _ in 0..count {
        if let Some(neighbour) = calc_neighbour(schedule, temp) {
            neighbours.push(neighbour);
        }
    }
    neighbours
}

pub fn calc_neighbour(initial: &Schedule, temp: f64) 
                                -> Option<Schedule> {
    let mut schedule = initial.clone();
    for _ in 0..((temp.log(5.0) + 1.0) as u64) {
        if let Some(neighbour) = neighbour(&schedule) {
            schedule = neighbour.to_owned();
        } else {
            return None;
        }
    }
    Some(schedule)
}

pub fn neighbour(initial: &Schedule) -> Option<Schedule> {
    let mut rng = rand::thread_rng();
    let mut cores = initial.cores().to_owned();

    if cores.len() < 2 {
        return None;
    }

    let mut max_time = 0;
    let mut min_time = u128::MAX;

    let mut fci = 0;
    let mut sci = 0;
    let alpha   = 0.60;
    
    // first core index
    if rng.gen::<f64>() > alpha {
        while cores[fci].get_tasks().len() == 0 {
            fci = rng.gen_range(0..cores.len());
        }
    } else {
        for i in 0..cores.len() {
           if cores[i].working_time() > max_time {
               max_time = cores[i].working_time();
               fci = i;
           }
        }
    }
    let mut fc_tasks = cores.remove(fci).get_tasks().to_owned();

    // second core index
    if rng.gen::<f64>() > alpha {
        sci = rng.gen_range(0..cores.len());
    } else {
        for i in 0..cores.len() {
            if cores[i].working_time() < min_time {
                min_time = cores[i].working_time();
                sci = i;
            }
        }
    }
    let mut sc_tasks = cores.remove(sci).get_tasks().to_owned();

    // gen random task index and move it to a second core
    min_time = u128::MAX;
    let mut fti = 0;
    if rng.gen::<f64>() > alpha {
        fti = rng.gen_range(0..fc_tasks.len());
    } else {
        for i in 0..fc_tasks.len() {
            if fc_tasks[i].length() < min_time as u64 {
                min_time = fc_tasks[i].length() as u128;
                fti = i;
            }
        }
    }
    let first_task = fc_tasks.remove(fti);
    sc_tasks.push(first_task);

    cores.push(Core::from(fc_tasks));
    cores.push(Core::from(sc_tasks));

    let mut schedule = Schedule::new();
    for core in cores {
        schedule.add_core(core);
    }

    Some(schedule)
}

#[cfg(test)]
mod test_simulated_annealing {
    use crate::utils::Task;

    use super::*;

    #[test]
    fn test_neighbour_cores() {
        let mut initial = Schedule::new();

        let mut first_core = Core::new();
        let mut second_core = Core::new();

        first_core.add_task(Task::with_length(1));
        second_core.add_task(Task::with_length(3));

        initial.add_core(first_core);
        assert_eq!(neighbour(&initial).to_owned().is_some(), false);

        initial.add_core(second_core);
        assert_eq!(neighbour(&initial).to_owned().is_some(), true);
    }
}
