pub trait Parse {
    fn parse(&self, s: &str) -> bool;
}