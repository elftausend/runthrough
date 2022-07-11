use std::{cell::RefCell, collections::HashMap};

use macroquad::{
    input,
    prelude::{
        draw_line, draw_rectangle, draw_text, Color, MouseButton, Vec2, LIGHTGRAY, MAGENTA,
        WHITE,
    },
};

#[derive(Debug, Clone)]
pub struct InputField {
    active: bool,
    text: String,
    insert_idx: usize,
}

pub struct InputFieldCache {
    fields: HashMap<u64, InputField>,
}

impl InputFieldCache {
    pub fn add(&mut self, id: u64) -> &mut InputField {
        let field = InputField {
            active: false,
            text: String::new(),
            insert_idx: 0,
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

pub fn clear_pressed_keys() {
    let no_field_active = INPUT_FIELDS.with(|cache| {
        let cache = cache.borrow();
        for field in &cache.fields {
            if field.1.active {
                return false;
            }
        }
        true
    });
    if no_field_active {
        input::get_char_pressed();
    }
}

pub fn input_field(pos: Vec2, size: Vec2, id: u64, text_color: Color) -> &'static str {
    let field = InputFieldCache::get(id);

    let border_color = if field.active { MAGENTA } else { LIGHTGRAY };
    draw_rectangle(pos.x, pos.y, size.x, size.y, border_color);
    draw_rectangle(pos.x + 1.5, pos.y + 1.5, size.x - 2.5, size.y - 3., WHITE);

    let (x, y) = input::mouse_position();
    if x >= pos.x && x <= pos.x + size.x && y >= pos.y && y <= pos.y + size.y {
        if input::is_mouse_button_pressed(MouseButton::Left) {
            field.active = true;
        }
    } else if input::is_mouse_button_pressed(MouseButton::Left) {
        field.active = false;
    }

    draw_text(
        &field.text,
        pos.x + 1.5,
        pos.y + size.y / 1.5,
        21.,
        text_color,
    );

    if field.active {
        let move_cursor = field.insert_idx as f32 * 9.19;
        draw_line(
            pos.x + 2.5 + move_cursor,
            pos.y + 8.5,
            pos.x + 2.5 + move_cursor,
            pos.y + size.y - 8.5,
            2.5,
            Color::new(1., 0.3, 0.4, 0.7),
        );
    } else {
        return &field.text;
    }

    if let Some(pressed) = input::get_char_pressed() {
        if pressed as u8 == 2 {
            if field.insert_idx > 0 {
                field.insert_idx -= 1;
            }
            return &field.text;
        }
        if pressed as u8 == 3 {
            if field.insert_idx < field.text.len() {
                field.insert_idx += 1;
            }
            return &field.text;
        }

        if pressed as u8 == 13 {
            field.active = false;
            return &field.text;
        }
        if pressed as u8 == 127 {
            if field.insert_idx > 0 {
                field.text.remove(field.insert_idx - 1);
                field.insert_idx -= 1;
            }
        } else if field.text.len() <= (size.x / (21. * 0.5)).ceil() as usize
            && (('a'..='z').contains(&pressed)
                || ('0'..='9').contains(&pressed)
                || ".+*/- ".contains(pressed))
        {
            field.text.insert(field.insert_idx, pressed);
            field.insert_idx += 1;
        }
    }

    &field.text
}
