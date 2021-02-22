use bevy::{prelude::*, render::pass::ClearColor};

mod bodies;
mod constants;
mod gui;
mod physics;
mod slingshot;

use bodies::{AsteroidSpawerTimer, Planet, Radius, ScreenShakeTimer};
use constants::{
	ASTEROID_SPAWN_PERIOD, INITIAL_BIOMASS, MENU_WIDTH, N_ASTEROIDS, RECIPES, SCREEN_HEIGHT,
	SCREEN_SHAKE_TIMER_DURATION, SCREEN_WIDTH, SPRITES_BOUNDARIES, VIEWPORT_SCALE,
};
use physics::AngularVelocity;

fn setup(
	commands: &mut Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
	let texture_handle = asset_server.load("images/textures.png");
	let mut texture_atlas = TextureAtlas::new_empty(texture_handle, Vec2::new(800., 600.));

	for (min, max) in SPRITES_BOUNDARIES.iter() {
		texture_atlas.add_texture(bevy::sprite::Rect {
			min: (*min).into(),
			max: (*max).into(),
		});
	}

	let texture_atlas_handle = texture_atlases.add(texture_atlas);

	commands
		.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
		.insert_resource(WindowDescriptor {
			title: "Biomass".to_string(),
			width: SCREEN_WIDTH,
			height: SCREEN_HEIGHT,
			resizable: false,
			vsync: true,
			..Default::default()
		})
		// .spawn(Camera2dBundle::default())
		.spawn(Camera2dBundle {
			transform: Transform::from_translation(Vec3::new(-MENU_WIDTH, 0., 0.))
				* Transform::from_scale(Vec3::splat(1. / VIEWPORT_SCALE)),
			..Default::default()
		})
		// Planet
		.spawn(SpriteSheetBundle {
			texture_atlas: texture_atlas_handle.clone(),
			sprite: TextureAtlasSprite::new(0),
			transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
			..Default::default()
		})
		.with(AngularVelocity(0.1))
		.with(Radius(100.))
		.with(Planet);
}

fn main() {
	App::build()
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup.system())
		// GUI
		.add_plugin(gui::GuiPlugin)
		// Physics & animations
		.add_plugin(physics::PhysicsPlugin)
		// Asteroid stuff
		.add_system(bodies::collision.system())
		.add_system(bodies::animate_explosion.system())
		.add_resource(AsteroidSpawerTimer(Timer::from_seconds(
			ASTEROID_SPAWN_PERIOD,
			true,
		)))
		.add_system(bodies::asteroid_spawner.system())
		.add_system(bodies::asteroid_despawner.system())
		// Slingshot
		.add_plugin(slingshot::SlingshotPlugin)
		// Screen shake
		.add_resource(ScreenShakeTimer::new(SCREEN_SHAKE_TIMER_DURATION))
		.add_system(bodies::screen_shaker.system())
		// Biomass stuff
		.add_resource(Biomass(INITIAL_BIOMASS))
		.add_resource(CurrentIngredients::new())
		.run();
}

type Ingredients = [usize; N_ASTEROIDS];

pub fn add_ingredient(ingredients: &Ingredients, index: usize) -> Ingredients {
	let mut new = ingredients.clone();
	new[index] += 1;
	new
}

pub struct Recipe {
	pub name: &'static str,
	pub requirement: u32,
	pub reward: u32,
	pub ingredients: Ingredients,
}

pub enum IngredientsComparison {
	TooFew,
	TooMany,
	JustRight,
}

impl Recipe {
	pub const fn new(
		name: &'static str,
		requirement: u32,
		reward: u32,
		ingredients: Ingredients,
	) -> Self {
		Self {
			name,
			requirement,
			reward,
			ingredients,
		}
	}

	pub fn check_ingredients(&self, ingredients: &Ingredients) -> IngredientsComparison {
		let mut result = IngredientsComparison::JustRight;
		for (i, j) in self.ingredients.iter().zip(ingredients) {
			if j < i {
				result = IngredientsComparison::TooFew;
			} else if j > i {
				return IngredientsComparison::TooMany;
			}
		}
		return result;
	}
}

pub struct Biomass(u32);

pub struct CurrentIngredients(Ingredients);

impl CurrentIngredients {
	pub fn new() -> Self {
		Self([0; N_ASTEROIDS])
	}
}

pub fn current_recipe(biomass: u32) -> usize {
	for (i, recipe) in RECIPES.iter().enumerate().rev() {
		if biomass >= recipe.requirement {
			return i;
		}
	}
	panic!("there are no recipes for {} biomass", biomass);
}
