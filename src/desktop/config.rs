#[derive(Debug, Clone)]
pub struct Config<'a> {
    title: &'a str,
    width: u32,
    height: u32,
}

impl<'a> Config<'a> {
    pub fn new(title: &str, width: u32, height: u32) -> Config {
        Config {
            title,
            width,
            height,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn title(&self) -> &str {
        self.title
    }
}
