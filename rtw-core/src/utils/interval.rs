use crate::{Result, RtwError};

pub type Interval = std::ops::Range<f64>;

pub trait Pad<T> {
    fn pad(&self, delta: T) -> Self;
}

impl<T> Pad<T> for std::ops::Range<T>
where
    T: Copy + std::ops::Sub<Output = T> + std::ops::Add<Output = T>,
{
    fn pad(&self, delta: T) -> Self {
        (self.start - delta)..(self.end + delta)
    }
}

pub trait IntervalExtend<T> {
    fn from_ranges(a: &Self, b: &Self) -> Self;
}

impl<T> IntervalExtend<T> for std::ops::Range<T>
where
    T: Copy + PartialOrd,
{
    fn from_ranges(a: &Self, b: &Self) -> Self {
        (if a.start < b.start { a.start } else { b.start })..(if a.end > b.end {
            a.end
        } else {
            b.end
        })
    }
}

pub trait New<T>
where
    T: Sized,
    Self: Sized,
{
    fn new(min: T, max: T) -> Result<Self>;
}

impl<T> New<T> for std::ops::Range<T>
where
    T: Copy + PartialOrd,
{
    fn new(a: T, b: T) -> Result<Self> {
        if a < b {
            Ok(a..b)
        } else if a > b {
            Ok(b..a)
        } else {
            Err(RtwError::InvalidInterval)
        }
    }
}
