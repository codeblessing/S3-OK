#![allow(dead_code)]
mod utils;
mod generator;
mod greedy;
mod pretty_print;
mod io;
mod simulated_annealing;

use utils::Case;

// use utils::case::Case;

fn main() {

    let files: Vec<&str> = vec![
        "./instances/m25.txt",
        "./instances/m50.txt",
        "./instances/m50n200.txt",
        "./instances/m10n200.txt",
        "./instances/m50n1000.txt"
    ];

    for name in files {

        let case = Case::read_from_file(name).unwrap();

        let solution = greedy::schedule(&case);

        // solution.print();
        println!("{}", solution.makespan());
    }
}
