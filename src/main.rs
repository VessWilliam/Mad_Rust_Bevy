use bevy::prelude::*;

mod game;
use bevy_rapier2d::{ plugin::{ NoUserData, RapierPhysicsPlugin }, render::RapierDebugRenderPlugin };
use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Crab Dungeon".into(),
                    mode: bevy::window::WindowMode::Fullscreen(
                        bevy::window::MonitorSelection::Primary,
                        bevy::window::VideoModeSelection::Current
                    ),
                    resizable: true,
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(GamePlugin)
        .run();
}
