use std::ffi::OsStr;

use sysinfo::{Pid, ProcessesToUpdate, System};

use crate::model::{ExitPortError, Result};

pub fn process_details(system: &System, pid: u32) -> (String, Option<String>) {
    let Some(process) = system.process(Pid::from_u32(pid)) else {
        return ("unknown".to_string(), None);
    };

    let process_name = os_str_to_string(process.name());
    let command = if process.cmd().is_empty() {
        None
    } else {
        Some(
            process
                .cmd()
                .iter()
                .map(|part| part.to_string_lossy())
                .collect::<Vec<_>>()
                .join(" "),
        )
    };

    (process_name, command)
}

pub fn stop_process(pid: u32) -> Result<()> {
    if pid == 0 {
        return Err(ExitPortError::MissingPid);
    }

    let mut system = System::new_all();
    let sys_pid = Pid::from_u32(pid);
    system.refresh_processes(ProcessesToUpdate::Some(&[sys_pid]), true);

    let process = system
        .process(sys_pid)
        .ok_or(ExitPortError::ProcessGone(pid))?;
    let name = os_str_to_string(process.name());

    if process.kill() {
        Ok(())
    } else {
        Err(ExitPortError::KillFailed { pid, name })
    }
}

fn os_str_to_string(value: &OsStr) -> String {
    let text = value.to_string_lossy().trim().to_string();
    if text.is_empty() {
        "unknown".to_string()
    } else {
        text
    }
}
