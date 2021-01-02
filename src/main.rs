#![allow(dead_code)]
mod app;
mod generator;
mod greedy;
mod io;
mod pretty_print;
mod serializer;
mod simulated_annealing;
mod utils;

use std::fs::OpenOptions;

use app::App;
use clap::load_yaml;
use utils::Settings;

fn main() {
    let cli_settings = load_yaml!("settings.yaml");
    let app_args = clap::App::from_yaml(cli_settings).get_matches();

    let settings = Settings {
        prompt: app_args.is_present("prompt"),
        unbuffered: app_args.is_present("unbuffered"),
        verbosity: app_args.occurrences_of("verbose") as u8,
        input_files: app_args
            .values_of("files")
            .and_then(|vals| Some(vals.map(ToString::to_string).collect::<Vec<String>>()))
            .unwrap_or(Vec::new()),
        log_file: app_args.value_of("logfile").unwrap().to_string(),
        kill_time: app_args.value_of("kill").unwrap().parse().unwrap(),
    };

    Settings::init(
        settings.prompt,
        settings.unbuffered,
        settings.verbosity,
        settings.input_files,
        settings.log_file,
        settings.kill_time,
    );

    let settings = Settings::get().unwrap().read().unwrap();

    // Clear file before usage
    {
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&settings.log_file)
            .unwrap();
    }

    for file in &settings.input_files {
        println!("Processing {}", file);
        match App::process(file) {
            Ok(schedule) => println!("SA solution: {}", schedule.makespan()),
            Err(err) => eprintln!("An error occured during processing. {}", err),
        }
    }

    if settings.prompt {
        loop {
            let file: String;

            println!("Enter path to data file (or type `exit` to end program): ");
            file = readln!();
            if file.eq_ignore_ascii_case("exit") {
                break;
            }

            println!("Processing {}", file);
            match App::process(&file) {
                Ok(schedule) => println!("SA solution: {}", schedule.makespan()),
                Err(err) => eprintln!("An error occured during processing. {}", err),
            }
        }
    }
}
