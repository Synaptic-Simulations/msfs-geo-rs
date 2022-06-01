pub mod bearing_to;
pub mod constants;
pub mod coordinates;
pub mod distance_to;
pub mod utility;

#[derive(PartialEq, Eq)]
pub enum Direction {
	Left,
	Right,
	Either,
}
