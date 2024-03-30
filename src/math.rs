use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
};

/// Represents a signed 8.8 fixed point number
#[derive(Clone, Copy)]
pub struct Fixed(i16);

impl Fixed {
    pub fn from(val: i16) -> Self {
        Self(val)
    }

    pub fn from_float(val: f32) -> Self {
        Self((val * 256.) as i16)
    }
}

impl Default for Fixed {
    fn default() -> Self {
        Self(256)
    }
}

impl Debug for Fixed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&(self.0 as f64 / 256.0), f)
    }
}

impl Display for Fixed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&(self.0 as f64 / 256.0), f)
    }
}

impl Add<Fixed> for Fixed {
    type Output = Fixed;

    fn add(self, rhs: Fixed) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<isize> for Fixed {
    type Output = Fixed;

    fn add(self, rhs: isize) -> Self::Output {
        Self((self.0 as isize + rhs * 256) as i16)
    }
}

impl AddAssign<Fixed> for Fixed {
    fn add_assign(&mut self, rhs: Fixed) {
        self.0 += rhs.0;
    }
}

impl AddAssign<isize> for Fixed {
    fn add_assign(&mut self, rhs: isize) {
        *self = *self + rhs;
    }
}

impl Sub<Fixed> for Fixed {
    type Output = Fixed;

    fn sub(self, rhs: Fixed) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Sub<isize> for Fixed {
    type Output = Fixed;

    fn sub(self, rhs: isize) -> Self::Output {
        Self((self.0 as isize - rhs * 256) as i16)
    }
}

impl SubAssign<Fixed> for Fixed {
    fn sub_assign(&mut self, rhs: Fixed) {
        self.0 -= rhs.0;
    }
}

impl SubAssign<isize> for Fixed {
    fn sub_assign(&mut self, rhs: isize) {
        *self = *self - rhs;
    }
}

impl Mul<Fixed> for Fixed {
    type Output = Fixed;

    fn mul(self, rhs: Fixed) -> Self::Output {
        Self(((self.0 as isize * rhs.0 as isize) >> 8) as i16)
    }
}

impl Mul<isize> for Fixed {
    type Output = Fixed;

    fn mul(self, rhs: isize) -> Self::Output {
        Self((self.0 as isize * rhs) as i16)
    }
}

impl Mul<Fixed> for isize {
    type Output = isize;

    fn mul(self, rhs: Fixed) -> Self::Output {
        self * rhs.0 as isize / 256
    }
}

impl MulAssign<Fixed> for Fixed {
    fn mul_assign(&mut self, rhs: Fixed) {
        *self = *self * rhs;
    }
}

impl MulAssign<isize> for Fixed {
    fn mul_assign(&mut self, rhs: isize) {
        *self = *self * rhs;
    }
}

impl MulAssign<Fixed> for isize {
    fn mul_assign(&mut self, rhs: Fixed) {
        *self = *self * rhs;
    }
}

impl Div<Fixed> for Fixed {
    type Output = Fixed;

    fn div(self, rhs: Fixed) -> Self::Output {
        Self((((self.0 as isize) << 8) / rhs.0 as isize) as i16)
    }
}

impl Div<isize> for Fixed {
    type Output = Fixed;

    fn div(self, rhs: isize) -> Self::Output {
        Self((self.0 as isize / rhs) as i16)
    }
}

impl Div<Fixed> for isize {
    type Output = isize;

    fn div(self, rhs: Fixed) -> Self::Output {
        self * 256 / rhs.0 as isize
    }
}

impl DivAssign<Fixed> for Fixed {
    fn div_assign(&mut self, rhs: Fixed) {
        *self = *self / rhs;
    }
}

impl DivAssign<isize> for Fixed {
    fn div_assign(&mut self, rhs: isize) {
        *self = *self * rhs;
    }
}

impl DivAssign<Fixed> for isize {
    fn div_assign(&mut self, rhs: Fixed) {
        *self = *self * rhs;
    }
}

impl Rem<Fixed> for Fixed {
    type Output = Fixed;

    fn rem(self, rhs: Fixed) -> Self::Output {
        Self(self.0 % rhs.0)
    }
}

impl RemAssign<Fixed> for Fixed {
    fn rem_assign(&mut self, rhs: Fixed) {
        *self = *self % rhs;
    }
}
