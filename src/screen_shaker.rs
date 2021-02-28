use bevy::prelude::*;

use crate::constants::{
	SCREEN_SHAKE_DAMPENING, SCREEN_SHAKE_INTENSITY, SCREEN_SHAKE_SPEED, SCREEN_SHAKE_TIMER_DURATION,
};

pub struct ScreenShakePlugin;

impl Plugin for ScreenShakePlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_resource(ScreenShakeTimer::new(SCREEN_SHAKE_TIMER_DURATION))
			.add_system(screen_shaker.system());
	}
}

#[derive(Bundle, Default)]
pub struct ScreenShakeBundle {
	screen_shaker: ScreenShaker,
	transform: Transform,
	global_transform: GlobalTransform,
}

#[derive(Default)]
pub struct ScreenShaker {
	pub direction: Vec2,
}

pub struct ScreenShakeTimer(pub Timer);

impl ScreenShakeTimer {
	pub fn new(seconds: f32) -> Self {
		let mut timer = Timer::from_seconds(seconds, false);
		timer.set_elapsed(seconds);
		Self(timer)
	}
}

pub fn screen_shaker(
	time: Res<Time>,
	mut timer: ResMut<ScreenShakeTimer>,
	mut query: Query<(&mut Transform, &ScreenShaker)>,
) {
	// Compute offset based on elapsed time
	let offset = if !timer.0.finished() {
		timer.0.tick(time.delta_seconds());
		// Offset is a cosine weighted by a negative exponential
		let elapsed = timer.0.elapsed();
		(-elapsed * SCREEN_SHAKE_DAMPENING).exp()
			* (std::f32::consts::TAU * elapsed * SCREEN_SHAKE_SPEED).cos()
	} else {
		0.
	};

	// Apply offset to camera
	for (mut transform, ScreenShaker { direction }) in query.iter_mut() {
		let displacement = offset * SCREEN_SHAKE_INTENSITY * *direction;
		transform.translation = displacement.extend(0.);
	}
}
