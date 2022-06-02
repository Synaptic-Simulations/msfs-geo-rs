use uom::si::{
	angle::radian,
	f64::{Angle, Length},
	ratio::ratio,
};

use crate::{
	constants::{EARTH_RADIUS, MAX_LAT, MAX_LONG, MIN_LAT, MIN_LONG},
	coordinates::Coordinates,
};

impl Coordinates {
	/// Returns the Southwest and Northeast corner of a box around coordinates with a minimum `distance`
	pub fn distance_bounds(self, distance: Length) -> (Coordinates, Coordinates) {
		let radial_distance: Angle = Angle::new::<radian>((distance / EARTH_RADIUS).get::<ratio>());

		let mut low_lat = self.lat - radial_distance;
		let mut high_lat = self.lat + radial_distance;

		let mut low_long;
		let mut high_long;

		if low_lat > MIN_LAT && high_lat < MAX_LAT {
			let delta_long = (radial_distance.sin().get::<ratio>() / self.lat.cos()).asin();
			low_long = self.long - delta_long;

			if low_long < MIN_LONG {
				low_long += Angle::FULL_TURN;
			}

			high_long = self.long + delta_long;

			if high_long > MAX_LONG {
				high_long -= Angle::FULL_TURN;
			}
		} else {
			low_lat = low_lat.max(MIN_LAT);
			high_lat = high_lat.max(MAX_LAT);

			low_long = MIN_LONG;
			high_long = MIN_LONG;
		}

		(
			Coordinates {
				lat: low_lat,
				long: low_long,
			},
			Coordinates {
				lat: high_lat,
				long: high_long,
			},
		)
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
	fn test_distance_bounds() {
		let (south_west, north_east) = Coordinates {
			lat: Angle::ZERO,
			long: Angle::ZERO,
		}
		.distance_bounds(Length::new::<nautical_mile>(60.0));
		assert_uom_eq!(south_west.long, Angle::new::<degree>(-1.0), 1e-4);
		assert_uom_eq!(north_east.long, Angle::new::<degree>(1.0), 1e-4);
		assert_uom_eq!(south_west.lat, Angle::new::<degree>(-1.0), 1e-4);
		assert_uom_eq!(north_east.lat, Angle::new::<degree>(1.0), 1e-4);
	}

	#[test]
	fn test_distance_bounds_long_cross() {
		let (south_west, north_east) = Coordinates {
			lat: Angle::ZERO,
			long: Angle::new::<degree>(179.0),
		}
		.distance_bounds(Length::new::<nautical_mile>(120.0));
		assert_uom_eq!(south_west.long, Angle::new::<degree>(177.0), 1e-4);
		assert_uom_eq!(north_east.long, Angle::new::<degree>(-179.0), 1e-4);
		assert_uom_eq!(south_west.lat, Angle::new::<degree>(-2.0), 1e-4);
		assert_uom_eq!(north_east.lat, Angle::new::<degree>(2.0), 1e-4);
	}
}
