use std::marker::PhantomData;

use uom::si::f64::{Angle, Length};

pub const EARTH_RADIUS: Length = Length {
	dimension: PhantomData,
	units: PhantomData,
	value: 6378137.0,
};

/// Latitude of the south pole - Lowest possible latitude
pub const MIN_LAT: Angle = Angle {
	dimension: PhantomData,
	units: PhantomData,
	value: -std::f64::consts::FRAC_PI_2,
};

/// Latitude of the north pole - Highest possible latitude
pub const MAX_LAT: Angle = Angle {
	dimension: PhantomData,
	units: PhantomData,
	value: std::f64::consts::FRAC_PI_2,
};

/// Lowest possible longitude - Opposite side of sphere from prime meridian - Same Location as `MAX_LONG` but with a
/// negative value
pub const MIN_LONG: Angle = Angle {
	dimension: PhantomData,
	units: PhantomData,
	value: -std::f64::consts::PI,
};

/// Highest possible longitude - Opposite side of sphere from prime meridian - Same Location as `M_LONG` but with a
/// positive value
pub const MAX_LONG: Angle = Angle {
	dimension: PhantomData,
	units: PhantomData,
	value: std::f64::consts::PI,
};
