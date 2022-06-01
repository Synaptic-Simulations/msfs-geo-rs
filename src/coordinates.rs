use uom::si::{angle::degree, f64::Angle};

/// Coordinates represent a location on the surface of a sphere using angular distance from its equator and prime
/// meridian
pub struct Coordinates {
	/// Latitude - Angular distance from equator
	pub lat: Angle,
	/// Longitude - Angular distance from prime meridian
	pub long: Angle,
}

impl Coordinates {
	/// Instantiates a set of Coordinates from `lat` and `long` in degrees
	pub fn new(lat: f64, long: f64) -> Self {
		Coordinates {
			lat: Angle::new::<degree>(lat),
			long: Angle::new::<degree>(long),
		}
	}
}
