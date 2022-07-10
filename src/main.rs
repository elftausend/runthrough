pub mod fun_eval;
mod grid;

use fun_eval::lexer::find_tokens;
use grid::{draw_axis, draw_graph, draw_grid, input_field, clear_pressed_keys};
use macroquad::{
    hash,
    prelude::{clear_background, next_frame, vec2, WHITE},
};

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

    let mut data0 = String::new();
    let mut data1 = String::new();

    loop {
        clear_pressed_keys();
        clear_background(WHITE);

        draw_grid();
        draw_axis();

        draw_graph(input);

        input_field(vec2(10., 10.), vec2(200., 30.), hash!());
        input_field(vec2(10., 100.), vec2(200., 30.), hash!());

        next_frame().await;
    }
}
