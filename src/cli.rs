use std::{error::Error, ffi::OsString};

use end_port::{render_cli_table, scan_dev_ports, stop_process};

pub enum Command {
    Tray,
    List,
    StopPid(u32),
    Help,
}

pub fn parse_command(args: impl IntoIterator<Item = String>) -> Result<Command, Box<dyn Error>> {
    let args = args.into_iter().collect::<Vec<_>>();

    match args.as_slice() {
        [] => Ok(Command::Tray),
        [flag] if flag == "--list" => Ok(Command::List),
        [flag, pid] if flag == "--stop-pid" => Ok(Command::StopPid(pid.parse::<u32>()?)),
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

pub fn print_help() {
    println!("End Port");
    println!();
    println!("Usage:");
    println!("  end-port              Start the tray/menu-bar utility");
    println!("  end-port --list       Print detected web dev ports");
    println!("  end-port --stop-pid N Stop one process by pid");
}

#[allow(dead_code)]
fn _keep_os_string_available_for_future_shell_paths(_: OsString) {}
