use std::ops::Range;

use crate::Recipe;

pub const SCREEN_HEIGHT: f32 = 600.;
pub const SCREEN_WIDTH: f32 = SCREEN_HEIGHT / 9. * 16.;
pub const MENU_WIDTH: f32 = 280.;
pub const VIEWPORT_SCALE: f32 = 0.5;

pub const EGUI_TEXTURE_ATLAS_ID: u64 = 0;

pub const FPS: u64 = 60;
pub const DELTA: f64 = 1. / FPS as f64;
pub const G: f32 = 2e6;

pub const EXPLOSION_FRAMES: [u32; 3] = [10, 11, 12];
pub const EXPLOSION_PERIOD: f32 = 0.4;

pub const SPRITES_BOUNDARIES: [((f32, f32), (f32, f32)); 14] = [
	((48.0, 22.0), (312.0, 286.0)),
	((394.0, 45.0), (458.0, 109.0)),
	((483.0, 84.0), (551.0, 152.0)),
	((413.0, 149.0), (475.0, 211.0)),
	((522.0, 198.0), (578.0, 254.0)),
	((566.0, 150.0), (610.0, 194.0)),
	((596.0, 76.0), (658.0, 138.0)),
	((652.0, 8.0), (718.0, 74.0)),
	((677.0, 75.0), (749.0, 147.0)),
	((634.0, 176.0), (690.0, 232.0)),
	((382.0, 328.0), (456.0, 402.0)),
	((468.0, 319.0), (578.0, 429.0)),
	((604.0, 308.0), (746.0, 450.0)),
	((82.0, 322.0), (162.0, 402.0)),
];
pub const DNA_SPRITE: u32 = 13;

pub const N_ASTEROIDS: usize = 9;
pub const ASTEROID_SPRITES: [u32; N_ASTEROIDS] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
pub const ASTEROID_SPAWN_PERIOD: f32 = 2.0;
pub const ASTEROID_ANGULAR_VELOCITY: Range<f32> = 0.2..1.0;
pub const ASTEROID_SPAWN_DISTANCE: f32 = SCREEN_WIDTH / VIEWPORT_SCALE;
pub const ASTEROID_INIT_VELOCITY_RADIAL: Range<f32> = 50.0..200.0;
pub const ASTEROID_INIT_VELOCITY_NORMAL: Range<f32> = -50.0..50.0;
pub const ASTEROID_DESPAWN_DISTANCE: f32 = 2.0 * SCREEN_WIDTH / VIEWPORT_SCALE;

pub const SLINGSHOT_STRENGTH: f32 = 0.8;
pub const SLINGSHOT_MAX_LENGTH: f32 = 300.0;

pub const SCREEN_SHAKE_INTENSITY: f32 = 20.;
pub const SCREEN_SHAKE_SPEED: f32 = 10.;
pub const SCREEN_SHAKE_DAMPENING: f32 = 5.;
pub const SCREEN_SHAKE_TIMER_DURATION: f32 = 10. / SCREEN_SHAKE_DAMPENING;

pub const RECIPES: [Recipe; 9] = [
	Recipe::new("Protocells", 0, 10, [1, 0, 0, 0, 0, 0, 0, 0, 0]),
	Recipe::new("Procaryotes", 40, 20, [1, 1, 0, 0, 0, 0, 0, 0, 0]),
	Recipe::new("Bacteria", 100, 50, [1, 1, 1, 0, 0, 0, 0, 0, 0]),
	Recipe::new("Archea", 250, 50, [1, 2, 0, 1, 1, 0, 0, 0, 0]),
	Recipe::new("Eucaryotes", 400, 200, [2, 2, 1, 1, 1, 0, 0, 0, 0]),
	Recipe::new("Multicellulars", 1000, 1000, [2, 3, 2, 2, 2, 1, 0, 0, 0]),
	Recipe::new("Plants", 4000, 5000, [4, 3, 2, 3, 1, 1, 1, 0, 0]),
	Recipe::new("Animals", 15_000, 10_000, [3, 4, 2, 4, 3, 1, 0, 1, 0]),
	Recipe::new("Sentients", 40_000, 20_000, [3, 3, 3, 3, 4, 2, 1, 1, 2]),
];
pub const INITIAL_BIOMASS: u32 = 10;
pub const EXTINCTION_RATE: f32 = 1.25;
