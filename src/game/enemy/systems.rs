use bevy::prelude::*;
use bevy_rapier2d::prelude::Ccd;
use bevy_rapier2d::prelude::CoefficientCombineRule;
use bevy_rapier2d::prelude::Collider;

use bevy_rapier2d::prelude::CollisionEvent;
use bevy_rapier2d::prelude::Damping;
use bevy_rapier2d::prelude::Restitution;
use bevy_rapier2d::prelude::Velocity;

use rand::Rng;
use bevy_rapier2d::prelude::{ Friction, GravityScale, LockedAxes, RigidBody };
use crate::game::tiled::resources::SpawnBounds;
use crate::game::tiled::events::MapFullyLoaded;

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
    mut map_loaded_event: EventReader<MapFullyLoaded>,
    mut commands: Commands,
    gametexture: Res<GameTexture>,
    spawn_bounds: Res<SpawnBounds>
) {
    for event in map_loaded_event.read() {
        info!("Spawning enemies for map : {:?}", event.map_entity);

        let mut rng = rand::rng();

        for i in 0..5 {
            let w_span = spawn_bounds.width / 2.0 - 100.0;
            let h_span = spawn_bounds.height / 2.0 - 100.0;
            let x = rng.random_range(-w_span..w_span);
            let y = rng.random_range(-h_span..h_span);

            let init_velocity = Vec2::new(150.0, 150.0);

            commands.spawn((
                Sprite::from_image(gametexture.enemy.clone()),
                Transform {
                    translation: Vec3::new(x, y, 10.0),
                    scale: Vec3::splat(ENEMY_SIZE_SCALE),
                    ..Default::default()
                },
                RigidBody::Dynamic,
                Collider::ball(16.0),
                Velocity::linear(init_velocity),
                Restitution {
                    coefficient: 1.0,
                    combine_rule: CoefficientCombineRule::Max,
                },
                Friction {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
                GravityScale(0.0),
                Damping { linear_damping: 0.0, angular_damping: 0.0 },
                LockedAxes::ROTATION_LOCKED,
                Ccd::enabled(),
                Enemy,
            ));
            info!("Spawn Enemy {} at ({:.0}, {:.0})", i, x, y);
        }
    }
}

pub fn speed_limit(max: Res<MaxSpeed>, mut query: Query<&mut Velocity, With<Enemy>>) {
    for mut velocity in query.iter_mut() {
        let speed = velocity.linvel.length();

        if speed > max.max_speed {
            let scale = max.max_speed / speed;
            velocity.linvel *= scale;
        } else if speed < max.max_speed * 0.95 {
            let scale = max.max_speed / speed;
            velocity.linvel += scale;
        }
    }
}

pub fn debug_enemy_collision(
    mut collision_events: EventReader<CollisionEvent>,
    enemy_query: Query<&Transform, With<Enemy>>
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _flag) = collision_event {
            if enemy_query.contains(*e1) || enemy_query.contains(*e2) {
                info!("Enemy Bounce");
            }
        }
    }
}
