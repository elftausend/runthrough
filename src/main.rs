
use macroquad::{
    hash,
    prelude::{clear_background, next_frame, vec2, WHITE, GREEN, BLUE, screen_width},
};
use runthrough::{fun_eval::lexer::find_tokens, draw_grid, draw_axis, clear_pressed_keys, input_field, draw_graph};



#[test]
fn x() {
    let calc = 2. + 2. / (1. + 2.);
    println!("c: {calc}");
}

#[macroquad::main("runthrough")]
async fn main() {
    let input = "x + 3";
    find_tokens(input);

    let mut last_input = "";
    
    loop {
        clear_pressed_keys();
        clear_background(WHITE);

        draw_grid();
        draw_axis();


        let input = input_field(vec2(10., 10.), vec2(200., 30.), hash!(), GREEN);
        if input != last_input {
        }

        draw_graph(input, GREEN);

        last_input = input;
        let input = input_field(vec2(screen_width() - 210., 10.), vec2(200., 30.), hash!(), BLUE);
        draw_graph(input, BLUE);
        next_frame().await;
    }
}
