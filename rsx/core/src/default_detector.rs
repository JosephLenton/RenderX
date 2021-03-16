use ::std::marker::PhantomData;

pub trait NotDefault<V> {
    fn maybe_default() -> Option<V>;
}

pub struct DefaultDetector<T>(PhantomData<T>);
impl<T> NotDefault<T> for DefaultDetector<T> {
    fn maybe_default() -> Option<T> {
        None
    }
}

impl<T: Default> DefaultDetector<T> {
    pub fn maybe_default() -> Option<T> {
        Some(Default::default())
    }
}

#[cfg(test)]
mod example {
    use super::*;

    #[test]
    fn it_returns_default_on_default_items() {
        let n: Option<u32> = DefaultDetector::<u32>::maybe_default();
        assert_eq!(n, Some(u32::default()));
    }

    #[test]
    fn it_returns_none_on_non_default_items() {
        struct NotDefault;

        let n = DefaultDetector::<NotDefault>::maybe_default();
        assert!(n.is_none());
    }
}
