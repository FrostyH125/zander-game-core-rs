use raylib::{
    color::Color,
    drawing::RaylibDrawHandle,
    math::{Rectangle, Vector2},
    texture::Texture2D,
};

use crate::raylib::{animation_data::SpriteAnimationData, sprite::Sprite};

pub struct SpriteAnimationInstance {
    pub sprite_animation: &'static SpriteAnimationData,
    pub current_frame_time: f32,
    pub current_frame_index: u8,
    pub finished_playing: bool,
    pub is_playing: bool
}

impl SpriteAnimationInstance {
    pub fn new(sprite_animation: &'static SpriteAnimationData) -> Self {
        SpriteAnimationInstance {
            sprite_animation,
            current_frame_time: 0.0,
            current_frame_index: 0,
            finished_playing: false,
            is_playing: false
        }
    }

    pub fn update(&mut self, dt: f32) {
        let frame_count = self.sprite_animation.frames.len();

        if self.finished_playing {
            return;
        }

        if frame_count == 0 {
            self.finished_playing = true;
            self.is_playing = false;
            return;
        }

        self.current_frame_time += dt;

        self.is_playing = true;

        while self.current_frame_time >= self.sprite_animation.frame_duration {
            self.current_frame_time -= self.sprite_animation.frame_duration;
            self.current_frame_index += 1;

            if (self.current_frame_index as usize) < frame_count {
                return;
            }

            // if it makes it this far then it needs to
            // make a decision based on whether it should
            // loop or not
            if self.sprite_animation.should_loop {
                self.current_frame_index = 0;
            } else {
                self.current_frame_index -= 1;
                self.finished_playing = true;
                self.is_playing = false;
                break;
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, pos: Vector2, texture: &Texture2D) {
        self.sprite_animation.frames[self.current_frame_index as usize].draw(d, pos, texture);
    }

    pub fn draw_flp(&self, d: &mut RaylibDrawHandle, pos: Vector2, texture: &Texture2D, flp_h: bool, flp_v: bool) {
        self.sprite_animation.frames[self.current_frame_index as usize].draw_flp(
            d,
            pos,
            texture,
            flp_h,
            flp_v,
            Color::WHITE,
        );
    }

    pub fn draw_flp_color(
        &self,
        d: &mut RaylibDrawHandle,
        pos: Vector2,
        texture: &Texture2D,
        flp_h: bool,
        flp_v: bool,
        tint: Color,
    ) {
        self.sprite_animation.frames[self.current_frame_index as usize].draw_flp(d, pos, texture, flp_h, flp_v, tint);
    }

    pub fn draw_pro(
        &self,
        d: &mut RaylibDrawHandle,
        dest_rect: Rectangle,
        origin: Vector2,
        rotation: f32,
        texture: &Texture2D,
        color: Color,
    ) {
        self.sprite_animation.frames[self.current_frame_index as usize]
            .draw_pro(d, dest_rect, origin, rotation, texture, color);
    }

    pub fn reset(&mut self) {
        self.current_frame_index = 0;
        self.current_frame_time = 0.0;
        self.finished_playing = false;
        self.is_playing = false;
    }

    pub fn current_sprite(&self) -> Sprite {
        return self.sprite_animation.frames[self.current_frame_index as usize];
    }
}
