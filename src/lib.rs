pub mod commands;
pub mod fs_utils;

use colored::Colorize;

use std::{
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

pub const REGISTRY_PATH: &'static str = "/Users/rahulramteke/.metz-registry";

pub fn run_process(command: &mut Command) -> Result<(), String> {
    let command = command.status().or_else(|e| Err(e.to_string()))?;
    match command.success() {
        false => Err(String::from("Command failed!")),
        true => Ok(()),
    }
}
pub fn print_helper() {
    println!(
        "metz local package manager
        
        {usage}: gocar [COMMAND]
        
        {commands}:
            {push}
                builds the project, packs it using yarn and stores the tgz
            
            {pull} <{name}>:<{version}>
                pulls the packs project and installs it using yarn
            
            {list}
                lists all packages stored with versions
        ",
        usage = "Usage".bright_green(),
        commands = "Commands".bright_green(),
        push = "push".cyan(),
        pull = "pull".cyan(),
        list = "list".cyan(),
        name = "PACKAGE_NAME".magenta(),
        version = "VERSION".magenta(),
    )
}

pub fn get_current_timestamp_string() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string()
}
