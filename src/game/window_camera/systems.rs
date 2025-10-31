use crate::game::window_camera::resources::FullscreenState;

use super::resources::WinSize;
use bevy::prelude::*;
use bevy::window::{ PrimaryWindow, WindowMode, WindowResized };

pub fn set_window_init(
    mut winsize: ResMut<WinSize>,
    mut window: Query<&Window, With<PrimaryWindow>>
) {
    if let Ok(win) = window.single_mut() {
        winsize.w = win.width();
        winsize.h = win.height();
    }
}

pub fn update_window_size(
    mut winsize: ResMut<WinSize>,
    mut resize_render: EventReader<WindowResized>
) {
    for event in resize_render.read() {
        let width_change = (winsize.w - event.width).abs() > 0.5;
        let height_change = (winsize.h - event.height).abs() > 0.5;

        if width_change || height_change {
            if width_change || height_change {
                winsize.w = event.width;
                winsize.h = event.height;
                println!("Window resized to: {}x{}", winsize.w, winsize.h);
            }
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
    mut windows: Query<&mut Window, With<PrimaryWindow>>
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
                    win.resolution.set(1300.0, 800.0);
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
