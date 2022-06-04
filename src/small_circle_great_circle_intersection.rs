use uom::si::{
	f64::{Angle, Length},
	length::meter,
	ratio::ratio,
};

use crate::{constants::EARTH_RADIUS, spherical::XYZ, utility::diff_angle, Coordinates, Direction::Either};

pub fn solve_with_permutations(
	small_circle_xyz: XYZ, ns: XYZ, small_circle_radius: Length, permutations: [[usize; 3]; 3],
) -> Option<(Coordinates, Coordinates)> {
	let mut permutation = permutations[0];
	let mut denominator = ns[permutation[2]].value * small_circle_xyz[permutation[1]].value
		- ns[permutation[1]].value * small_circle_xyz[permutation[2]].value;
	let mut i = 1;
	while denominator.abs() < 1e-4 && i < 3 {
		permutation = permutations[i];
		denominator = ns[permutation[2]].value * small_circle_xyz[permutation[1]].value
			- ns[permutation[1]].value * small_circle_xyz[permutation[2]].value;
		i += 1;
	}

	let a = (-ns[permutation[2]].value * (small_circle_radius.value.powi(2) - 2.0 * EARTH_RADIUS.value.powi(2)))
		/ 2.0 / denominator;
	let b: f64 = -(ns[permutation[2]].value * small_circle_xyz[permutation[0]].value
		- ns[permutation[0]].value * small_circle_xyz[permutation[2]].value)
		/ denominator;
	let c = (ns[permutation[1]].value * (small_circle_radius.value.powi(2) - 2.0 * EARTH_RADIUS.value.powi(2)))
		/ 2.0 / denominator;
	let d: f64 = -(ns[permutation[1]].value * small_circle_xyz[permutation[0]].value
		- ns[permutation[0]].value * small_circle_xyz[permutation[1]].value)
		/ denominator;

	let discriminant = -c.powi(2) * (1.0 + b.powi(2)) + 2.0 * a * b * c * d - a.powi(2) * (1.0 + d.powi(2))
		+ (1.0 + b.powi(2) + d.powi(2)) * EARTH_RADIUS.value.powi(2);

	if discriminant < 0.0 {
		return None;
	}

	let mut result1 = [0.0, 0.0, 0.0];
	let mut result2 = [0.0, 0.0, 0.0];

	result1[permutation[0]] = (-a * b - c * d - discriminant.sqrt()) / (1.0 + b.powi(2) + d.powi(2));
	result2[permutation[0]] = (-a * b - c * d + discriminant.sqrt()) / (1.0 + b.powi(2) + d.powi(2));

	result1[permutation[1]] = a + b * result1[permutation[0]];
	result2[permutation[1]] = a + b * result2[permutation[0]];

	result1[permutation[2]] = c + d * result1[permutation[0]];
	result2[permutation[2]] = c + d * result2[permutation[0]];

	Some((
		XYZ::new(
			Length::new::<meter>(result1[0]),
			Length::new::<meter>(result1[1]),
			Length::new::<meter>(result1[2]),
		)
		.into(),
		XYZ::new(
			Length::new::<meter>(result2[0]),
			Length::new::<meter>(result2[1]),
			Length::new::<meter>(result2[2]),
		)
		.into(),
	))
}

impl Coordinates {
	pub fn small_circle_great_circle_intersection(
		self, radius: Length, great_circle_reference: Coordinates, great_circle_bearing: Angle,
	) -> Option<(Coordinates, Coordinates)> {
		let small_circle_xyz: XYZ = self.into();
		let great_circle_xyz: XYZ = great_circle_reference.into();

		let v = great_circle_reference.calculate_v(great_circle_bearing);

		let normal_vector = XYZ::new(
			great_circle_xyz[1] * v.z.get::<ratio>() - great_circle_xyz[2] * v.y.get::<ratio>(),
			great_circle_xyz[2] * v.x.get::<ratio>() - great_circle_xyz[0] * v.z.get::<ratio>(),
			great_circle_xyz[0] * v.y.get::<ratio>() - great_circle_xyz[1] * v.x.get::<ratio>(),
		);

		solve_with_permutations(
			small_circle_xyz,
			normal_vector,
			radius,
			[[0, 1, 2], [2, 0, 1], [1, 2, 0]],
		)
	}

	/// Returns the **first** intersection to occur between a circle at `radius` around `self` with a great circle with
	/// `bearing`, passing through point `bearing_reference`.
	///
	/// "**first**" means, the first intersection in the direction of the `bearing` from `bearing_reference`. If there
	/// is an intersection close behind the reference, then the returned point may be on the other side of the planet.
	///
	/// In this case, reversing the direction of the bearing (adding or subtracting 180&deg;) would yield the
	/// intersection close behind the reference instead.
	pub fn first_small_circle_intersection(
		self, radius: Length, bearing_reference: Coordinates, bearing: Angle,
	) -> Option<Coordinates> {
		return match self.small_circle_great_circle_intersection(radius, bearing_reference, bearing) {
			None => None,
			Some(intercepts) => {
				return if bearing_reference.distance_to(self) <= radius {
					// The great circle reference is inside the circle, use the intercept which is in-front of the great
					// circle reference as per the great circle bearing
					if diff_angle(bearing, bearing_reference.bearing_to(intercepts.0), Either).abs()
						<= Angle::HALF_TURN / 2.0
					{
						Some(intercepts.0)
					} else {
						Some(intercepts.1)
					}
				} else if diff_angle(bearing, bearing_reference.bearing_to(self), Either).abs()
					<= Angle::HALF_TURN / 2.0
				{
					// The small circle centre is in-front of the great circle reference, use the closest intercept
					if bearing_reference.distance_to(intercepts.0) < bearing_reference.distance_to(intercepts.1) {
						Some(intercepts.0)
					} else {
						Some(intercepts.1)
					}
				} else {
					// The small circle centre is behind the great circle reference, use the furthest intercept
					if bearing_reference.distance_to(intercepts.0) > bearing_reference.distance_to(intercepts.1) {
						Some(intercepts.0)
					} else {
						Some(intercepts.1)
					}
				};
			},
		};
	}

	/// Returns the **closest** intersection to occur between a circle at `radius` around `self` with a great circle
	/// with `bearing`, passing through point `bearing_reference`
	///
	/// "**closest**" means, the intersection that is the least distance from `bearing_reference`.
	///
	/// This means, unlike `first_small_circle_intersection`, reversing the direction of the bearing (adding or
	/// subtracting 180&deg;) would not change the intersection which is returned.
	pub fn closest_small_circle_intersection(
		self, radius: Length, great_circle_reference: Coordinates, great_circle_bearing: Angle,
	) -> Option<Coordinates> {
		return match self.small_circle_great_circle_intersection(radius, great_circle_reference, great_circle_bearing) {
			None => None,
			Some(intercepts) => {
				return if great_circle_reference.distance_to(intercepts.0)
					< great_circle_reference.distance_to(intercepts.1)
				{
					Some(intercepts.0)
				} else {
					Some(intercepts.1)
				}
			},
		};
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
	fn test_small_circle_intersection_none() {
		match Coordinates::new(0.0, 0.0).small_circle_great_circle_intersection(
			Length::new::<nautical_mile>(59.0),
			Coordinates::new(0.0, 1.0),
			Angle::ZERO,
		) {
			None => (),
			_ => panic!(),
		}
	}

	#[test]
	fn test_small_circle_intersection_same_long() {
		match Coordinates::new(90.0, 0.0).small_circle_great_circle_intersection(
			Length::new::<nautical_mile>(5.0),
			Coordinates::new(85.0, 10.0),
			Angle::ZERO,
		) {
			None => panic!(),
			Some(intersections) => {
				assert_uom_eq!(intersections.0.long, Angle::new::<degree>(-170.0));
				assert_uom_eq!(intersections.1.long, Angle::new::<degree>(10.0));
			},
		}
	}
	// TODO: More Tests
}
