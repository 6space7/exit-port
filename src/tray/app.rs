use std::{collections::HashMap, process::Command, thread, time::Duration};

use end_port::{
    format_menu_title, format_port_label, format_status_line, scan_dev_ports, stop_process,
    ListenerPort, UpdateAvailability, UpdateInfo,
};
use tao::event_loop::EventLoopProxy;
use tray_icon::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    TrayIcon,
};

use super::UserEvent;

pub(super) const MENU_REFRESH_ID: &str = "refresh";
pub(super) const MENU_CHECK_UPDATES_ID: &str = "check-updates";
pub(super) const MENU_OPEN_UPDATE_ID: &str = "open-update";
pub(super) const MENU_QUIT_ID: &str = "quit";

pub(super) struct TrayApp {
    menu: Menu,
    tray_icon: TrayIcon,
    ports_by_id: HashMap<String, ListenerPort>,
    status: Option<String>,
    update: UpdateMenuState,
}

#[derive(Debug, Clone)]
enum UpdateMenuState {
    Idle,
    Checking,
    Current(String),
    Available {
        version: String,
        release_url: String,
    },
    Error(String),
}

impl TrayApp {
    pub(super) fn new(menu: Menu, tray_icon: TrayIcon) -> Self {
        Self {
            menu,
            tray_icon,
            ports_by_id: HashMap::new(),
            status: None,
            update: UpdateMenuState::Idle,
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

    pub(super) fn begin_update_check(&mut self) {
        self.update = UpdateMenuState::Checking;
        self.refresh();
    }

    pub(super) fn finish_update_check(&mut self, result: std::result::Result<UpdateInfo, String>) {
        self.update = match result {
            Ok(info) => match info.availability {
                UpdateAvailability::Current => UpdateMenuState::Current(info.latest_version),
                UpdateAvailability::Available { release_url } => UpdateMenuState::Available {
                    version: info.latest_version,
                    release_url,
                },
            },
            Err(error) => UpdateMenuState::Error(error),
        };
        self.refresh();
    }

    pub(super) fn open_update(&mut self) {
        let UpdateMenuState::Available { release_url, .. } = &self.update else {
            self.set_status("No update is ready to open.".to_string());
            self.refresh();
            return;
        };

        let message = match open_url(release_url) {
            Ok(()) => "Opened the latest End Port release.".to_string(),
            Err(error) => format!("Could not open update page: {error}"),
        };
        self.set_status(message);
        self.refresh();
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
        self.render_update_items();
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
        self.render_update_items();
        self.add_item(MENU_REFRESH_ID, "Refresh", true);
        self.add_item(MENU_QUIT_ID, "Quit End Port", true);
        let _ = self.tray_icon.set_tooltip(Some("End Port scan failed"));
        self.tray_icon.set_title(None::<String>);
    }

    fn render_update_items(&self) {
        match &self.update {
            UpdateMenuState::Idle => {
                self.add_item(MENU_CHECK_UPDATES_ID, "Check for Updates", true);
            }
            UpdateMenuState::Checking => {
                self.add_item("update-checking", "Checking for updates...", false);
            }
            UpdateMenuState::Current(version) => {
                self.add_item(
                    "update-current",
                    format!("End Port {version} is current"),
                    false,
                );
                self.add_item(MENU_CHECK_UPDATES_ID, "Check Again", true);
            }
            UpdateMenuState::Available {
                version,
                release_url: _,
            } => {
                self.add_item(
                    MENU_OPEN_UPDATE_ID,
                    format!("Update available: End Port {version}"),
                    true,
                );
                self.add_item("update-command", "Run: brew upgrade --cask end-port", false);
                self.add_item(MENU_CHECK_UPDATES_ID, "Check Again", true);
            }
            UpdateMenuState::Error(error) => {
                self.add_item("update-error", "Update check failed", false);
                self.add_item("update-error-detail", error, false);
                self.add_item(MENU_CHECK_UPDATES_ID, "Try Again", true);
            }
        }

        self.add_separator();
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

fn open_url(url: &str) -> std::io::Result<()> {
    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg(url).spawn()?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd").args(["/C", "start", "", url]).spawn()?;
        Ok(())
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        Command::new("xdg-open").arg(url).spawn()?;
        Ok(())
    }
}
