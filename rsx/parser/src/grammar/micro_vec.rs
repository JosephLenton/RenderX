use ::std::mem::replace;

/// MicroVec is like SmallVec. Only smaller.
/// So small, it only caches one item before into a Vec.
pub enum MicroVec<T> {
    None,
    Item(T),
    Vec(Vec<T>),
}

impl<T> MicroVec<T> {
    pub fn new() -> Self {
        Self::None
    }

    pub fn push(mut self, t: T) -> Self {
        match self {
            Self::None => Self::Item(t),
            Self::Item(t0) => Self::Vec(vec![t0, t]),
            Self::Vec(mut ts) => {
                ts.push(t);
                Self::Vec(ts)
            }
        }
    }
}
