use uom::si::f64::{Angle, Length};

use crate::{constants::EARTH_RADIUS, utility::clamp_angle_either, Coordinates};

impl Coordinates {
	/// Returns Coordinates on a given `bearing` at a given `distance` from Coordinates `self`
	pub fn bearing_distance(self, bearing: Angle, distance: Length) -> Coordinates {
		let radial_distance: Angle = (distance / EARTH_RADIUS).into();
		let lat =
			(self.lat.sin() * radial_distance.cos() + self.lat.cos() * radial_distance.sin() * bearing.cos()).asin();

		let long = clamp_angle_either(
			self.long
				+ (bearing.sin() * radial_distance.sin() * self.lat.cos())
					.atan2(radial_distance.cos() - self.lat.sin() * lat.sin()),
		);

		Coordinates { lat, long }
	}
}

#[cfg(test)]
mod tests {
	use uom::{
		si::{angle::degree, length::nautical_mile},
		ConstZero,
	};

	use super::*;
	use crate::assert_uom_eq;

	#[test]
	fn test_bearing_distance() {
		let point = Coordinates::new(52.518611, 13.408056)
			.bearing_distance(Angle::HALF_TURN, Length::new::<nautical_mile>(8.09935205184));
		assert_uom_eq!(point.lat, Angle::new::<degree>(52.383863707381906));
		assert_uom_eq!(point.long, Angle::new::<degree>(13.408056));

		let point = Coordinates::new(52.518611, 13.408056)
			.bearing_distance(Angle::new::<degree>(135.0), Length::new::<nautical_mile>(8.09935205184));
		assert_uom_eq!(point.lat, Angle::new::<degree>(52.4232272267234));
		assert_uom_eq!(point.long, Angle::new::<degree>(13.564299057246314));
	}

	#[test]
	fn test_inverse_meridian_cross() {
		let point =
			Coordinates::new(10.0, 175.0).bearing_distance(Angle::HALF_TURN / 2.0, Length::new::<nautical_mile>(300.0));
		assert_uom_eq!(point.lat, Angle::new::<degree>(9.9616956596193304563));
		assert_uom_eq!(point.long, Angle::new::<degree>(-179.9323633476183204));
	}

	#[test]
	fn test_vertical() {
		let point = Coordinates::new(10.0, 175.0).bearing_distance(Angle::ZERO, Length::new::<nautical_mile>(300.0));
		assert_uom_eq!(point.lat, Angle::new::<degree>(14.991039718605245312));
		assert_uom_eq!(point.long, Angle::new::<degree>(175.0));

		let point = Coordinates::new(86.0, -50.0).bearing_distance(Angle::ZERO, Length::new::<nautical_mile>(300.0));
		assert_uom_eq!(point.lat, Angle::new::<degree>(89.008960281652903745));
		assert_uom_eq!(point.long, Angle::new::<degree>(130.0));
	}
}
