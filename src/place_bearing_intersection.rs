use std::marker::PhantomData;

use uom::si::f64::{Angle, Length};

use crate::{spherical::Spherical, utility::clamp_angle_cw, Coordinates};

const INTERMEDIATE_PLACE_DISTANCE: Length = Length {
	units: PhantomData,
	dimension: PhantomData,
	value: 926000.0,
};

impl Coordinates {
	/// Calculates the two intercept Coordinates of two great circles running through `self` and `place2` respectively,
	/// and on bearings `bearing` and `bearing2` respectively. **Note** that the great circles only have the given
	/// bearings at the given Coordinates
	pub fn place_bearing_intersection(
		self, bearing: Angle, place2: Coordinates, bearing2: Angle,
	) -> (Coordinates, Coordinates) {
		let pa11: Spherical = self.into();
		let pa12: Spherical = self.bearing_distance(bearing, INTERMEDIATE_PLACE_DISTANCE).into();
		let pa21: Spherical = place2.into();
		let pa22: Spherical = place2.bearing_distance(bearing2, INTERMEDIATE_PLACE_DISTANCE).into();

		let n1 = pa11.cross(&pa12);
		let n2 = pa21.cross(&pa22);

		let l = n1.cross(&n2);
		let i = l.norm();

		let i1 = l / i;
		let i2 = i1 * -1.0;

		let s1 = Coordinates::from(i1);
		let s2 = Coordinates::from(i2);

		let brg_to_s1 = self.bearing_to(s1);
		let brg_to_s2 = self.bearing_to(s2);

		let delta1 = (clamp_angle_cw(bearing) - brg_to_s1).abs();
		let delta2 = (clamp_angle_cw(bearing) - brg_to_s2).abs();

		if delta1 < delta2 {
			(s1, s2)
		} else {
			(s2, s1)
		}
	}
}

#[cfg(test)]
mod tests {
	use uom::{
		si::angle::{degree, radian},
		ConstZero,
	};

	use super::*;
	use crate::assert_uom_eq;

	#[test]
	fn test_place_bearing_intersection_poles() {
		let points = Coordinates::new(39.778889, -104.9825).place_bearing_intersection(
			Angle::ZERO,
			Coordinates::new(43.778889, -102.9825),
			Angle::ZERO,
		);
		assert_uom_eq!(points.0.lat, Angle::HALF_TURN / 2.0);
		assert_uom_eq!(points.1.lat, Angle::HALF_TURN / -2.0);

		let points = Coordinates::new(39.778889, -104.9825).place_bearing_intersection(
			Angle::HALF_TURN,
			Coordinates::new(43.778889, -102.9825),
			Angle::HALF_TURN,
		);
		assert_uom_eq!(points.0.lat, Angle::HALF_TURN / -2.0);
		assert_uom_eq!(points.1.lat, Angle::HALF_TURN / 2.0);
	}

	#[test]
	fn test_place_bearing_intersection_equator() {
		let points = Coordinates::new(43.0, -104.9825).place_bearing_intersection(
			Angle::new::<degree>(175.0),
			Coordinates::new(-43.0, -104.9825),
			Angle::new::<degree>(5.0),
		);
		assert_uom_eq!(points.0.lat, Angle::ZERO);
		assert_uom_eq!(points.1.lat, Angle::ZERO);
	}

	#[test]
	fn test_place_bearing_intersection_triangle() {
		let points = Coordinates::new(-43.0, 0.0).place_bearing_intersection(
			Angle::HALF_TURN / -4.0,
			Coordinates::new(-43.0, -90.0),
			Angle::HALF_TURN / 4.0,
		);
		assert_uom_eq!(points.0.lat, Angle::new::<radian>(0.29828558585826787));
		assert_uom_eq!(points.0.long, Angle::HALF_TURN / -4.0);
		assert_uom_eq!(points.1.long, Angle::FULL_TURN / (8.0 / 3.0));
	}
}
