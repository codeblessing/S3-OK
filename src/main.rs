#![allow(dead_code)]
mod generator;
mod greedy;
mod io;
mod pretty_print;
mod simulated_annealing;
mod utils;
mod serializer;

use std::fs::OpenOptions;

use clap::load_yaml;
use simulated_annealing::{Reduction, Solution};
use utils::{Case, Settings};

fn main() {
    let cli_settings = load_yaml!("settings.yaml");
    let app_args = clap::App::from_yaml(cli_settings).get_matches();

    let settings = Settings {
        prompt: app_args.is_present("prompt"),
        verbosity: app_args.occurrences_of("verbose") as u8,
        input_files: app_args.values_of("files").and_then(|vals| Some(vals.map(|val| val.to_string()).collect::<Vec<String>>())).unwrap_or(Vec::new()),
        log_file: app_args.value_of("logfile").unwrap_or("data.log").to_string(),
        kill_time: app_args.value_of("kill").unwrap_or("3000").parse().unwrap()
    };

    Settings::init(settings.prompt, settings.verbosity, settings.input_files, settings.log_file, settings.kill_time);

    loop {
        let filename: String;
        if settings.prompt {
            println!("Enter path to data file (or type `exit` to end program): ");
            filename = readln!();
            if filename.eq_ignore_ascii_case("exit") {
                break;
            }

            let case = Case::read_from_file(filename).unwrap();

            let greedy = greedy::schedule(&case);

            let mut solution: Solution = Solution::new()
                .with_final_temperature(0.0)
                .with_temperature(100.0)
                .with_iterations_per_temperature(20)
                .with_reduction_rule(Reduction::Linear(1.0))
                .with_initial_solution(greedy.clone());

            let log_file = OpenOptions::new().write(true).create(true).open("data.log").unwrap();
            let sa_solution = solution.run(log_file);

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
