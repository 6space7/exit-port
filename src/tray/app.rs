use std::{collections::HashMap, thread, time::Duration};

use end_port::{
    format_menu_title, format_port_label, format_status_line, scan_dev_ports, stop_process,
    ListenerPort,
};
use tao::event_loop::EventLoopProxy;
use tray_icon::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    TrayIcon,
};

use super::UserEvent;

pub(super) const MENU_REFRESH_ID: &str = "refresh";
pub(super) const MENU_QUIT_ID: &str = "quit";

pub(super) struct TrayApp {
    menu: Menu,
    tray_icon: TrayIcon,
    ports_by_id: HashMap<String, ListenerPort>,
    status: Option<String>,
}

impl TrayApp {
    pub(super) fn new(menu: Menu, tray_icon: TrayIcon) -> Self {
        Self {
            menu,
            tray_icon,
            ports_by_id: HashMap::new(),
            status: None,
        }
    }

    pub(super) fn set_status(&mut self, status: String) {
        self.status = Some(status);
    }

    pub(super) fn refresh(&mut self) {
        self.clear_menu();
        self.ports_by_id.clear();

        match scan_dev_ports() {
            Ok(ports) => self.render_ports(ports),
            Err(error) => self.render_scan_error(error.to_string()),
        }
    }

    pub(super) fn stop_port(&mut self, id: &str, proxy: EventLoopProxy<UserEvent>) {
        let Some(port) = self.ports_by_id.get(id).cloned() else {
            self.set_status("Port list changed. Refreshed.".to_string());
            self.refresh();
            return;
        };

        if port.pid == 0 {
            self.set_status(format!(
                "Cannot stop :{} because its pid is unknown.",
                port.port
            ));
            self.refresh();
            return;
        }

        let label = format!(":{} {}", port.port, port.process_name);
        self.set_status(format!("Stopping {label}"));
        self.refresh();

        thread::spawn(move || {
            let message = match stop_process(port.pid) {
                Ok(()) => format!("Stopped {label}"),
                Err(error) => format!("Could not stop {label}: {error}"),
            };

            thread::sleep(Duration::from_millis(300));
            let _ = proxy.send_event(UserEvent::Refresh(Some(message)));
        });
    }

    fn render_ports(&mut self, ports: Vec<ListenerPort>) {
        let count = ports.len();
        let title = format_menu_title(count);
        let status = format_status_line(count);

        self.add_item("title", &title, false);
        self.add_item("summary", &status, false);
        if let Some(status) = self.status.clone() {
            self.add_item("last-action", status, false);
        }
        self.add_separator();

        if ports.is_empty() {
            self.add_item("empty", "Nothing to stop right now", false);
        } else {
            for port in ports {
                let id = format!("stop:{}:{}", port.pid, port.port);
                self.add_item(&id, format_port_label(&port), port.pid != 0);
                self.ports_by_id.insert(id, port);
            }
        }

        self.add_separator();
        self.add_item(MENU_REFRESH_ID, "Refresh", true);
        self.add_item(MENU_QUIT_ID, "Quit End Port", true);

        let _ = self.tray_icon.set_tooltip(Some(status));
        self.tray_icon
            .set_title((count > 0).then(|| count.to_string()));
    }

    fn render_scan_error(&mut self, error: String) {
        self.add_item("title", "End Port", false);
        self.add_item("error-title", "Scan failed", false);
        self.add_item("error", error, false);
        self.add_separator();
        self.add_item(MENU_REFRESH_ID, "Refresh", true);
        self.add_item(MENU_QUIT_ID, "Quit End Port", true);
        let _ = self.tray_icon.set_tooltip(Some("End Port scan failed"));
        self.tray_icon.set_title(None::<String>);
    }

    fn clear_menu(&self) {
        while !self.menu.items().is_empty() {
            let _ = self.menu.remove_at(0);
        }
    }

    fn add_item(&self, id: &str, text: impl AsRef<str>, enabled: bool) {
        let item = MenuItem::with_id(id, text.as_ref(), enabled, None);
        if let Err(error) = self.menu.append(&item) {
            eprintln!("end-port: failed to add menu item: {error}");
        }
    }

    fn add_separator(&self) {
        let separator = PredefinedMenuItem::separator();
        if let Err(error) = self.menu.append(&separator) {
            eprintln!("end-port: failed to add menu separator: {error}");
        }
    }
}
