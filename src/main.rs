#![allow(dead_code)]
mod generator;
mod greedy;
mod io;
mod pretty_print;
mod simulated_annealing;
mod utils;

use clap::load_yaml;
use utils::Case;

// use utils::case::Case;

fn main() {
    let cli_settings = load_yaml!("settings.yaml");
    let app_args = clap::App::from_yaml(cli_settings).get_matches();

    loop {
        let filename: String;
        if app_args.is_present("prompt") {
            println!("Enter path to data file (or type `exit` to end program): ");
            filename = readln!();
            print!("Filename: {}", filename);
            if filename.eq_ignore_ascii_case("exit") {
                dbg!("Typed `exit`");
                break;
            }

            let case = Case::read_from_file(filename).unwrap();

            let solution = greedy::schedule(&case);

            solution.print();
            println!("{}", solution.makespan());
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
