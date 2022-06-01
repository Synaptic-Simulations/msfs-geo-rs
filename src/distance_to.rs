use uom::si::{angle::radian, f64::Length};

use crate::{constants::EARTH_RADIUS, coordinates::Coordinates};

impl Coordinates {
	pub fn distance_to(self, to: Coordinates) -> Length {
		let lat1 = self.lat.get::<radian>();
		let lat2 = to.lat.get::<radian>();

		let delta_lat = (to.lat - self.lat).get::<radian>();
		let delta_long = (to.long - self.long).get::<radian>();

		let a = (delta_lat / 2.0).sin() * (delta_lat / 2.0).sin()
			+ lat1.cos() * lat2.cos() * (delta_long / 2.0).sin() * (delta_long / 2.0).sin();

		let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

		return EARTH_RADIUS * c;
	}
}

#[cfg(test)]
mod tests {
	use uom::si::angle::degree;

	use super::*;

	#[test]
	fn test_distance_to() {
		let bearing = Coordinates::new(39.778889, -104.9825).bearing_to(Coordinates::new(43.778889, -102.9825));
		assert_eq!(bearing.get::<degree>(), 19.787524850709293);

		let bearing = Coordinates::new(51.5104, 7.3256).bearing_to(Coordinates::new(43.778889, 7.491));
		assert_eq!(bearing.get::<degree>(), 179.11237166124724);
	}
}
