use crate::game::player::constants::{
    AIR_DAMPING_FACTOR, GROUND_DAMPING_FACTOR, PLAYER_CRAB_TEXTUE,
};

use super::components::*;
use super::helper::{calculate_collider_size, is_grounded};
use super::resources::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn set_player_texture(asset_server: Res<AssetServer>, mut gametexture: ResMut<GameTexture>) {
    gametexture.player = asset_server.load(PLAYER_CRAB_TEXTUE);
}

pub fn spawn_player(
    mut commands: Commands,
    gametexture: Res<GameTexture>,
    config: Res<PlayerConfig>,
) {
    let (collider_half_w, collider_half_h) = calculate_collider_size(config.size, config.scale);
    commands.spawn((
        Sprite::from_image(gametexture.player.clone()),
        Transform {
            translation: Vec3::new(config.spawn_x, config.spawn_y, 3.0),
            scale: Vec3::splat(config.scale),
            ..Default::default()
        },
        PlayerPhysicsBundle::new(collider_half_w, collider_half_h),
        Player,
        CoyoteTime::default(),
    ));
}

pub fn player_keyboard_event(
    input_key: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &CoyoteTime), With<Player>>,
    config: Res<PlayerConfig>,
) {
    let Ok((mut velocity, coyote)) = query.single_mut() else {
        return;
    };

    if is_grounded(&velocity) {
        velocity.linvel.x = 0.0;

        if input_key.pressed(KeyCode::KeyA) || input_key.pressed(KeyCode::ArrowLeft) {
            velocity.linvel.x = -config.speed;
        }
        if input_key.pressed(KeyCode::KeyD) || input_key.pressed(KeyCode::ArrowRight) {
            velocity.linvel.x = config.speed;
        }
    }

    if input_key.just_pressed(KeyCode::Space) {
        if is_grounded(&velocity) || coyote.is_active() {
            velocity.linvel.y = config.jump;
        }
    }
}

pub fn update_coyote_time(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut CoyoteTime), With<Player>>,
) {
    let Ok((velocity, mut coyote)) = query.single_mut() else {
        return;
    };

    if is_grounded(velocity) {
        coyote.reset();
        return;
    }

    coyote.timer.tick(time.delta());
}

pub fn adjust_player_damping(mut query: Query<(&Velocity, &mut Damping), With<Player>>) {
    let Ok((velocity, mut damping)) = query.single_mut() else {
        return;
    };

    if is_grounded(velocity) {
        damping.linear_damping = GROUND_DAMPING_FACTOR;
        return;
    }

    damping.linear_damping = AIR_DAMPING_FACTOR;
}
