use uom::si::f64::Angle;

use crate::Coordinates;

impl Coordinates {
	pub fn bearing_to(self, to: Coordinates) -> Angle {
		let y = (to.long - self.long).sin() * to.lat.cos();
		let x = self.lat.cos() * to.lat.sin() - self.lat.sin() * to.lat.cos() * (to.long - self.long).cos();

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
