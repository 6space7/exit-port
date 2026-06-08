use std::{error::Error, ffi::OsString};

use end_port::{
    check_for_updates, render_cli_table, scan_dev_ports, stop_process, UpdateAvailability,
};

pub enum Command {
    Tray,
    List,
    StopPid(u32),
    CheckUpdates,
    Help,
}

pub fn parse_command(args: impl IntoIterator<Item = String>) -> Result<Command, Box<dyn Error>> {
    let args = args.into_iter().collect::<Vec<_>>();

    match args.as_slice() {
        [] => Ok(Command::Tray),
        [flag] if flag == "--list" => Ok(Command::List),
        [flag, pid] if flag == "--stop-pid" => Ok(Command::StopPid(pid.parse::<u32>()?)),
        [flag] if flag == "--check-updates" => Ok(Command::CheckUpdates),
        [flag] if flag == "--help" || flag == "-h" => Ok(Command::Help),
        _ => {
            print_help();
            Err("unknown arguments".into())
        }
    }
}

pub fn list_ports() -> Result<(), Box<dyn Error>> {
    let ports = scan_dev_ports()?;
    print!("{}", render_cli_table(&ports));
    Ok(())
}

pub fn stop_pid(pid: u32) -> Result<(), Box<dyn Error>> {
    stop_process(pid)?;
    println!("Sent stop signal to pid {pid}.");
    Ok(())
}

pub fn check_updates() -> Result<(), Box<dyn Error>> {
    let update = check_for_updates()?;

    match update.availability {
        UpdateAvailability::Current => {
            println!("End Port {} is up to date.", update.current_version);
        }
        UpdateAvailability::Available { release_url } => {
            println!(
                "End Port {} is available. You are running {}.",
                update.latest_version, update.current_version
            );
            println!("Release: {release_url}");
            println!();
            println!("Update with:");
            println!("  brew upgrade --cask end-port");
        }
    }

    Ok(())
}

pub fn print_help() {
    println!("End Port");
    println!();
    println!("Usage:");
    println!("  end-port              Start the tray/menu-bar utility");
    println!("  end-port --list       Print detected web dev ports");
    println!("  end-port --stop-pid N Stop one process by pid");
    println!("  end-port --check-updates");
}

#[allow(dead_code)]
fn _keep_os_string_available_for_future_shell_paths(_: OsString) {}
