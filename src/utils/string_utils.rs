use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    math::Vector2,
    text::{Font, RaylibFont},
};

use crate::system::timer::Timer;

pub fn wrap_string(string: &str, max_length_in_pixels: f32, font: &Font, font_size: f32, spacing: f32) -> String {
    let words: Vec<&str> = string.split(" ").collect();
    let mut final_string = String::new();
    let mut current_line = String::new();

    for word in words {
        let original_len = current_line.len();

        if !current_line.is_empty() {
            current_line.push(' ');
        }

        current_line.push_str(word);

        let line_size = font.measure_text(&current_line, font_size, spacing);

        if line_size.x >= max_length_in_pixels {
            current_line.truncate(original_len);

            if !current_line.is_empty() {
                final_string.push_str(&current_line);
                final_string.push('\n');
            }

            current_line.clear();
            current_line.push_str(word);
        }

        //only reasonably should happen if the word ends with \n
        if current_line.contains('\n') {
            final_string.push_str(&current_line);
            current_line.clear();
        }
    }

    if !current_line.is_empty() {
        final_string.push_str(&current_line);
    }

    return final_string;
}

pub fn draw_string_centered_on_pos(
    d: &mut RaylibDrawHandle,
    pos: Vector2,
    string: &str,
    font: &Font,
    font_size: f32,
    spacing: f32,
    color: Color,
) {
    let size_of_string = font.measure_text(string, font_size, spacing);

    let draw_pos = pos - (size_of_string / 2.0);

    d.draw_text_ex(font, string, draw_pos, font_size, spacing, color);
}

pub struct StringSinWave {
    pub string: String,
    pub pos: Vector2,
    pub max_wave_height: f32,
    pub wave_speed: f32,
    pub sin_offset_per_character: f32,
    pub color: Color,
    pub font_size: f32,
    pub spacing: f32,
    pub timer: Timer,
}

impl StringSinWave {
    fn update(&mut self, dt: f32) {
        self.timer.track(dt);
    }

    fn draw(&self, d: &mut RaylibDrawHandle, font: &Font) {
        // necessary buffer for a low allocation char -> &str conversion
        let mut ch_buffer = [0u8; 4];

        let mut progress = self.timer.progress();

        if progress >= 1.0 {
            progress = 1.0;
        }

        let amplitude_as_per_progress = 1.0 - progress;
        let time_elapsed = self.timer.current_time;

        let ch_size =
            font.measure_text(&self.string, self.font_size, self.spacing) / Vector2::new(self.string.len() as f32, 1.0);

        for (i, ch) in self.string.chars().enumerate() {
            // get the x position first
            let x = self.pos.x + ch_size.x * i as f32;

            // calculate what to use for the sin() function based on the speed of the wave, 
            // the current time, and the current char. time * speed will give a base offset, adding the 
            // offset based on char will give a smooth wave over that
            let sin_offset = time_elapsed * self.wave_speed + i as f32 * self.sin_offset_per_character;

            // get the final y offset by taking the sin of the sin offset, and multiplying it by 
            // what the wave height is set at (changed by how far the timer has progressed, moving
            // toward the center over time)
            let y_offset = sin_offset.sin() * (self.max_wave_height * amplitude_as_per_progress);
            let y = self.pos.y + y_offset;

            // necessary as d.draw_text_ex cannot take a char
            // needs to manually encode the character into a 4 byte buffer
            // and get a reference to it as a &str
            let s = ch.encode_utf8(&mut ch_buffer);

            d.draw_text_ex(font, s, Vector2::new(x, y), self.font_size, self.spacing, self.color);
        }
    }
}
