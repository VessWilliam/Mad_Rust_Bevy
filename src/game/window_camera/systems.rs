use super::constants::WINDOW_RESIZE_THRESHOLD;
use super::resources::{FullscreenState, WinSize, WindowResolution};
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode, WindowResized};

pub fn set_window_init(
    mut winsize: ResMut<WinSize>,
    mut window: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(win) = window.single_mut() {
        winsize.width = win.width();
        winsize.height = win.height();
    }
}

pub fn update_window_size(
    mut winsize: ResMut<WinSize>,
    mut resize_render: MessageReader<WindowResized>,
) {
    if let Some(event) = resize_render.read().last() {
        let width_change = (winsize.width - event.width).abs() > WINDOW_RESIZE_THRESHOLD;
        let height_change = (winsize.height - event.height).abs() > WINDOW_RESIZE_THRESHOLD;

        if width_change || height_change {
            winsize.width = event.width;
            winsize.height = event.height;
            println!("Window resized to: {}x{}", winsize.width, winsize.height);
        }
    }
}

pub fn exit_window(keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        println!("Esc Press: close window");
        std::process::exit(0)
    }
}

pub fn toggle_fullscreen(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut fullscreen_state: ResMut<FullscreenState>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut win) = windows.single_mut() {
        if keyboard.just_pressed(KeyCode::F11) {
            match (fullscreen_state.is_fullscreen, fullscreen_state.is_small) {
                (false, false) => {
                    win.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Current);
                    fullscreen_state.is_fullscreen = true;
                }
                (true, false) => {
                    win.mode = WindowMode::Windowed;
                    WindowResolution::DEFAULT_WINDOWED.apply_to(&mut win);
                    fullscreen_state.is_fullscreen = false;
                    fullscreen_state.is_small = true;
                }

                (_, true) => {
                    win.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Current);
                    fullscreen_state.is_fullscreen = true;
                    fullscreen_state.is_small = false;
                }
            }
        }
    }
}
