use crate::component::Component;

pub struct Game {
    components: Vec<Component>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            components: vec![]
        }
    }
    pub fn load_content(&self) {}
    pub fn tick(&self) {}
    pub fn render(&self) {
        for component in self.components.iter() {
            component.render();
        }
    }
}
