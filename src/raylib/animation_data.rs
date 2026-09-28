use crate::raylib::sprite::Sprite;

pub struct SpriteAnimationData {
    pub frames: &'static [Sprite],
    pub frame_duration: f32,
    pub should_loop: bool,
}
