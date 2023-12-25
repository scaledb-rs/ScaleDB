// Copyright (c) ScaleDB LLC; used with permission
// Licensed under the MIT License

pub struct CommandLineOptions {
}

impl CommandLineOptions {
    pub fn new(mut args: std::env::Args) -> Result<CommandLineOptions, (/*exit_code: */i32, /*message: */String)> {
        // skip the first argument (which is the full path to our executable)
        args.next();

        // parse arguments
        // [none]

        Ok(CommandLineOptions { })
    } 
}

pub enum ExecutionError {
    Other{exit_code: i32, message: String}
}

pub fn run(_options: CommandLineOptions) -> Result<(), ExecutionError> {

    
    //

    // requested operation was successful; return Ok
    Ok(())
}
