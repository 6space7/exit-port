mod app;
mod icon;

use std::{error::Error, time::Instant};

use tao::{
    event::{Event, StartCause},
    event_loop::{ControlFlow, EventLoopBuilder},
};
use tray_icon::{
    menu::{Menu, MenuEvent},
    MouseButtonState, TrayIconBuilder, TrayIconEvent,
};

use app::{TrayApp, MENU_QUIT_ID, MENU_REFRESH_ID};

const REFRESH_EVERY: std::time::Duration = std::time::Duration::from_secs(5);

enum UserEvent {
    Menu(String),
    Refresh(Option<String>),
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut builder = EventLoopBuilder::<UserEvent>::with_user_event();
    let event_loop = builder.build();
    let menu = Menu::new();

    let tray_icon = TrayIconBuilder::new()
        .with_icon(icon::tray_icon()?)
        .with_icon_as_template(cfg!(target_os = "macos"))
        .with_menu(Box::new(menu.clone()))
        .with_tooltip("Exit Port")
        .build()?;

    let proxy = event_loop.create_proxy();
    MenuEvent::set_event_handler(Some(move |event: MenuEvent| {
        let _ = proxy.send_event(UserEvent::Menu(event.id().as_ref().to_string()));
    }));

    let proxy = event_loop.create_proxy();
    TrayIconEvent::set_event_handler(Some(move |event: TrayIconEvent| {
        if matches!(
            event,
            TrayIconEvent::Click {
                button_state: MouseButtonState::Down,
                ..
            }
        ) {
            let _ = proxy.send_event(UserEvent::Refresh(None));
        }
    }));

    let mut app = TrayApp::new(menu, tray_icon);
    let action_proxy = event_loop.create_proxy();
    let mut next_refresh = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(next_refresh);

        match event {
            Event::NewEvents(StartCause::Init)
            | Event::NewEvents(StartCause::ResumeTimeReached { .. }) => {
                app.refresh();
                next_refresh = Instant::now() + REFRESH_EVERY;
                *control_flow = ControlFlow::WaitUntil(next_refresh);
            }
            Event::UserEvent(UserEvent::Refresh(status)) => {
                if let Some(status) = status {
                    app.set_status(status);
                }
                app.refresh();
                next_refresh = Instant::now() + REFRESH_EVERY;
                *control_flow = ControlFlow::WaitUntil(next_refresh);
            }
            Event::UserEvent(UserEvent::Menu(id)) if id == MENU_REFRESH_ID => {
                app.refresh();
                next_refresh = Instant::now() + REFRESH_EVERY;
                *control_flow = ControlFlow::WaitUntil(next_refresh);
            }
            Event::UserEvent(UserEvent::Menu(id)) if id == MENU_QUIT_ID => {
                *control_flow = ControlFlow::Exit;
            }
            Event::UserEvent(UserEvent::Menu(id)) if id.starts_with("stop:") => {
                app.stop_port(&id, action_proxy.clone());
            }
            _ => {}
        }
    });
}
