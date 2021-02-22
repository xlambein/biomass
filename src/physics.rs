use bevy::{
	core::{FixedTimestep, FixedTimesteps},
	prelude::*,
};

use crate::{
	bodies::{Asteroid, Planet},
	constants::{DELTA, G},
};

pub struct Velocity(pub Vec2);

pub struct AngularVelocity(pub f32);

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_stage_after(
			stage::UPDATE,
			"fixed_update",
			SystemStage::parallel()
				.with_run_criteria(FixedTimestep::step(DELTA))
				.with_system(gravity.system())
				.with_system(velocity.system())
				.with_system(angular_velocity.system()),
		);
	}
}

// TODO use timesteps value instead of DELTA
pub fn velocity(_timesteps: Res<FixedTimesteps>, mut query: Query<(&Velocity, &mut Transform)>) {
	for (Velocity(vel), mut transform) in query.iter_mut() {
		transform.translation += DELTA as f32 * vel.extend(0.);
	}
}

pub fn angular_velocity(
	_timesteps: Res<FixedTimesteps>,
	mut query: Query<(&AngularVelocity, &mut Transform)>,
) {
	for (AngularVelocity(av), mut transform) in query.iter_mut() {
		transform.rotate(Quat::from_rotation_z(DELTA as f32 * av));
	}
}

pub fn gravity(
	_timesteps: Res<FixedTimesteps>,
	mut asteroids: Query<(&mut Velocity, &Transform), With<Asteroid>>,
	planets: Query<&Transform, With<Planet>>,
) {
	for (
		mut velocity,
		Transform {
			translation: asteroid,
			..
		},
	) in asteroids.iter_mut()
	{
		for Transform {
			translation: planet,
			..
		} in planets.iter()
		{
			let r = *planet - *asteroid;
			let f = r * G * r.length_recip().powi(3);
			velocity.0 += DELTA as f32 * f.truncate();
		}
	}
}
