pub mod fun_eval;
mod grid;
mod text_field;

pub use fun_eval::*;
pub use grid::*;
pub use text_field::*;

const EDGE_DISTANCE: f32 = 40.;
const AXIS_THICKNESS: f32 = 3.;
const SPACINGX: f32 = 40.;
const SPACINGY: f32 = 40.;
