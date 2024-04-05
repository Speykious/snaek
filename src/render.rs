use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub mod bitmap;
pub mod pixel;

/// Position of something on the bitmap, in pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C, align(4))]
pub struct Pos {
    pub x: i16,
    pub y: i16,
}

#[inline]
pub fn pos(x: i16, y: i16) -> Pos {
    Pos { x, y }
}

impl Add for Pos {
    type Output = Pos;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }
}

impl SubAssign for Pos {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul<i16> for Pos {
    type Output = Pos;

    fn mul(mut self, rhs: i16) -> Self::Output {
        self.x *= rhs;
        self.y *= rhs;
        self
    }
}

impl Mul<f32> for Pos {
    type Output = Pos;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self.x = (self.x as f32 * rhs) as i16;
        self.y = (self.y as f32 * rhs) as i16;
        self
    }
}

impl MulAssign<i16> for Pos {
    fn mul_assign(&mut self, rhs: i16) {
        *self = *self * rhs;
    }
}

impl Div<i16> for Pos {
    type Output = Pos;

    fn div(mut self, rhs: i16) -> Self::Output {
        self.x /= rhs;
        self.y /= rhs;
        self
    }
}

impl DivAssign<i16> for Pos {
    fn div_assign(&mut self, rhs: i16) {
        *self = *self / rhs;
    }
}

/// Size of a rectangle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C, align(4))]
pub struct Size {
    pub w: u16,
    pub h: u16,
}

#[inline]
pub fn size(w: u16, h: u16) -> Size {
    Size { w, h }
}

impl Add for Size {
    type Output = Size;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.w += rhs.w;
        self.h += rhs.h;
        self
    }
}

impl AddAssign for Size {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Size {
    type Output = Size;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.w -= rhs.w;
        self.h -= rhs.h;
        self
    }
}

impl SubAssign for Size {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul<u16> for Size {
    type Output = Size;

    fn mul(mut self, rhs: u16) -> Self::Output {
        self.w *= rhs;
        self.h *= rhs;
        self
    }
}

impl Mul<f32> for Size {
    type Output = Size;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self.w = (self.w as f32 * rhs) as u16;
        self.h = (self.h as f32 * rhs) as u16;
        self
    }
}

impl MulAssign<u16> for Size {
    fn mul_assign(&mut self, rhs: u16) {
        *self = *self * rhs;
    }
}

impl Div<u16> for Size {
    type Output = Size;

    fn div(mut self, rhs: u16) -> Self::Output {
        self.w /= rhs;
        self.h /= rhs;
        self
    }
}

impl DivAssign<u16> for Size {
    fn div_assign(&mut self, rhs: u16) {
        *self = *self / rhs;
    }
}

/// A rectangle with a position and size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C, align(8))]
pub struct Rect {
    pub x: i16,
    pub y: i16,
    pub w: u16,
    pub h: u16,
}

impl Rect {
    #[inline]
    pub fn from_xywh(x: i16, y: i16, w: u16, h: u16) -> Self {
        Self { x, y, w, h }
    }

    #[inline]
    pub fn from_pos_size(pos: Pos, size: Size) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            w: size.w,
            h: size.h,
        }
    }

    #[inline]
    pub fn pos(&self) -> Pos {
        Pos {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    pub fn size(&self) -> Size {
        Size {
            w: self.w,
            h: self.h,
        }
    }
}
