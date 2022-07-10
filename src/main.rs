pub mod fun_eval;
mod grid;

use fun_eval::lexer::find_tokens;
//use grid::{draw_axis, draw_graph, draw_grid};
//use macroquad::prelude::{clear_background, next_frame, WHITE};

const EDGE_DISTANCE: f32 = 40.;
const AXIS_THICKNESS: f32 = 3.;
const SPACINGX: f32 = 40.;
const SPACINGY: f32 = 40.;

#[test]
fn x() {
    let calc = 2. + 2. / (1. + 2.);
    println!("c: {calc}");
}

#[test]
fn test_graplot() {
    let postfix = fun_eval::interpret::interpret_fn("x^2 + 3");

    let mut xs = vec![0.; 200];

    let mut add = -100f64;
    for x in &mut xs {
        *x = (add / 100.) * 1.;
        add += 1.;
    }

    let mut ys = vec![0.; 200];
    for (i, y) in ys.iter_mut().enumerate() {
        *y = fun_eval::interpret::postfix_eval(&postfix, xs[i]).unwrap();
    }

    let plot = graplot::Plot::new(ys);
    plot.show();
}

fn main() {}

/* 
#[macroquad::main("runthrough")]
async fn main() {
    let input = "x + 3";
    find_tokens(input);

    loop {
        clear_background(WHITE);

        draw_grid();
        draw_axis();

        draw_graph(input);

        next_frame().await;
    }
}
*/