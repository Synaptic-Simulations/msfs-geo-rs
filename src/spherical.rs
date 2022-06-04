use nalgebra::Matrix1x3;
use uom::{
	si::{
		angle::radian,
		f64::{Angle, Length, Ratio},
		ratio::ratio,
	},
	ConstZero,
};

use crate::{constants::EARTH_RADIUS, Coordinates};

pub type Spherical = Matrix1x3<f64>;
pub type XYZ = Matrix1x3<Length>;
pub type UnitVector = Matrix1x3<Ratio>;

impl Coordinates {
	pub fn calculate_v(self, course: Angle) -> Matrix1x3<Ratio> {
		let theta_unit = self.theta_unit_vector();
		let phi_unit = self.phi_unit_vector();

		Matrix1x3::new(
			-course.cos() * theta_unit[0] + course.sin() * phi_unit[0],
			-course.cos() * theta_unit[1] + course.sin() * phi_unit[1],
			-course.cos() * theta_unit[2] + course.sin() * phi_unit[2],
		)
	}

	pub fn theta_unit_vector(self) -> UnitVector {
		let theta = self.theta();
		let phi = self.phi();

		UnitVector::new(theta.cos() * phi.cos(), theta.cos() * phi.sin(), -theta.sin())
	}

	pub fn phi_unit_vector(self) -> UnitVector {
		let phi = self.phi();

		UnitVector::new(phi.sin(), phi.cos(), Ratio::ZERO)
	}

	pub fn theta(self) -> Angle { Angle::HALF_TURN / 2.0 - self.lat }

	pub fn phi(self) -> Angle {
		if self.long < Angle::ZERO {
			self.long + Angle::FULL_TURN
		} else {
			self.long
		}
	}

	pub fn from_theta_phi(theta: Angle, phi: Angle) -> Self {
		Self {
			lat: Angle::HALF_TURN / 2.0 - theta,
			long: if phi > Angle::HALF_TURN {
				Angle::HALF_TURN - phi
			} else {
				phi
			},
		}
	}
}

impl From<Spherical> for Coordinates {
	fn from(spherical: Spherical) -> Self {
		Self {
			lat: Angle::new::<radian>(spherical[2].asin()),
			long: Angle::new::<radian>(spherical[1].atan2(spherical[0])),
		}
	}
}

impl From<Coordinates> for Spherical {
	fn from(coordinates: Coordinates) -> Self {
		Self::new(
			coordinates.lat.cos().get::<ratio>() * coordinates.long.cos().get::<ratio>(),
			coordinates.lat.cos().get::<ratio>() * coordinates.long.sin().get::<ratio>(),
			coordinates.lat.sin().get::<ratio>(),
		)
	}
}

impl From<XYZ> for Coordinates {
	fn from(xyz: XYZ) -> Self {
		let x = xyz.x;
		let y = xyz.y;
		let z = xyz.z;

		let theta = (x * x + y * y).sqrt().atan2(z);

		let phi;
		if x > Length::ZERO {
			phi = (y / x).atan();
		} else if x < Length::ZERO && y >= Length::ZERO {
			phi = (y / x).atan() + Angle::HALF_TURN;
		} else if x < Length::ZERO && y < Length::ZERO {
			phi = (y / x).atan() - Angle::HALF_TURN;
		} else if x == Length::ZERO && y > Length::ZERO {
			phi = Angle::HALF_TURN;
		} else {
			phi = -Angle::HALF_TURN;
		}

		Self::from_theta_phi(theta, phi)
	}
}

impl From<Coordinates> for XYZ {
	fn from(coordinates: Coordinates) -> Self {
		let theta = coordinates.theta();
		let phi = coordinates.phi();

		Self::new(
			EARTH_RADIUS * theta.sin() * phi.cos(),
			EARTH_RADIUS * theta.sin() * phi.sin(),
			EARTH_RADIUS * theta.cos(),
		)
	}
}
