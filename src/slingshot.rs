use bevy::{
	prelude::*,
	render::camera::{Camera, OrthographicProjection},
};
use mouse_tracking::{MousePosPlugin, MousePosWorld};

use crate::{
	bodies::{Asteroid, Radius},
	constants::{SLINGSHOT_MAX_LENGTH, SLINGSHOT_STRENGTH},
	physics::Velocity,
};

pub struct Slingshot {
	pub target: Entity,
}

pub struct SlingshotPlugin;

impl Plugin for SlingshotPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app
			// Mouse stuff
			.add_plugin(MousePosPlugin::None)
			.add_resource(mouse_tracking::MousePosWorld::default())
			.add_system_to_stage(stage::EVENT, update_pos_ortho.system())
			// Slingshot stuff
			.add_system(load_slingshot.system())
			.add_system(update_slingshot.system())
			.add_system(release_slingshot.system());
	}
}

pub fn update_pos_ortho(
	mut mouse_world: ResMut<mouse_tracking::MousePosWorld>,
	mut event_reader: Local<EventReader<CursorMoved>>,
	cursor_moved: Res<Events<CursorMoved>>,
	cameras: Query<(&GlobalTransform, &OrthographicProjection), With<Camera>>,
) {
	if let Some(event) = event_reader.latest(&cursor_moved) {
		let (camera, proj) = cameras
			.iter()
			.next()
			.expect("could not find an orthographic camera");
		mouse_world.0 = event.position.extend(0.0) + Vec3::new(proj.left, proj.bottom, proj.near);
		mouse_world.0 = camera.mul_vec3(mouse_world.0);
	}
}

pub fn load_slingshot(
	commands: &mut Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mouse_button_input: Res<Input<MouseButton>>,
	mouse: Res<MousePosWorld>,
	mut asteroids: Query<(Entity, &Transform, &Radius), With<Asteroid>>,
) {
	if mouse_button_input.just_pressed(MouseButton::Left) {
		for (asteroid, transform, radius) in asteroids.iter_mut() {
			if radius.contains((transform.translation - mouse.0).truncate()) {
				commands
					.spawn(SpriteBundle {
						material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
						transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
						sprite: Sprite::new(Vec2::new(0.0, 20.0)),
						..Default::default()
					})
					.with(Slingshot { target: asteroid });
			}
		}
	}
}

pub fn update_slingshot(
	mouse_button_input: Res<Input<MouseButton>>,
	mouse: Res<MousePosWorld>,
	mut asteroids: Query<&Transform, With<Asteroid>>,
	mut slingshots: Query<(&Slingshot, &mut Sprite, &mut Transform)>,
) {
	if mouse_button_input.pressed(MouseButton::Left) {
		if let Some((slingshot, mut slingshot_sprite, mut slingshot_transform)) =
			slingshots.iter_mut().next()
		{
			if let Ok(asteroid) = asteroids.get_mut(slingshot.target) {
				let r = (asteroid.translation - mouse.0).truncate();
				let length = r.length().min(SLINGSHOT_MAX_LENGTH);
				let angle = r.y.atan2(r.x);
				let r = r.normalize() * length;
				let position = asteroid.translation.truncate() - r / 2.0;
				*slingshot_transform = Transform::from_translation(position.extend(0.0));
				slingshot_transform.rotate(Quat::from_rotation_z(angle));
				slingshot_sprite.size.x = length;
			}
		}
	}
}

pub fn release_slingshot(
	commands: &mut Commands,
	mouse_button_input: Res<Input<MouseButton>>,
	mouse: Res<MousePosWorld>,
	mut asteroids: Query<(&Transform, &mut Velocity), With<Asteroid>>,
	slingshots: Query<(Entity, &Slingshot)>,
) {
	if mouse_button_input.just_released(MouseButton::Left) {
		if let Some((entity, slingshot)) = slingshots.iter().next() {
			if let Ok((transform, mut velocity)) = asteroids.get_mut(slingshot.target) {
				let r = (transform.translation - mouse.0).truncate();
				let r = r.normalize() * r.length().min(SLINGSHOT_MAX_LENGTH);
				velocity.0 += SLINGSHOT_STRENGTH * r;
			}
			commands.despawn(entity);
		}
	}
}
