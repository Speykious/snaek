pub mod bitmap;

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
