use opengl_graphics::Texture;

pub struct Sprite {
    imagePath: String,
    sprite: G2dTexture,
}

impl Sprite {
    pub fn new(imageName: String) -> Self {
        sprite {
            imagePath: imageName,
            sprite: None,
        }
    }

    pub fn load(&self) {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();

        let img = assets.join(&self.imagePath);
        self.sprite = Texture::from_path(
            &mut window.create_texture_context(),
            &rust_logo,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();
    }

    pub fn render(&self) -> Texture {
        
    }
}