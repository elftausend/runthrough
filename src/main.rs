pub mod fun_eval;
mod grid;

use fun_eval::lexer::find_tokens;
use grid::{draw_axis, draw_grid};
use macroquad::prelude::{clear_background, next_frame, WHITE};

const EDGE_DISTANCE: f32 = 40.;
const AXIS_THICKNESS: f32 = 3.;
const SPACINGX: f32 = 40.;
const SPACINGY: f32 = 40.;

#[test]
fn x() {
    let calc = 2. + 2. / (1. + 2.);
    println!("c: {calc}");
}

#[macroquad::main("runthrough")]
async fn main() {
    let input = "x + 3";
    find_tokens(input);

    loop {
        clear_background(WHITE);

        draw_grid();
        draw_axis();

        next_frame().await;
    }
}
