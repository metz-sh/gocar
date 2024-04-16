pub mod commands;
pub mod fs_utils;

use commands::CommandFailedError;

use std::{
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

pub const REGISTRY_PATH: &'static str = "/Users/rahulramteke/.metz-registry";

pub fn run_process(command: &mut Command) -> Result<(), CommandFailedError> {
    let command = command
        .status()
        .or_else(|_| Err(CommandFailedError::ProcessFailed))?;
    match command.success() {
        false => Err(CommandFailedError::ProcessFailed),
        true => Ok(()),
    }
}

pub fn get_current_timestamp_string() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string()
}
