use raylib::prelude::*;
#[derive(Copy, Clone)]
pub struct Sprite {
    pub src_rect: Rectangle,
}

pub const EMPTY: Sprite = Sprite::new(0, 0, 0, 0);
const INSET_FOR_SPRITE_BLEED_FIX: f32 = 1.0 / 100.0;

impl Sprite {
    pub const fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Sprite {
            src_rect: Rectangle {
                x: x as f32 + INSET_FOR_SPRITE_BLEED_FIX,
                y: y as f32 + INSET_FOR_SPRITE_BLEED_FIX,
                width: width as f32 - (INSET_FOR_SPRITE_BLEED_FIX),
                height: height as f32 - (INSET_FOR_SPRITE_BLEED_FIX),
            },
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, pos: Vector2, texture: &Texture2D) {
        d.draw_texture_pro(
            texture,
            self.src_rect,
            Rectangle::new(pos.x.floor(), pos.y.floor(), self.src_rect.width.ceil(), self.src_rect.height.ceil()),
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );
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
        d.draw_texture_pro(texture, self.src_rect, dest_rect, origin, rotation, color);
    }

    pub fn draw_col(&self, d: &mut RaylibDrawHandle, pos: Vector2, texture: &Texture2D, tint: Color) {
        d.draw_texture_pro(
            texture,
            self.src_rect,
            Rectangle::new(pos.x.floor(), pos.y.floor(), self.src_rect.width.ceil(), self.src_rect.height.ceil()),
            Vector2::zero(),
            0.0,
            tint,
        );
    }

    pub fn draw_flp(
        &self,
        d: &mut RaylibDrawHandle,
        pos: Vector2,
        texture: &Texture2D,
        flp_h: bool,
        flp_v: bool,
        tint: Color,
    ) {
        let spr_width = self.src_rect.width.ceil();
        let spr_height = self.src_rect.height.ceil();

        let flp_h_mult = match flp_h {
            true => -1.0,
            false => 1.0,
        };
        let flp_v_mult = match flp_v {
            true => -1.0,
            false => 1.0,
        };

        let new_sprite = Sprite::new(
            self.src_rect.x as i32,
            self.src_rect.y as i32,
            (spr_width * flp_h_mult) as i32,
            (spr_height * flp_v_mult) as i32,
        );
        let dest_rect = Rectangle::new(pos.x, pos.y, spr_width, spr_height);
        d.draw_texture_pro(texture, new_sprite.src_rect, dest_rect, Vector2::zero(), 0.0, tint);
    }

    pub const fn create_sequence_of_sprites<const N: usize>(
        start_x: i32,
        start_y: i32,
        width: i32,
        height: i32,
    ) -> [Sprite; N] {

        let mut sprites = [EMPTY; N];

        let mut i = 0;
        while i < N {
            let sprite = Sprite::new(start_x + (i as i32 * width), start_y, width, height);
            sprites[i] = sprite;
            i += 1;
        }

        return sprites;
    }
}
