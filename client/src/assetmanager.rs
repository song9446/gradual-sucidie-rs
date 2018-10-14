use {
    std::{
        path::Path,
        collections::HashMap,
    }
}
enum Asset<T>{
    Loading(T),
    Loaded(T),
}
struct Assetmanager{
    cache:Vec<Asset<Font>>, 
}
impl Assetmanager {
    fn load<T>(&mut self, future:T) {
        self.cache.insert(P, future);
    }
}
