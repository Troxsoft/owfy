#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position<T> {
    pub x: T,
    pub y: T,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Size<T> {
    pub w: T,
    pub h: T,
}
impl<T> Position<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl<T> Size<T> {
    pub fn new(w: T, h: T) -> Self {
        Self { w, h }
    }
}
impl<T> Default for Position<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
        }
    }
}

impl<T> Default for Size<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            w: T::default(),
            h: T::default(),
        }
    }
}
