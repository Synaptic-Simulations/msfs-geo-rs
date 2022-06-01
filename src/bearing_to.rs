use uom::si::{
	angle::radian,
	f64::{Angle, Ratio},
	ratio::ratio,
};

use crate::coordinates::Coordinates;

impl Coordinates {
	pub fn bearing_to(self, to: Coordinates) -> Angle {
		let lat1 = self.lat.get::<radian>();
		let lat2 = to.lat.get::<radian>();
		let long1 = self.long.get::<radian>();
		let long2 = to.long.get::<radian>();

		let y = Ratio::new::<ratio>((long2 - long1).sin() * lat2.cos());
		let x = Ratio::new::<ratio>(lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * (long2 - long1).cos());

		let theta = y.atan2(x);
		return (theta + Angle::FULL_TURN) % Angle::FULL_TURN;
	}
}

#[cfg(test)]
mod tests {
	use uom::si::angle::degree;

	use super::*;

	#[test]
	fn test_bearing_to() {
		let bearing = Coordinates::new(39.778889, -104.9825).bearing_to(Coordinates::new(43.778889, -102.9825));
		assert_eq!(bearing.get::<degree>(), 19.787524850709293);

		let bearing = Coordinates::new(51.5104, 7.3256).bearing_to(Coordinates::new(43.778889, 7.491));
		assert_eq!(bearing.get::<degree>(), 179.11237166124724);
	}
}
