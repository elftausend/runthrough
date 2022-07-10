use std::{cell::RefCell, collections::HashMap};

use macroquad::{
    input,
    prelude::{
        draw_line, draw_rectangle, draw_text, screen_height, screen_width, MouseButton,
        Vec2, DARKGRAY, GRAY, GREEN, LIGHTGRAY, WHITE,
    },
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

#[derive(Debug, Clone)]
pub struct InputField {
    active: bool,
    text: String,
}

pub struct InputFieldCache {
    fields: HashMap<u64, InputField>,
}

impl InputFieldCache {
    pub fn add(&mut self, id: u64) -> &mut InputField {
        let field = InputField {
            active: false,
            text: String::new(),
        };
        self.fields.insert(id, field);
        self.fields.get_mut(&id).unwrap()
    }
    pub fn get(id: u64) -> &'static mut InputField {
        INPUT_FIELDS.with(|cache| {
            let cache = cache.as_ptr();

            unsafe {
                let field = (*cache).fields.get_mut(&id);
                match field {
                    Some(field) => field,
                    None => (*cache).add(id),
                }
            }
        })
    }
}

thread_local! {
    pub static INPUT_FIELDS: RefCell<InputFieldCache> = RefCell::new(InputFieldCache { fields: HashMap::new() })
}

pub fn input_field(pos: Vec2, size: Vec2, id: u64) {
    let field = InputFieldCache::get(id);
    draw_rectangle(pos.x, pos.y, size.x, size.y, LIGHTGRAY);
    draw_rectangle(pos.x + 1.5, pos.y + 1.5, size.x - 2.5, size.y - 3., WHITE);

    let (x, y) = input::mouse_position();
    if x >= pos.x && x <= size.x && y >= pos.y && y <= size.y {
        if input::is_mouse_button_pressed(MouseButton::Left) {
            field.active = true;
        }
    } else if input::is_mouse_button_pressed(MouseButton::Left) {
        field.active = false;
    }

    draw_text(&field.text, pos.x + 1.5, pos.y + size.y / 1.5, 21., GREEN);

    if !field.active {
        return;
    }

    if let Some(pressed) = input::get_char_pressed() {
        if pressed as u8 == 127 {
            field.text.pop();
        } else if field.text.len() <= (size.x / (21. * 0.5)).ceil() as usize {
            field.text.push(pressed);
        }
        
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
