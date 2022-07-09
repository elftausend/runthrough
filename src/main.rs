mod grid;

use grid::{draw_axis, draw_grid};
use macroquad::prelude::{
    clear_background, next_frame, WHITE,
};

const EDGE_DISTANCE: f32 = 40.;
const AXIS_THICKNESS: f32 = 3.;
const SPACINGX: f32 = 40.;
const SPACINGY: f32 = 40.;

#[macroquad::main("runthrough")]
async fn main() {
    loop {
        clear_background(WHITE);

        draw_grid();
        draw_axis();
        
        next_frame().await;
    }
}
