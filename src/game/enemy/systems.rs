use bevy::ecs::query;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::MapCreated;
use bevy_ecs_tiled::prelude::TiledEvent;
use bevy_rapier2d::prelude::Collider;

use bevy_rapier2d::prelude::Damping;
use bevy_rapier2d::prelude::Restitution;
use rand::Rng;
use bevy_rapier2d::prelude::{ Friction, GravityScale, LockedAxes, RigidBody };
use crate::game::tiled::resources::SpawnBounds;

use super::components::*;
use super::resources::*;

use super::const_string::{ ENEMY_SIZE_SCALE, ENEMY_SPRITE };

fn generate_random_velocity(max_velocity: f32) -> Vec2 {
    let mut rand = rand::rng();
    Vec2::new(
        rand.random_range(-max_velocity..max_velocity),
        rand.random_range(-max_velocity..max_velocity)
    )
}

pub fn set_enemy_texture(asset_server: Res<AssetServer>, mut gametexture: ResMut<GameTexture>) {
    gametexture.enemy = asset_server.load(ENEMY_SPRITE);
}

pub fn spawn_enemy(
    trigger: Trigger<TiledEvent<MapCreated>>,
    mut commands: Commands,
    gametexture: Res<GameTexture>,
    spawn_bounds: Res<SpawnBounds>
) {
    let mut rand = rand::rng();

    let w_span = spawn_bounds.width / 2.0 - 100.0;
    let h_span = spawn_bounds.height / 2.0 - 100.0;
    let x = rand.random_range(-w_span..w_span);
    let y = rand.random_range(-h_span..h_span);

    let max_velocity = 50.0;
    let velocity = generate_random_velocity(max_velocity);

    commands.spawn((
        Sprite::from_image(gametexture.enemy.clone()),
        Transform {
            translation: Vec3::new(x, y, 0.0),
            scale: Vec3::splat(ENEMY_SIZE_SCALE),
            ..Default::default()
        },
        RigidBody::Dynamic,
        Collider::ball(16.0),
        Velocity { value: velocity },
        Restitution::coefficient(1.0),
        Friction::coefficient(0.0),
        GravityScale(0.0),
        Damping { linear_damping: 0.0, angular_damping: 0.0 },
        LockedAxes::ROTATION_LOCKED,
        Enemy,
    ));
}


pub  fn speed_limit(max: Res<MaxSpeed>, mut query: Query<&mut Velocity>){
    for mut velocity in query.iter_mut(){
          let speed = (velocity.value.x.powi(2) + velocity.value.y.powi(2).sqrt());
          if speed > max.max_speed{
            let scale =  max.max_speed / speed;
            velocity.value.x *= scale;
            velocity.value.y *= scale
          }
    }
}