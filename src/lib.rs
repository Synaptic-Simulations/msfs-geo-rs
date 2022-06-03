mod bearing_distance;
mod bearing_to;
pub mod constants;
mod distance_bounds;
mod distance_to;
pub mod macros;
mod place_bearing_intersection;
pub mod utility;

use nalgebra::Matrix1x3;
use uom::si::{
	angle::{degree, radian},
	f64::Angle,
	ratio::ratio,
};

#[derive(PartialEq, Eq)]
pub enum Direction {
	Left,
	Right,
	Either,
}

#[derive(Copy, Clone, Default, Debug)]
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

pub type Spherical = Matrix1x3<f64>;

impl From<Spherical> for Coordinates {
	fn from(spherical: Spherical) -> Self {
		Self {
			lat: Angle::new::<radian>(spherical[2].asin()),
			long: Angle::new::<radian>(spherical[1].atan2(spherical[0])),
		}
	}
}

impl Into<Spherical> for Coordinates {
	fn into(self) -> Spherical {
		Matrix1x3::new(
			self.lat.cos().get::<ratio>() * self.long.cos().get::<ratio>(),
			self.lat.cos().get::<ratio>() * self.long.sin().get::<ratio>(),
			self.lat.sin().get::<ratio>(),
		)
	}
}
