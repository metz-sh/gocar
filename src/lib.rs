pub mod commands;
pub mod fs_utils;
pub mod config;

use std::process::Command as ProcessCommand;

use commands::CommandFailedError;

use std::{
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn run_process(command: &mut Command) -> Result<(), CommandFailedError> {
    command
    .status()
    .map(|_| ())
    .map_err(|err| CommandFailedError::ProcessFailed(err.to_string()))
}

pub fn get_current_timestamp_string() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string()
}

pub fn run_script(script: String) -> Result<(), CommandFailedError> {
	let mut split_command_string: Vec<&str> = script.split(' ').collect();
	let command_string = split_command_string.remove(0);
	run_process(ProcessCommand::new(command_string).args(split_command_string))
}
