// Copyright (c) ScaleDB LLC; used with permission
// Licensed under the MIT License

use scaledb::{
    CommandLineOptions,
    ExecutionError
};
use std::env;
use std::process;

fn main() {
    let application_name = "ScaleDB".to_string();
    let application_name_and_version = application_name + " v" + env!("CARGO_PKG_VERSION_MAJOR") + "." + env!("CARGO_PKG_VERSION_MINOR") + "." + env!("CARGO_PKG_VERSION_PATCH");
    //
    println!("{}", application_name_and_version);
    println!("");

    // capture and parse the command-line arguments
    let command_line_options = CommandLineOptions::new(env::args()).unwrap_or_else(|(exit_code, message)| {
        assert!(exit_code != 0 /* !success */, "Command line could not be parsed. Returned error code was zero, which is an invalid exit code.");
        //
        eprintln!("Error: {}", message);
        process::exit(exit_code);
    });

    if let Err(e) = scaledb::run(command_line_options) {
        match e {
            ExecutionError::Other{exit_code, message} => {
                assert!(exit_code != 0 /* !success */, "Execution failed. Returned error code was zero, which is an invalid exit code.");
                //
                eprintln!("Error: {}", message);
                process::exit(exit_code);        
            }
        }
    }

    process::exit(0 /* success */);
}
