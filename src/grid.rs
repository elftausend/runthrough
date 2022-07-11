

use macroquad::prelude::{
        draw_line, screen_height, screen_width, DARKGRAY, GRAY, Color,
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

pub fn draw_graph(fun: &str, color: Color) {

    if fun.is_empty() {
        return;
    }

    let postfix = match interpret_fn(fun) {
        Ok(postfix) => postfix,
        Err(_) => return,
    };

    let mut xs = vec![0.; 20000];

    let mut add = -10000f64;
    for x in &mut xs {
        *x = (add / 10000.) * 30.;
        add += 1.;
    }

    let mut ys = vec![0.; 20000];
    for (i, y) in ys.iter_mut().enumerate() {
        *y = postfix_eval(&postfix, xs[i]).unwrap();
    }

    //let xs = [1., 2., 3., 4., 5.,];
    //let ys = [1., 2., 3., 4., 5.,];

    let mut coords = Vec::new();

    for i in 0..xs.len() {
        let x = SPACINGX * xs[i] as f32 + EDGE_DISTANCE;
        let y = -SPACINGY * ys[i] as f32 + screen_height() - EDGE_DISTANCE;
        
        coords.push((x, y));

        if coords.len() >= 2 {
            draw_line(
                coords[0].0,
                coords[0].1,
                coords[1].0,
                coords[1].1,
                3.,
                color);
            coords.remove(0);
        } 
        
    }
    
}
