#![allow(dead_code)]
mod generator;
mod greedy;
mod io;
mod pretty_print;
mod simulated_annealing;
mod utils;

use clap::load_yaml;
use simulated_annealing::{Reduction, Solution};
use utils::Case;

fn main() {
    let cli_settings = load_yaml!("settings.yaml");
    let app_args = clap::App::from_yaml(cli_settings).get_matches();

    loop {
        let filename: String;
        if app_args.is_present("prompt") {
            println!("Enter path to data file (or type `exit` to end program): ");
            filename = readln!();
            if filename.eq_ignore_ascii_case("exit") {
                dbg!("Typed `exit`");
                break;
            }

            let case = Case::read_from_file(filename).unwrap();

            let greedy = greedy::schedule(&case);

            let mut solution: Solution = Solution::new()
                .with_final_temperature(0.0)
                .with_temperature(80.0)
                .with_iterations_per_temperature(20)
                .with_reduction_rule(Reduction::Linear(5.0))
                .with_initial_solution(greedy.clone());

            let sa_solution = solution.run();

            println!("Initial solution: {}\nSA solution: {}", greedy.makespan(), sa_solution.makespan());
        }
    }
    // let files: Vec<&str> = vec![
    //     "./instances/m25.txt",
    //     "./instances/m50.txt",
    //     "./instances/m50n200.txt",
    //     "./instances/m10n200.txt",
    //     "./instances/m50n1000.txt"
    // ];

    // for name in files {

    //     let case = Case::read_from_file(name).unwrap();

    //     let solution = greedy::schedule(&case);

    //     // solution.print();
    //     println!("{}", solution.makespan());
    // }
}
