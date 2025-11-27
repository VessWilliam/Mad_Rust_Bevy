use super::components::*;
use super::resources::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::Damping;
use bevy_rapier2d::prelude::Restitution;
use bevy_rapier2d::prelude::*;

use super::const_string::PLAYER_CRAB_TEXTUE;

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

    let collider_half_w = sprite_half_w * 0.8 + 14.0;
    let collider_half_h = sprite_half_h * 0.8 + 14.0;

    commands.spawn((
        Sprite::from_image(gametexture.player.clone()),
        Transform {
            translation: Vec3::new(config.spawn_x, config.spawn_y, 3.0),
            scale: Vec3::splat(config.scale),
            ..Default::default()
        },
        CrabPlayerCom {},
        RigidBody::Dynamic,
        Collider::cuboid(collider_half_w, collider_half_h),
        LockedAxes::ROTATION_LOCKED,
        Velocity::zero(),
        VelocityCom { x: 0.0, y: 0.0 },
        GravityScale(2.0),
        Friction {
            coefficient: 0.1,
            combine_rule: CoefficientCombineRule::Min,
        },
        Restitution::coefficient(0.0),
        Damping {
            linear_damping: 5.0,
            angular_damping: 0.0,
        },
    ));
}

pub fn player_keyboad_event(
    input_key: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut VelocityCom, &Velocity), With<CrabPlayerCom>>,
    config: Res<PlayerConfig>,
) {
    let Ok((mut vel_com, rap_vel)) = query.single_mut() else {
        return;
    };

    let speed = config.speed;
    vel_com.x = 0.0;

    if input_key.pressed(KeyCode::KeyA) || input_key.pressed(KeyCode::ArrowLeft) {
        vel_com.x = -speed;
    }
    if input_key.pressed(KeyCode::KeyD) || input_key.pressed(KeyCode::ArrowRight) {
        vel_com.x = speed;
    }

    if input_key.just_pressed(KeyCode::Space) {
        if rap_vel.linvel.y.abs() < 0.5 {
            vel_com.y = config.jump;
        }
        return;
    }

    vel_com.y = 0.0;
}

pub fn sync_velocity_to_rapier(
    mut query: Query<(&VelocityCom, &mut Velocity), With<CrabPlayerCom>>,
) {
    for (vel_com, mut rap_vel) in query.iter_mut() {
        rap_vel.linvel.x = vel_com.x;

        if vel_com.y != 0.0 {
            rap_vel.linvel.y = vel_com.y;
        }
    }
}
