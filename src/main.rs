#![allow(dead_code)]
mod app;
mod greedy;

mod random;

mod io;
mod modules;
mod pretty_print;
mod serializer;
mod simulated_annealing;
mod utils;

use std::{
    error::Error,
    fs::{self, File, OpenOptions},
    path::Path,
};

use app::App;
use clap::load_yaml;
use std::io::Write;
use utils::Settings;

use modules::generator;

fn open_file(name: &str, dir: &str) -> Result<File, Box<dyn Error>> {
    fs::create_dir_all(dir)?;
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(format!("{}/{}", dir, name))?;

    Ok(file)
}

fn main() {
    let cli_settings = load_yaml!("settings.yaml");
    let app_args = clap::App::from_yaml(cli_settings).get_matches();

    match app_args.subcommand() {
        ("generate", Some(args)) => {
            let path = Path::new(args.value_of("output").unwrap());
            let filename = path.file_name().unwrap().to_str().unwrap();
            let dir = path.parent().unwrap().to_str().unwrap();

            let mut case_output = open_file(filename, dir).unwrap();
            let mut schedule_output = open_file(&format!("{}.schedule", filename), dir).unwrap();

            let cores = args.value_of("cores").unwrap().parse::<u16>().unwrap();
            let optimal = args
                .value_of("optimal")
                .and_then(|val| val.parse::<u64>().ok());

            let (case, schedule) = generator::generate(cores, optimal);

            let case_serialized = case.serialize();
            let schedule_serialized = schedule.serialize();

            case_output.write_all(case_serialized.as_bytes()).unwrap();
            schedule_output
                .write_all(schedule_serialized.as_bytes())
                .unwrap();
        }
        _ => (),
    }

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
        let path = Path::new(&settings.log_file);
        let filename = path.file_name().unwrap().to_str().unwrap();
        let dir = path.parent().unwrap().to_str().unwrap();

        open_file(filename, dir).unwrap();
    }

    for file in &settings.input_files {
        println!("Processing {}", file);
        match App::process(file) {
            Ok(schedule) => println!("SA solution: {}", schedule.makespan().unwrap()),
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
                Ok(schedule) => println!("SA solution: {}", schedule.makespan().unwrap()),
                Err(err) => eprintln!("An error occured during processing. {}", err),
            }
        }
    }
}
