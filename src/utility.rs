use uom::{si::f64::Angle, ConstZero};

use crate::Direction;

/// Takes an `angle`, and returns it clamped to between 0 and 360 degrees
/// # Examples
/// `clamp_angle(361) -> 1`
///
/// `clamp_angle(400) -> 40`
///
/// `clamp_angle(-50) -> 310`
pub fn clamp_angle_cw(mut angle: Angle) -> Angle {
	while angle >= Angle::FULL_TURN {
		angle -= Angle::FULL_TURN;
	}
	while angle < Angle::ZERO {
		angle += Angle::FULL_TURN;
	}
	angle
}

/// Takes an `angle`, and returns it clamped to between 0 and -360 degrees
/// # Examples
/// `clamp_angle(361) -> -359`
///
/// `clamp_angle(-400) -> -40`
///
/// `clamp_angle(-50) -> -50`
pub fn clamp_angle_acw(mut angle: Angle) -> Angle {
	while angle <= -Angle::FULL_TURN {
		angle += Angle::FULL_TURN;
	}
	while angle > Angle::ZERO {
		angle -= Angle::FULL_TURN;
	}
	angle
}

/// Takes an `angle`, and returns it clamped to between -180 and 180 degrees
/// # Examples
/// `clamp_angle(361) -> 1`
///
/// `clamp_angle(200) -> -160`
///
/// `clamp_angle(-50) -> -50`
pub fn clamp_angle_either(mut angle: Angle) -> Angle {
	while angle > Angle::HALF_TURN {
		angle -= Angle::FULL_TURN;
	}
	while angle <= -Angle::HALF_TURN {
		angle += Angle::FULL_TURN;
	}
	angle
}

/// Takes two angles, and returns the angular difference, based on `direction`
/// # Returns
/// `direction: AntiClockwise -> 0 to -360`
///
/// `direction: Clockwise -> 0 to 360`
///
/// `direction: Either -> 180 to -180`
pub fn diff_angle(a: Angle, b: Angle, direction: Direction) -> Angle {
	let diff = b - a;
	match direction {
		Direction::Left => clamp_angle_acw(diff),
		Direction::Right => clamp_angle_cw(diff),
		Direction::Either => clamp_angle_either(diff),
	}
}

#[cfg(test)]
mod tests {
	use uom::si::f64::Angle;

	use super::*;

	#[test]
	fn test_clamp_angle_cw() {
		assert_eq!(clamp_angle_cw(3.0 * Angle::HALF_TURN), Angle::HALF_TURN);
		assert_eq!(clamp_angle_cw(-3.0 * Angle::HALF_TURN), Angle::HALF_TURN);
		assert_eq!(clamp_angle_cw(Angle::HALF_TURN), Angle::HALF_TURN);
		assert_eq!(clamp_angle_cw(-Angle::HALF_TURN), Angle::HALF_TURN);
	}
	#[test]
	fn test_clamp_angle_acw() {
		assert_eq!(clamp_angle_acw(3.0 * Angle::HALF_TURN), -Angle::HALF_TURN);
		assert_eq!(clamp_angle_acw(-3.0 * Angle::HALF_TURN), -Angle::HALF_TURN);
		assert_eq!(clamp_angle_acw(-Angle::HALF_TURN), -Angle::HALF_TURN);
		assert_eq!(clamp_angle_acw(Angle::HALF_TURN), -Angle::HALF_TURN);
	}
	#[test]
	fn test_clamp_angle_either() {
		assert_eq!(clamp_angle_either(1.5 * Angle::HALF_TURN), -Angle::HALF_TURN / 2.0);
		assert_eq!(clamp_angle_either(-1.5 * Angle::HALF_TURN), Angle::HALF_TURN / 2.0);
		assert_eq!(clamp_angle_either(-Angle::HALF_TURN / 2.0), -Angle::HALF_TURN / 2.0);
		assert_eq!(clamp_angle_either(Angle::HALF_TURN / 2.0), Angle::HALF_TURN / 2.0);
	}
	#[test]
	fn test_diff_angle() {
		assert_eq!(
			diff_angle(Angle::HALF_TURN / 2.0, Angle::HALF_TURN, Direction::Right),
			Angle::HALF_TURN / 2.0
		);
		assert_eq!(
			diff_angle(Angle::HALF_TURN / 2.0, Angle::HALF_TURN, Direction::Left),
			Angle::HALF_TURN * -1.5
		);
		assert_eq!(
			diff_angle(Angle::ZERO, Angle::HALF_TURN / 2.0, Direction::Either),
			Angle::HALF_TURN / 2.0
		);
		assert_eq!(
			diff_angle(Angle::ZERO, -Angle::HALF_TURN / 2.0, Direction::Either),
			Angle::HALF_TURN / -2.0
		);
	}
}
