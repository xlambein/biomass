use bevy::prelude::*;
// use bevy_rng::*;
use rand::prelude::*;

use crate::{
	add_ingredient,
	constants::{
		ASTEROID_ANGULAR_VELOCITY, ASTEROID_DESPAWN_DISTANCE, ASTEROID_INIT_VELOCITY_NORMAL,
		ASTEROID_INIT_VELOCITY_RADIAL, ASTEROID_SPAWN_DISTANCE, ASTEROID_SPRITES, EXPLOSION_FRAMES,
		EXPLOSION_PERIOD, EXTINCTION_RATE, N_ASTEROIDS, RECIPES,
	},
	current_recipe,
	physics::{AngularVelocity, Velocity},
	screen_shaker::{ScreenShakeTimer, ScreenShaker},
	Biomass, CurrentIngredients, IngredientsComparison,
};

pub struct Planet;

pub struct Asteroid(usize);

pub struct Radius(pub f32);

impl Radius {
	pub fn contains(&self, point: Vec2) -> bool {
		point.length() <= self.0
	}
}

pub struct Explosion {
	pub timer: Timer,
	pub frame: usize,
}

impl Explosion {
	pub fn new() -> Self {
		Explosion {
			timer: Timer::from_seconds(EXPLOSION_PERIOD, true),
			frame: 0,
		}
	}
}

pub struct AsteroidSpawerTimer(pub Timer);

pub fn animate_explosion(
	time: Res<Time>,
	commands: &mut Commands,
	mut query: Query<(Entity, &mut Explosion, &mut TextureAtlasSprite)>,
) {
	for (entity, mut explosion, mut sprite) in query.iter_mut() {
		if explosion.timer.tick(time.delta_seconds()).just_finished() {
			explosion.frame += 1;
			if explosion.frame >= EXPLOSION_FRAMES.len() {
				commands.despawn(entity);
			} else {
				sprite.index = EXPLOSION_FRAMES[explosion.frame];
			}
		}
	}
}

pub fn collision(
	commands: &mut Commands,
	texture_atlases: Res<Assets<TextureAtlas>>,
	asteroids: Query<(Entity, &Transform, &Radius, &Velocity, &Asteroid)>,
	planets: Query<(&Transform, &Radius), With<Planet>>,
	mut screen_shaker: Query<&mut ScreenShaker>,
	mut screen_shaker_timer: ResMut<ScreenShakeTimer>,
	mut biomass: ResMut<Biomass>,
	mut current_ingredients: ResMut<CurrentIngredients>,
) {
	let mut explosions = vec![];
	for (entity, asteroid_t, Radius(asteroid_radius), Velocity(velocity), Asteroid(asteroid)) in
		asteroids.iter()
	{
		for (planet_t, Radius(planet_radius)) in planets.iter() {
			let r = (planet_t.translation - asteroid_t.translation).length();
			// If the asteroid collides with the planet
			if r <= asteroid_radius + planet_radius {
				// Despawn the asteroid
				commands.despawn(entity);

				// Store the asteroid's location to later spawn an explosion.  We have to do
				// this because, for some reason, if we do it in the loop, it doesn't work .__.
				explosions.push(asteroid_t.clone());

				// Reset screen shaker timer & set direction to velocity of the asteroid
				for mut screen_shaker in screen_shaker.iter_mut() {
					screen_shaker.direction = velocity.normalize();
					screen_shaker_timer.0.reset();
				}

				let ingredients = add_ingredient(&current_ingredients.0, *asteroid);
				let recipe = &RECIPES[current_recipe(biomass.0)];
				match recipe.check_ingredients(&ingredients) {
					IngredientsComparison::TooFew => {
						*current_ingredients = CurrentIngredients(ingredients);
					}
					IngredientsComparison::TooMany => {
						*current_ingredients = CurrentIngredients::new();
						biomass.0 = (biomass.0 as f32 / EXTINCTION_RATE).max(1.) as u32;
					}
					IngredientsComparison::JustRight => {
						*current_ingredients = CurrentIngredients::new();
						biomass.0 += recipe.reward;
					}
				}
			}
		}
	}

	// Spawn an explosion for each impact
	for transform in explosions {
		commands
			.spawn(SpriteSheetBundle {
				texture_atlas: texture_atlases.get_handle(texture_atlases.ids().next().unwrap()),
				sprite: TextureAtlasSprite::new(EXPLOSION_FRAMES[0]),
				transform,
				..Default::default()
			})
			.with(Explosion::new());
	}
}

fn vec2_from_radial(radius: f32, angle: f32) -> Vec2 {
	radius * Vec2::new(angle.cos(), angle.sin())
}

pub fn asteroid_spawner(
	// mut rng: Local<Rng>,
	time: Res<Time>,
	mut timer: ResMut<AsteroidSpawerTimer>,
	commands: &mut Commands,
	texture_atlases: Res<Assets<TextureAtlas>>,
) {
	if timer.0.tick(time.delta_seconds()).just_finished() {
		let mut rng = rand::thread_rng();
		let angular_velocity =
			[-1.0, 1.0].choose(&mut rng).unwrap() * rng.gen_range(ASTEROID_ANGULAR_VELOCITY);
		let position = vec2_from_radial(
			ASTEROID_SPAWN_DISTANCE,
			rng.gen_range(0.0..std::f32::consts::TAU),
		);
		let r = (Vec2::zero() - position).normalize();
		let n = r.perp();
		let velocity = rng.gen_range(ASTEROID_INIT_VELOCITY_RADIAL) * r
			+ rng.gen_range(ASTEROID_INIT_VELOCITY_NORMAL) * n;
		let asteroid = rng.gen_range(0..N_ASTEROIDS);
		let sprite = TextureAtlasSprite::new(ASTEROID_SPRITES[asteroid]);

		commands
			.spawn(SpriteSheetBundle {
				texture_atlas: texture_atlases.get_handle(texture_atlases.ids().next().unwrap()),
				sprite,
				transform: Transform::from_translation(position.extend(0.)),
				..Default::default()
			})
			.with(Velocity(velocity))
			.with(AngularVelocity(angular_velocity))
			.with(Radius(30.))
			.with(Asteroid(asteroid));
	}
}

pub fn asteroid_despawner(
	commands: &mut Commands,
	query: Query<(Entity, &Transform), With<Asteroid>>,
) {
	for (asteroid, transform) in query.iter() {
		if transform.translation.length() > ASTEROID_DESPAWN_DISTANCE {
			commands.despawn(asteroid);
		}
	}
}
