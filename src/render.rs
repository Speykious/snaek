use std::cell::RefCell;
use std::marker::PhantomData;

use self::bitmap::Bitmap;
use self::pixel::alphacomp::{self, AlphaCompFn};
use self::pixel::Pixel;
use self::pos::Pos;
use self::size::Size;

pub mod bitmap;
pub mod pixel;
pub mod pos;
pub mod size;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sprite {
    pub id: SpritesheetId,
    /// Describes where the sprite is located in the spritesheet.
    pub rect: Rect,
}

impl Sprite {
    #[inline]
    pub fn new(id: SpritesheetId, rect: Rect) -> Self {
        Self { id, rect }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum NineSlicePart {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NineSlicingSprite {
    pub sprite: Sprite,

    /// position of the left vertical bar (from top left corner)
    pub vl: u16,
    /// position of the right vertical bar (from top left corner)
    pub vr: u16,
    /// position of the top horizontal bar (from top left corner)
    pub ht: u16,
    /// position of the bottom horizontal bar (from top left corner)
    pub hb: u16,
}

impl NineSlicingSprite {
    #[inline]
    pub fn new(id: SpritesheetId, rect: Rect, vl: u16, vr: u16, ht: u16, hb: u16) -> Self {
        Self {
            sprite: Sprite::new(id, rect),
            vl,
            vr,
            ht,
            hb,
        }
    }

    pub fn slice(&self, part: NineSlicePart) -> Sprite {
        match part {
            NineSlicePart::TopLeft => {
                let x = self.sprite.rect.x;
                let y = self.sprite.rect.y;
                let w = self.vl;
                let h = self.ht;
                Sprite::new(self.sprite.id, Rect::from_xywh(x, y, w, h))
            },
            NineSlicePart::TopCenter => {
                let x = self.sprite.rect.x + self.vl as i16;
                let y = self.sprite.rect.y;
                let w = self.vr - self.vl;
                let h = self.ht;
                Sprite::new(self.sprite.id, Rect::from_xywh(x, y, w, h))
            },
            NineSlicePart::TopRight => {
                let x = self.sprite.rect.x + self.vr as i16;
                let y = self.sprite.rect.y;
                let w = self.sprite.rect.w - self.vr;
                let h = self.ht;
                Sprite::new(self.sprite.id, Rect::from_xywh(x, y, w, h))
            },
            NineSlicePart::CenterLeft => {
                let x = self.sprite.rect.x;
                let y = self.sprite.rect.y + self.ht as i16;
                let w = self.vl;
                let h = self.hb - self.ht;
                Sprite::new(self.sprite.id, Rect::from_xywh(x, y, w, h))
            },
            NineSlicePart::Center => {
                let x = self.sprite.rect.x + self.vl as i16;
                let y = self.sprite.rect.y + self.ht as i16;
                let w = self.vr - self.vl;
                let h = self.hb - self.ht;
                Sprite::new(self.sprite.id, Rect::from_xywh(x, y, w, h))
            },
            NineSlicePart::CenterRight => {
                let x = self.sprite.rect.x + self.vr as i16;
                let y = self.sprite.rect.y + self.ht as i16;
                let w = self.sprite.rect.w - self.vr;
                let h = self.hb - self.ht;
                Sprite::new(self.sprite.id, Rect::from_xywh(x, y, w, h))
            },
            NineSlicePart::BottomLeft => {
                let x = self.sprite.rect.x;
                let y = self.sprite.rect.y + self.hb as i16;
                let w = self.vl;
                let h = self.sprite.rect.h - self.hb;
                Sprite::new(self.sprite.id, Rect::from_xywh(x, y, w, h))
            },
            NineSlicePart::BottomCenter => {
                let x = self.sprite.rect.x + self.vl as i16;
                let y = self.sprite.rect.y + self.hb as i16;
                let w = self.vr - self.vl;
                let h = self.sprite.rect.h - self.hb;
                Sprite::new(self.sprite.id, Rect::from_xywh(x, y, w, h))
            },
            NineSlicePart::BottomRight => {
                let x = self.sprite.rect.x + self.vr as i16;
                let y = self.sprite.rect.y + self.hb as i16;
                let w = self.sprite.rect.w - self.vr;
                let h = self.sprite.rect.h - self.hb;
                Sprite::new(self.sprite.id, Rect::from_xywh(x, y, w, h))
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpritesheetId(usize);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DrawCommand {
    Clear,
    Fill {
        rect: Rect,
        color: Pixel,
        acf: AlphaCompFn,
    },
    Sprite {
        pos: Pos,
        sprite: Sprite,
        acf: AlphaCompFn,
    },
    BeginComposite,
    EndComposite(AlphaCompFn),
}

struct FramebufferStack {
    size: Size,
    fbs: Vec<Bitmap>,
}

impl FramebufferStack {
    pub fn new(framebuffer: Bitmap) -> Self {
        Self {
            size: framebuffer.size(),
            fbs: vec![framebuffer],
        }
    }

    pub fn blit_fb_down(&mut self, zindex: usize, acf: AlphaCompFn) {
        if zindex >= self.fbs.len() || zindex == 0 {
            return;
        }

        // this is probably a hack but it's a good way to get two mutable elements from a vec
        let [fba, fbb] = &mut self.fbs[(zindex - 1)..=zindex] else {
            return;
        };

        fba.copy_bitmap(fbb, acf);
    }

    fn push_fbs(&mut self, zindex: usize) {
        while zindex >= self.fbs.len() {
            self.fbs.push(Bitmap::new(self.size));
        }
    }

    fn fb_mut(&mut self, zindex: usize) -> &mut Bitmap {
        self.push_fbs(zindex);
        &mut self.fbs[zindex]
    }

    fn fb(&mut self, zindex: usize) -> &Bitmap {
        self.push_fbs(zindex);
        &self.fbs[zindex]
    }
}

pub struct Renderer {
    fb_stack: FramebufferStack,
    spritesheets: Vec<Bitmap>,
}

impl Renderer {
    pub fn new(framebuffer: Bitmap) -> Self {
        Self {
            fb_stack: FramebufferStack::new(framebuffer),
            spritesheets: Vec::new(),
        }
    }

    pub fn register_spritesheet(&mut self, sheet: Bitmap) -> SpritesheetId {
        let id = SpritesheetId(self.spritesheets.len());
        self.spritesheets.push(sheet);
        id
    }

    pub fn first_framebuffer(&mut self) -> &Bitmap {
        self.fb_stack.fb(0)
    }

    pub fn draw(&mut self, commands: &[DrawCommand]) {
        draw(commands, &mut self.fb_stack, &self.spritesheets);
    }

    pub fn size(&self) -> Size {
        self.fb_stack.size
    }

    pub fn rect(&self) -> Rect {
        Rect::from_pos_size(Pos::ZERO, self.size())
    }
}

fn draw(commands: &[DrawCommand], fb_stack: &mut FramebufferStack, spritesheets: &[Bitmap]) {
    let mut fb_id = 0;
    for command in commands {
        match *command {
            DrawCommand::Clear => (fb_stack.fb_mut(fb_id)).fill(Pixel::ZERO, alphacomp::dst),
            DrawCommand::Fill { rect, color, acf } => {
                (fb_stack.fb_mut(fb_id)).fill_area(color, rect, acf)
            }
            DrawCommand::Sprite { pos, sprite, acf } => {
                if let Some(bitmap) = spritesheets.get(sprite.id.0) {
                    (fb_stack.fb_mut(fb_id)).copy_bitmap_area(
                        bitmap,
                        pos,
                        sprite.rect.pos(),
                        sprite.rect.size(),
                        acf,
                    );
                };
            }
            DrawCommand::BeginComposite => {
                fb_id += 1;
            }
            DrawCommand::EndComposite(acf) => {
                fb_stack.blit_fb_down(fb_id, acf);
                fb_id -= 1;
            }
        }
    }
}
