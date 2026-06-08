use exit_port::{
    format_menu_title, format_port_label, format_status_line, plan_dev_ports, ListenerPort,
    PortScope,
};
use pretty_assertions::assert_eq;

fn port(port: u16, pid: u32, process_name: &str) -> ListenerPort {
    ListenerPort {
        port,
        pid,
        process_name: process_name.to_string(),
        scope: PortScope::Loopback,
        command: None,
    }
}

#[test]
fn plan_dev_ports_filters_dedupes_and_sorts_ports() {
    let planned = plan_dev_ports(vec![
        port(3000, 42, "node"),
        port(22, 1, "sshd"),
        port(5173, 77, "vite"),
        port(3000, 42, "node"),
        port(5432, 88, "postgres"),
        port(8080, 99, "python"),
    ]);

    assert_eq!(
        planned,
        vec![
            port(3000, 42, "node"),
            port(5173, 77, "vite"),
            port(8080, 99, "python"),
        ]
    );
}

#[test]
fn plan_dev_ports_keeps_unknown_processes_on_common_dev_ports() {
    let planned = plan_dev_ports(vec![port(4321, 0, "unknown"), port(9229, 0, "unknown")]);

    assert_eq!(
        planned,
        vec![port(4321, 0, "unknown"), port(9229, 0, "unknown")]
    );
}

#[test]
fn plan_dev_ports_excludes_regular_desktop_app_listeners() {
    let planned = plan_dev_ports(vec![
        port(5000, 643, "ControlCenter"),
        port(7265, 1013, "Raycast"),
        port(49285, 971, "Spotify"),
    ]);

    assert_eq!(planned, Vec::new());
}

#[test]
fn format_port_label_includes_port_process_and_pid() {
    assert_eq!(
        format_port_label(&port(5173, 77, "vite")),
        "Stop :5173  vite  pid 77"
    );
}

#[test]
fn presentation_summarizes_menu_state_cleanly() {
    assert_eq!(format_menu_title(0), "Exit Port");
    assert_eq!(format_status_line(0), "No development servers running");
    assert_eq!(format_status_line(1), "1 development server running");
    assert_eq!(format_status_line(3), "3 development servers running");
}
