use macroquad::{
    hash,
    prelude::{clear_background, next_frame, screen_width, vec2, BLUE, GREEN, WHITE},
};
use runthrough::{
    clear_pressed_keys, draw_axis, draw_graph, draw_grid, fun_eval::lexer::find_tokens, input_field,
};

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
        if input != last_input {}

        draw_graph(input, GREEN);

        last_input = input;
        let input = input_field(
            vec2(screen_width() - 210., 10.),
            vec2(200., 30.),
            hash!(),
            BLUE,
        );
        draw_graph(input, BLUE);
        next_frame().await;
    }
}
