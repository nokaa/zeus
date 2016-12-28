pub trait Backend {
    fn open() -> Self;
    fn read(&self);
    fn write(&self);
}
