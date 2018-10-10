#[macro_use]
extern crate stdweb;
extern crate quicksilver;

use stdweb::web::ArrayBuffer;
use stdweb::InstanceOf;
use stdweb::web::TypedArray;
use stdweb::unstable::TryInto;
use stdweb::Reference;
use stdweb::Value;
use futures::{Future, future};
use quicksilver::{
    Result,
    combinators::result,
    geom::{Shape, Vector, Rectangle},
    graphics::{Background::Img, Background::Col, Color, Font, FontStyle, Image},
    lifecycle::{Asset, Settings, State, Window, run},
};

mod gui;

struct SampleText {
    asset: Asset<Image>,
    xhr: stdweb::web::XmlHttpRequest,
}

impl State for SampleText {
    fn new() -> Result<SampleText> {
        let asset = Asset::new(Font::load("ttf/font.ttf")
            .and_then(|font| {
                let style = FontStyle::new(72.0, Color::BLACK);
                result(font.render("Sample Text", &style))
            }));
        let xhr = stdweb::web::XmlHttpRequest::new();
        js! { @{&xhr}.responseType = "arraybuffer"; }
        xhr.open("GET", "ttf/font.ttf");
        xhr.send();
        Ok(SampleText{asset, xhr})
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        window.draw(&Rectangle::new((100, 100), (32, 32)), Col(Color::BLUE));
        self.asset.execute(|image| {
            window.draw(&image.area().with_center((400, 300)), Img(&image));
            Ok(())
        });
        Ok(())
    }
}

fn main() {
    run::<SampleText>("Font Example", Vector::new(800, 600), Settings::default());
}
