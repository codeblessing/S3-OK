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
                let neighbors = gen_neighbours(&current_solution, 200);
                // FOR DEBUG PURPOSES:
                // if iteration == 1 || iteration == 20 {
                //     println!("---");
                //     for schedule in &neighbors {
                //         println!("{}", schedule.makespan());
                //     }
                // }
                let neighbor = neighbors.iter().choose(&mut rng).unwrap().to_owned();

                // Calculate delta between best timed schedule and neighbor.
                // The reason for comparing with best time is simple:
                // if our delta will differ a lot from best solution then
                // chances to approve it will dive. If we'd been comparing with
                // current solution then we could worse makespans step by step
                // instead of improving them.
                let delta = (neighbor.makespan() - best_solution.makespan()) as f64;

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

pub fn gen_neighbours(schedule: &Schedule, count: u8) -> Vec<Schedule> {
    let mut neighbours: Vec<Schedule> = Vec::new();
    for _ in 0..count {
        if let Some(neighbour) = neighbour(schedule) {
            neighbours.push(neighbour);
        }
    }
    neighbours
}

pub fn neighbour(initial: &Schedule) -> Option<Schedule> {
    let mut rng = rand::thread_rng();
    let mut cores = initial.cores().to_owned();

    if cores.len() < 2 {
        return None;
    }

    // first core index
    let fci = rng.gen_range(0..cores.len());
    let mut fc_tasks = cores.remove(fci).get_tasks().to_owned();

    // second core index
    let sci = rng.gen_range(0..cores.len());
    let mut sc_tasks = cores.remove(sci).get_tasks().to_owned();

    // random task indices
    let fti = rng.gen_range(0..fc_tasks.len());
    let sti = rng.gen_range(0..sc_tasks.len());

    let first_task = fc_tasks.remove(fti);
    let second_task = sc_tasks.remove(sti);

    fc_tasks.push(second_task);
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

    /*
    #[test]
    fn test_neighbour_difference() {
        let mut count   = 0;
        let mut initial = Schedule::new();

        let mut first_core  = Core::new();
        let mut second_core = Core::new();
        let mut third_core  = Core::new();

        first_core.add_task(crate::utils::Task::new().with_length(1));
        first_core.add_task(crate::utils::Task::new().with_length(1));

        second_core.add_task(crate::utils::Task::new().with_length(2));
        second_core.add_task(crate::utils::Task::new().with_length(2));

        third_core.add_task(crate::utils::Task::new().with_length(3));
        third_core.add_task(crate::utils::Task::new().with_length(3));

        initial.add_core(first_core);
        initial.add_core(second_core);
        initial.add_core(third_core);

        let case = neighbour(&initial);

        match case {
            Some(test) =>,
            None =>
                count = 0
        }
        assert_eq!(count, 2);
    }
    */
}
