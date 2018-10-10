extern crate quicksilver;
use std::collections::HashMap;
use sdt::path::Path;
use quicksilver::{
    Result,
    graphics::{Color, Font, FontStyle, Image},
}
struct<T> AssetManager {
    images: HashMap<String, T>,
}
impl<T> AssetManager<T> {
    fn new() -> AssetManager{
        AssetManager{
            images: HashMap::new(),
        }
    }
    fn add(&mut self, key:String, path:impl AsRef<Path>){
        T::load(path).and_then()
        self.images.insert(key, T::load);
    }
    fn add_font(&mut self, 
}
