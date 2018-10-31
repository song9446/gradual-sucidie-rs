use quicksilver::{
    Result,
    combinators::result,
    geom::{Shape, Vector, Rectangle},
    graphics::{Background, Background::Img, Background::Col, Color, Font, FontStyle, Image},
    lifecycle::{Asset, Settings, Window, run},
    input::{Key, ButtonState, MouseButton},
};
pub trait Scene {
    type Data;
    fn update(&mut self, window: &mut Window, data: &mut Self::Data) -> Option<Box<Scene<Data=Self::Data>>>;
    fn draw(&mut self, window: &mut Window, data: &mut Self::Data) -> Result<()>;
}

pub struct Game<D> {
    pub current_scene: Box<Scene<Data=D>>,
    pub data: D,
}
impl<D> Game<D> {
    pub fn update(&mut self, window: &mut Window) -> Result<()>{
        if let Some(next_scene) = self.current_scene.update(window, &mut self.data) {
            self.current_scene = next_scene
        };
        Ok(())
    }
    pub fn draw(&mut self, window: &mut Window) -> Result<()>{
        self.current_scene.draw(window, &mut self.data);
        Ok(())
    }
}

