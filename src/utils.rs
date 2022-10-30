use macroquad::rand::{self, RandomRange};

#[inline]
pub fn random<T>(min: T, max: T) -> T
where
    T: PartialOrd + PartialEq + RandomRange,
{
    rand::gen_range(min, max)
}
