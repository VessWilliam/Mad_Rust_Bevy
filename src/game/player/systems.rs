use super::components::*;
use super::resources::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::constants::{COLLIDER_PADDING, COLLIDER_SCALE, GROUNDED_THRESHOLD, PLAYER_CRAB_TEXTUE};

pub fn set_player_texture(asset_server: Res<AssetServer>, mut gametexture: ResMut<GameTexture>) {
    gametexture.player = asset_server.load(PLAYER_CRAB_TEXTUE);
}

pub fn spawn_player(
    mut commands: Commands,
    gametexture: Res<GameTexture>,
    config: Res<PlayerConfig>,
) {
    let sprite_half_w = (config.size * config.scale) / 2.0;
    let sprite_half_h = (config.size * config.scale) / 2.0;

    let collider_half_w = sprite_half_w * COLLIDER_SCALE + COLLIDER_PADDING;
    let collider_half_h = sprite_half_h * COLLIDER_SCALE + COLLIDER_PADDING;

    commands.spawn((
        Sprite::from_image(gametexture.player.clone()),
        Transform {
            translation: Vec3::new(config.spawn_x, config.spawn_y, 3.0),
            scale: Vec3::splat(config.scale),
            ..Default::default()
        },
        PlayerPhysicsBundle::new(collider_half_w, collider_half_h),
        Player,
    ));
}

pub fn player_keyboad_event(
    input_key: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
    config: Res<PlayerConfig>,
) {
    let Ok(mut velocity) = query.single_mut() else {
        return;
    };

    let speed = config.speed;
    velocity.linvel.x = 0.0;

    if input_key.pressed(KeyCode::KeyA) || input_key.pressed(KeyCode::ArrowLeft) {
        velocity.linvel.x = -speed;
    }
    if input_key.pressed(KeyCode::KeyD) || input_key.pressed(KeyCode::ArrowRight) {
        velocity.linvel.x = speed;
    }

    if input_key.just_pressed(KeyCode::Space) {
        if velocity.linvel.y.abs() < GROUNDED_THRESHOLD {
            velocity.linvel.y = config.jump;
        }
    }
}
