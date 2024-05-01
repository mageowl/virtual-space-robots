use raylib::{
    texture::{Image, Texture2D},
    RaylibHandle, RaylibThread,
};

pub struct Assets {
    pub ship: Texture2D,
}

pub fn load(rl: &mut RaylibHandle, thread: &RaylibThread) -> Assets {
    Assets {
        ship: rl
            .load_texture_from_image(
                &thread,
                &Image::load_image("assets/ship.png").expect("failed to load image"),
            )
            .expect("failed to load texture"),
    }
}
