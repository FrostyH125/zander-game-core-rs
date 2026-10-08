use crate::raylib::sprite::Sprite;

pub struct SpriteAnimationData {
    pub frames: &'static [Sprite],
    pub frame_duration: f32,
    pub should_loop: bool,
}

impl SpriteAnimationData {
    pub fn total_duration(&self) -> f32 {
        return self.frame_duration * self.frames.len() as f32;
    }
}
