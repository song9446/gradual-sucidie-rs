use std::path::Path;

struct Asset<T> {
    loaded: T,
}
impl<T> Asset<T> {
    fn new(path: impl AsRef<Path>) -> Result<Asset>{
        Asset<T>{data}
    }
    fn execute() -> Result<Asset>{
    }
}
