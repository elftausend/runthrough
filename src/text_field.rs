use std::{collections::HashMap, cell::RefCell};

use macroquad::{prelude::{draw_rectangle, Vec2, LIGHTGRAY, WHITE, MouseButton, draw_text, Color}, input};

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
    draw_rectangle(pos.x, pos.y, size.x, size.y, LIGHTGRAY);
    draw_rectangle(pos.x + 1.5, pos.y + 1.5, size.x - 2.5, size.y - 3., WHITE);

    let (x, y) = input::mouse_position();
    if x >= pos.x && x <= pos.x+size.x && y >= pos.y && y <= pos.y+size.y {
        if input::is_mouse_button_pressed(MouseButton::Left) {
            field.active = true;
        }
    } else if input::is_mouse_button_pressed(MouseButton::Left) {
        field.active = false;
    }

    draw_text(&field.text, pos.x + 1.5, pos.y + size.y / 1.5, 21., text_color);

    if !field.active {
        return &field.text;
    }
    
    if let Some(pressed) = input::get_char_pressed() {
        if pressed as u8 == 13 {
            field.active = false;
            return &field.text;
        }
        if pressed as u8 == 127 {
            field.text.pop();
        } else if field.text.len() <= (size.x / (21. * 0.5)).ceil() as usize {
            field.text.push(pressed);
        }
    }

    &field.text

}