use clap::{Parser, Subcommand};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;

use crate::fs_utils::{
    get_current_directory_name, get_files_from_directory, get_latest_file_from_directory,
    parse_package_to_directory,
};
use crate::{get_current_timestamp_string, run_process, REGISTRY_PATH};
use std::path::Path;
use std::process::Command as ProcessCommand;

#[derive(Subcommand)]
pub enum CommandType {
    ///build, pack and push to local registry
    Push {
        #[arg(short, long)]
        skip_build: bool,
    },
    ///pull from registry and install
    Pull {
        #[arg(short, long)]
        package_name: String,
    },
    ///pull a particular version of package by selecting from a list
    PullVersion {
        #[arg(short, long)]
        package_name: String,
    },
}

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CommandType,
}

// This is the interface for various commands that our tool handles
pub trait Command {
    //The result captures either a success message or an error message
    fn handle(&self) -> Result<String, String>;
}

pub struct PullCommand {
    package_name: String,
}

pub struct PullVersionCommand {
    package_name: String,
}

pub struct PushCommand {
    skip_build: bool,
}

//This is the parser for converting command enums to structs
//TODO: find a better way because we are repeating arguments and cloning them as well
impl CommandType {
    pub fn parse(&self) -> Box<dyn Command> {
        match self {
            CommandType::Pull { package_name } => Box::new(PullCommand {
                package_name: package_name.clone(),
            }),
            CommandType::Push { skip_build } => Box::new(PushCommand {
                skip_build: skip_build.clone(),
            }),
            CommandType::PullVersion { package_name } => Box::new(PullVersionCommand {
                package_name: package_name.clone(),
            }),
        }
    }
}

/*
* To push, we pick the current directory and use it as the package name. We salt it with timestamp
* to make it unique.
*
* The file is stored as .registry/<current-directory-name>/<current-directory-name+timestamp>.tgz
* For example, if run from a folder named sandbox, the folder structure of registry will look like:
* ./registry/sandbox/sandbox_1712940454858.tgz
*/
impl Command for PushCommand {
    fn handle(&self) -> Result<String, String> {
        if !self.skip_build {
            run_process(&mut ProcessCommand::new("yarn").arg("build"))?;
        }

        let current_directory_name = get_current_directory_name();
        let file_name = format!(
            "{current_directory_name}_{time_millis}.tgz",
            time_millis = get_current_timestamp_string()
        );
        let destination_folder = format!("{REGISTRY_PATH}/{current_directory_name}");

        run_process(&mut ProcessCommand::new("mkdir").args(["-p", &destination_folder]))?;

        let destination = format!("{destination_folder}/{file_name}");
        run_process(&mut ProcessCommand::new("yarn").args(["pack", "-f", destination.as_str()]))
            .map(|_| format!("{}", "Pushed successfully!"))
    }
}

/*
* To pull, we first figure out the folder we need to look into in our registry.
* For package-name sandbox, we'd look into ./registry/sandbox
*
* Once we have the folder, we just figure out the latest file created in that folder and install it.
*/
impl Command for PullCommand {
    fn handle(&self) -> Result<String, String> {
        let full_path = format!("{REGISTRY_PATH}/{name}", name = &self.package_name);
        let path = parse_package_to_directory(&full_path)?;
        let latest_file = get_latest_file_from_directory(path)?;
        PullCommand::install(latest_file)
    }
}

impl PullCommand {
    fn install(file: String) -> Result<String, String> {
        run_process(&mut ProcessCommand::new("yarn").args(["add", format!("file:{file}").as_str()]))
            .map(|_| {
                format!(
                    "Installed {}",
                    Path::new(&file).file_name().unwrap().to_str().unwrap()
                )
            })
    }
}

/*
* To pull an older version of the package, we open a select menu with options from
* the folder corresponding to the package.
*
* Once the user selects the file they want, then we emulate the "pull" command
*/
impl Command for PullVersionCommand {
    fn handle(&self) -> Result<String, String> {
        let full_path = format!("{REGISTRY_PATH}/{name}", name = &self.package_name);
        let path = parse_package_to_directory(&full_path)?;
        let sorted_entries: Vec<String> = get_files_from_directory(path)
            .into_iter()
            .map(|entry| entry.unwrap().file_name().into_string().unwrap())
            .collect();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .default(0)
            .with_prompt("Pick version")
            .max_length(3)
            .items(&sorted_entries)
            .interact()
            .unwrap();

        let selected_file = sorted_entries[selection].clone();
        let selected_file_full_path = format!(
            "{REGISTRY_PATH}/{name}/{selected_file}",
            name = &self.package_name
        );

        PullCommand::install(selected_file_full_path)
    }
}
