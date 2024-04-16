use clap::Parser;
use colored::Colorize;
use gocar::commands::Cli;

/*
* This cli tool has one job, emulate local npm registry in a really scrappy way.
* To achieve this, we maintain a folder where we keep our "packages" ready for retrieval.
* A push to this registry is the act of building the project, packing it up and lastly,
* storing the tar ball in our special folder.
*
* And to pull is to just unpacking that tarball through yarn.
*/

fn main() {
    let cli = Cli::parse();
    let message = match cli.command.parse().handle() {
        Ok(success_message) => success_message.green(),
        Err(error_message) => error_message.to_string().red(),
    };
    println!("{message}");
}
