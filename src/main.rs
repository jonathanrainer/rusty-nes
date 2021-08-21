use std::path::PathBuf;

use clap::{App, load_yaml};
use log4rs;
use log::info;

use crate::cpu::Processor;
use crate::memory::{Memory, MemoryInitialiser};

mod cpu;
mod memory;

fn main() {
    // Initialise Logging
    log4rs::init_file(get_config_file_path("log4rs.yaml").as_path(),
                      Default::default()).unwrap();
    info!("RustyNES - Booting...");

    // Parse command line args
    let yaml = load_yaml!("../config/cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    // Initialise CPU
    let cpu = Processor::new(initialise_memory(
        matches.value_of("SOURCE_MODE").unwrap(),
        matches.value_of("SOURCE").unwrap())
    );

    //cpu.start();
    info!("RustyNES - Shutting Down...");
}

fn get_config_file_path(filename: &str) -> PathBuf {
    let mut config_path = PathBuf::new();
    config_path.push(env!("CARGO_MANIFEST_DIR"));

    let mut result = PathBuf::from(config_path);
    result.push("config");
    result.push(filename);
    result
}

fn initialise_memory(source_mode: &str, source: &str) -> Memory {
    let mem = match source_mode {
        "string" => {
            info!("Running program from input string...");
            MemoryInitialiser::initialise_from_string(source)
        }
        "file" => {
            let mut input_file = PathBuf::new();
            input_file.push(source);
            info!("Running program located: {}", input_file.as_path().display());
            MemoryInitialiser::new().initialise_from_text_file(input_file.as_path())
        }
        _ => unreachable!()
    };
    mem
}
