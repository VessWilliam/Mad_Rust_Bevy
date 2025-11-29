//Player Size & Texture
pub const PLAYER_SIZE: f32 = 1.0;
pub const PLAYER_CRAB_TEXTUE: &str = "crap.png";
pub const PLAYER_SIZE_SCALE: f32 = 0.50;

//Player Physics Configuration
pub const PHYSICS_GRAVITY_SCALE: f32 = 2.0;
pub const PHYSICS_FRICTION: f32 = 0.1;
pub const PHYSICS_RESTITUTION: f32 = 0.0;
pub const PHYSICS_LINEAR_DAMPING: f32 = 5.0;

//Player Collider Configuration
pub const COLLIDER_SCALE: f32 = 0.8;
pub const COLLIDER_PADDING: f32 = 14.0;

// Player Jump Mechanics
pub const COYOTE_TIME_SEC: f32 = 0.10;
pub const GROUNDED_THRESHOLD: f32 = 10.0;

// Player Dynamic Damping For Better Air Control
pub const AIR_DAMPING_FACTOR: f32 = 0.2;
pub const GROUND_DAMPING_FACTOR: f32 = 5.0;

// Player Default Spawn Position
pub const DEFAULT_PLAYER_SPAWN_X: f32 = 40.0;
pub const DEFAULT_PLAYER_SPAWN_Y: f32 = 50.0;

// Player Default Speed & Jump
pub const DEFAULT_PLAYER_SPEED: f32 = 150.0;
pub const DEFAULT_PLAYER_JUMP: f32 = 500.0;
