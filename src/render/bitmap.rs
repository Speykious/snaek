use super::pixel::alphacomp::AlphaCompFn;
use super::pixel::Pixel;
use super::{pos, Pos, Rect, Size};

/// RGBA bitmap.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Bitmap {
    data: Vec<u32>,
    size: Size,
}

impl Bitmap {
    #[inline]
    pub fn new(size: Size) -> Self {
        Self {
            data: vec![0; size.w as usize * size.h as usize],
            size,
        }
    }

    /// Resizes the bitmap with `size` as the size.
    /// This completely resets the data.
    pub fn resize(&mut self, size: Size) {
        self.data = vec![0; size.w as usize * size.h as usize];
        self.size = size;
    }

    #[inline]
    pub fn pixels(&self) -> &[u32] {
        &self.data
    }

    fn line_indices(&self, pos: Pos, width: u16) -> (usize, usize) {
        let start_x = pos.x as usize;
        let end_x = (self.size.w as usize).min(width as usize + start_x);
        let width = end_x - start_x;

        let start = self.index(pos);
        (start, start + width)
    }

    /// Gives a full horizontal line of pixels
    pub fn line(&self, pos: Pos, width: u16) -> &[u32] {
        let (start, end) = self.line_indices(pos, width);
        &self.data[start..end]
    }

    /// Gives a full horizontal line of pixels (mutable)
    pub fn line_mut(&mut self, pos: Pos, width: u16) -> &mut [u32] {
        let (start, end) = self.line_indices(pos, width);
        &mut self.data[start..end]
    }

    /// A function that looks like this:
    ///
    /// ```ignore
    /// #[inline]
    /// pub fn size(&self) -> Size {
    ///     self.size
    /// }
    /// ```
    #[inline]
    pub fn size(&self) -> Size {
        self.size
    }

    pub fn copy_bitmap(&mut self, other: &Bitmap, acf: AlphaCompFn) {
        for (px, other_px) in self.data.iter_mut().zip(other.data.iter()) {
            *px = (acf)(Pixel::from_hex(*other_px), Pixel::from_hex(*px)).to_u32();
        }
    }

    pub fn copy_bitmap_area(
        &mut self,
        other: &Bitmap,
        this_pos: Pos,
        other_pos: Pos,
        size: Size,
        acf: AlphaCompFn,
    ) {
        let size = {
            let cropped_rect = self.crop_rect(Rect::from_pos_size(this_pos, size));
            let cropped_rect = self.crop_rect(Rect::from_pos_size(other_pos, cropped_rect.size()));
            cropped_rect.size()
        };

        for y in 0..size.h as i16 {
            let this_line = self.line_mut(pos(this_pos.x, this_pos.y + y), size.w);
            let other_line = other.line(pos(other_pos.x, other_pos.y + y), size.w);
            for (this_px, other_px) in this_line.iter_mut().zip(other_line.iter()) {
                *this_px = (acf)(Pixel::from_hex(*other_px), Pixel::from_hex(*this_px)).to_u32();
            }
        }
    }

    pub fn fill(&mut self, pixel: Pixel, acf: AlphaCompFn) {
        for px in &mut self.data {
            *px = (acf)(pixel, Pixel::from_hex(*px)).to_u32();
        }
    }

    pub fn fill_area(&mut self, pixel: Pixel, rect: Rect, acf: AlphaCompFn) {
        let rect = self.crop_rect(rect);

        for y in 0..rect.h as i16 {
            for px in self.line_mut(pos(rect.x, rect.y + y), rect.w) {
                *px = (acf)(pixel, Pixel::from_hex(*px)).to_u32();
            }
        }
    }

    fn crop_rect(&self, mut rect: Rect) -> Rect {
        if rect.x < 0 {
            rect.w = rect.w.saturating_add_signed(rect.x);
            rect.x = 0;
        }

        if rect.y < 0 {
            rect.h = rect.h.saturating_add_signed(rect.y);
            rect.y = 0;
        }

        let difference = (rect.x as i32 + rect.w as i32 - self.size.w as i32) as i16;
        if difference > 0 {
            rect.w = rect.w.saturating_add_signed(-difference);
        }

        let difference = (rect.y as i32 + rect.h as i32 - self.size.h as i32) as i16;
        if difference > 0 {
            rect.h = rect.h.saturating_add_signed(-difference);
        }

        rect
    }

    /// Converts a position to the index of a pixel on the bitmap.
    fn index(&self, pos: Pos) -> usize {
        debug_assert!(
            pos.x >= 0 && pos.y >= 0,
            "Position has a negative coordinate"
        );
        debug_assert!(
            (pos.x as i32) < (self.size.w as i32),
            "Position exceeds bitmap width"
        );
        debug_assert!(
            (pos.y as i32) < (self.size.h as i32),
            "Position exceeds bitmap height"
        );

        pos.y as usize * self.size.w as usize + pos.x as usize
    }
}
