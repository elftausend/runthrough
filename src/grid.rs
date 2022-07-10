

use macroquad::prelude::{
        draw_line, screen_height, screen_width, DARKGRAY, GRAY,
    };

use crate::{
    fun_eval::interpret::{interpret_fn, postfix_eval},
    AXIS_THICKNESS, EDGE_DISTANCE, SPACINGX, SPACINGY,
};

pub fn draw_axis() {
    // x axis
    draw_line(
        0.,
        screen_height() - EDGE_DISTANCE,
        screen_width(),
        screen_height() - EDGE_DISTANCE,
        AXIS_THICKNESS,
        DARKGRAY,
    );
    // y axis
    draw_line(
        EDGE_DISTANCE,
        0.,
        EDGE_DISTANCE,
        screen_height(),
        AXIS_THICKNESS,
        DARKGRAY,
    );
}

pub fn draw_grid() {
    let vertical_grid_lines = screen_height() / SPACINGY;
    for line in 1..vertical_grid_lines as usize {
        let y_coord = screen_height() - EDGE_DISTANCE - line as f32 * SPACINGY;
        draw_line(
            0.,
            y_coord,
            screen_width(),
            y_coord,
            AXIS_THICKNESS - 1.8,
            GRAY,
        );
    }

    let horizontal_grid_lines = screen_width() / SPACINGX;
    for line in 1..horizontal_grid_lines as usize {
        let x_coord = EDGE_DISTANCE + line as f32 * SPACINGX;
        draw_line(
            x_coord,
            0.,
            x_coord,
            screen_height(),
            AXIS_THICKNESS - 1.8,
            GRAY,
        );
    }
}

pub fn draw_graph(fun: &str) {
    let postfix = interpret_fn(fun);

    let mut xs = vec![0.; 200];

    let mut add = -100f64;
    for x in &mut xs {
        *x = (add / 100.) * 1.;
        add += 1.;
    }

    let mut ys = vec![0.; 200];
    for (i, y) in ys.iter_mut().enumerate() {
        *y = postfix_eval(&postfix, xs[i]).unwrap();
    }
}
