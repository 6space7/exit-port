mod cli;
mod tray;

use std::error::Error;

use cli::{parse_command, Command};

fn main() {
    if let Err(error) = run() {
        eprintln!("exit-port: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    match parse_command(std::env::args().skip(1))? {
        Command::Tray => tray::run(),
        Command::List => cli::list_ports(),
        Command::StopPid(pid) => cli::stop_pid(pid),
        Command::Help => {
            cli::print_help();
            Ok(())
        }
    }
}
