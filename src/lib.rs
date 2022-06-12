extern crate core;

mod bearing_distance;
mod bearing_to;
pub mod constants;
mod distance_bounds;
mod distance_to;
pub mod macros;
mod place_bearing_intersection;
mod small_circle_great_circle_intersection;
mod spherical;
pub mod utility;

use uom::si::{angle::degree, f64::Angle};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
	Left,
	Right,
	Either,
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
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
