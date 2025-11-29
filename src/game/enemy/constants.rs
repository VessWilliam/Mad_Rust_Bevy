//Enemy Texture & Visual
pub const ENEMY_SPRITE: &str = "bevy.png";
pub const ENEMY_SIZE_SCALE: f32 = 0.70;

//Enemy Speed & Safe Margin
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SAFE_MARGIN: f32 = 60.0;

//Enemy Collision Configuration
pub const COLLISION_BOUNCE: f32 = 0.5;
pub const COLLISION_ANGLE_VARIATION: f32 = 0.3;
pub const COLLISION_MIN_SPEED: f32 = 1.0;
pub const COLLISION_MIN_DISTANCE_SQ: f32 = 0.001;

//Enemy Speed Limit
pub const MAX_SPEED_LIMIT: f32 = 300.0;
pub const MIN_SPEED_LIMIT: f32 = 150.0;

//Enemy Physic Configuration
pub const PHYSICS_COLLIDER_RADIUS: f32 = 16.0;
pub const PHYSICS_RESTITUTION: f32 = 1.0;
pub const PHYSICS_FRICTION: f32 = 0.0;
pub const PHYSICS_GRAVITY: f32 = 0.0;
pub const PHYSICS_LINEAR_DAMPING: f32 = 0.0;
pub const PHYSICS_ANGULAR_DAMPING: f32 = 0.0;
