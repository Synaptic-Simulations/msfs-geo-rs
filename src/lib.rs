pub mod bearing_to;
pub mod constants;
pub mod coordinates;
pub mod distance_bounds;
pub mod distance_to;
pub mod macros;
pub mod utility;

#[derive(PartialEq, Eq)]
pub enum Direction {
	Left,
	Right,
	Either,
}
