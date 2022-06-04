use nalgebra::Matrix1x3;
use uom::{
	num_traits::Pow,
	si::{
		angle::radian,
		f64::{Angle, Length, Ratio},
		length::meter,
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
			-course.cos() * theta_unit[0].get::<ratio>() + course.sin() * phi_unit[0].get::<ratio>(),
			-course.cos() * theta_unit[1].get::<ratio>() + course.sin() * phi_unit[1].get::<ratio>(),
			-course.cos() * theta_unit[2].get::<ratio>() + course.sin() * phi_unit[2].get::<ratio>(),
		)
	}

	pub fn theta_unit_vector(self) -> UnitVector {
		let theta = self.theta();
		let phi = self.phi();

		UnitVector::new(
			theta.cos() * phi.cos().get::<ratio>(),
			theta.cos() * phi.sin().get::<ratio>(),
			-theta.sin(),
		)
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

		let theta = Angle::new::<radian>(
			(x.get::<meter>().pow(2.0 as f64) + y.get::<meter>().pow(2.0 as f64))
				.sqrt()
				.atan2(z.get::<meter>()),
		);

		let phi;
		if x > Length::ZERO {
			phi = Ratio::new::<ratio>(y.value / x.value).atan();
		} else if x < Length::ZERO && y >= Length::ZERO {
			phi = Ratio::new::<ratio>(y.value / x.value).atan() + Angle::HALF_TURN;
		} else if x < Length::ZERO && y < Length::ZERO {
			phi = Ratio::new::<ratio>(y.value / x.value).atan() - Angle::HALF_TURN;
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
			EARTH_RADIUS * theta.sin().get::<ratio>() * phi.cos().get::<ratio>(),
			EARTH_RADIUS * theta.sin().get::<ratio>() * phi.sin().get::<ratio>(),
			EARTH_RADIUS * theta.cos().get::<ratio>(),
		)
	}
}
