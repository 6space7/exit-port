use crate::model::ListenerPort;

pub fn format_menu_title(_count: usize) -> String {
    "End Port".to_string()
}

pub fn format_status_line(count: usize) -> String {
    match count {
        0 => "No development servers running".to_string(),
        1 => "1 development server running".to_string(),
        count => format!("{count} development servers running"),
    }
}

pub fn format_port_label(port: &ListenerPort) -> String {
    if port.pid == 0 {
        format!(
            "Stop :{}  {}  pid unknown",
            port.port,
            compact_process_name(&port.process_name)
        )
    } else {
        format!(
            "Stop :{}  {}  pid {}",
            port.port,
            compact_process_name(&port.process_name),
            port.pid
        )
    }
}

pub fn render_cli_table(ports: &[ListenerPort]) -> String {
    if ports.is_empty() {
        return "No web dev ports found.\n".to_string();
    }

    let mut output = String::from("PORT     PID      PROCESS            COMMAND\n");
    for port in ports {
        output.push_str(&format!(
            "{:<8} {:<8} {:<18} {}\n",
            format!(":{}", port.port),
            if port.pid == 0 {
                "-".to_string()
            } else {
                port.pid.to_string()
            },
            compact_process_name(&port.process_name),
            port.command.as_deref().unwrap_or_default()
        ));
    }
    output
}

fn compact_process_name(process_name: &str) -> String {
    process_name
        .split_whitespace()
        .take(3)
        .collect::<Vec<_>>()
        .join(" ")
}
