pub trait Window {
    fn title(&self) -> &str;

    fn close(&self);
}
