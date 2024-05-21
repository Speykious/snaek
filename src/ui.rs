use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::num::NonZeroUsize;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, DerefMut};

use crate::math::pos::Pos;
use crate::math::rect::Rect;
use crate::math::size::Size;
use crate::render::color::alphacomp::AlphaCompFn;
use crate::render::color::{alphacomp, Color};
use crate::render::sprite::{NineSlicingSprite, Sprite};
use crate::render::{DrawCommand, Rotate, SpritesheetId, Text};

pub mod components;
pub mod layout;

/// ID of a widget.
///
/// The root of all widgets, which is the window itself, has an ID of 0. So all top-level widgets have a `parent_id` of 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// Uses NonZeroUsize to enable null pointer optimization with Option<...>. All accesses are offset by one.
pub struct WidgetId(NonZeroUsize);

/// A key that uniquely identifies a widget.
///
/// It contains
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(C)]
pub struct WidgetKey(u64);

pub struct WidgetKeyHasher<H: Hasher = DefaultHasher>(H);

impl<H: Hasher> WidgetKeyHasher<H> {
	#[doc(hidden)]
	pub fn new(h: H) -> Self {
		Self(h)
	}

	#[doc(hidden)]
	pub fn hash_loc(mut self, module: &'static str, line: u32, col: u32) -> Self {
		module.hash(&mut self.0);
		line.hash(&mut self.0);
		col.hash(&mut self.0);
		self
	}

	pub fn hash_n(mut self, n: u64) -> Self {
		n.hash(&mut self.0);
		self
	}

	pub fn hash_key(mut self, key: WidgetKey) -> Self {
		key.hash(&mut self.0);
		self
	}

	pub fn finish(self) -> WidgetKey {
		WidgetKey(self.0.finish())
	}
}

#[macro_export]
macro_rules! wk {
	( $( [ $($key:ident),* ] )? $($n:ident),* ) => {
		$crate::ui::WidgetKeyHasher::new(::std::hash::DefaultHasher::default())
			.hash_loc(module_path!(), line!(), column!())
			$($(.hash_key($key))*)?
			$(.hash_n($n))*
			.finish()
	};
}

#[derive(Debug, Clone)]
pub struct Widget {
	// tree links
	// It's side-stepping-the-borrow-checker time!
	// note: `next` and `prev` get reused as an intrusive free list when the widget is "freed".
	parent: Option<WidgetId>,
	prev: Option<WidgetId>,
	next: Option<WidgetId>,
	first_child: Option<WidgetId>,
	last_child: Option<WidgetId>,
	children_count: usize,

	// any userland properties
	props: WidgetProps,

	// persistent state
	/// If a widget hasn't been touched in the current frame,
	/// we "free" it (using a free list)
	last_frame_touched: u64,
	freed: bool,

	hovered: bool,
	pressed: bool,
	clicked: bool,

	// Layout state calculated each frame
	solved_rect: Rect,
	solved_min_size: Size,
}

#[derive(Debug, Clone, Copy)]
pub struct WidgetReaction {
	id: WidgetId,
	hovered: bool,
	pressed: bool,
	clicked: bool,
}

impl WidgetReaction {
	#[inline]
	pub const fn id(&self) -> WidgetId {
		self.id
	}

	#[inline]
	pub const fn hovered(&self) -> bool {
		self.hovered
	}

	#[inline]
	pub const fn pressed(&self) -> bool {
		self.pressed
	}

	#[inline]
	pub const fn clicked(&self) -> bool {
		self.clicked
	}
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Anchor {
	pub x: f32,
	pub y: f32,
}

#[rustfmt::skip]
#[allow(unused)]
impl Anchor {
	pub const TOP_LEFT:      Self = Self { x: 0.0, y: 0.0 };
	pub const TOP_CENTER:    Self = Self { x: 0.5, y: 0.0 };
	pub const TOP_RIGHT:     Self = Self { x: 1.0, y: 0.0 };
	pub const CENTER_LEFT:   Self = Self { x: 0.0, y: 0.5 };
	pub const CENTER:        Self = Self { x: 0.5, y: 0.5 };
	pub const CENTER_RIGHT:  Self = Self { x: 1.0, y: 0.5 };
	pub const BOTTOM_LEFT:   Self = Self { x: 0.0, y: 1.0 };
	pub const BOTTOM_CENTER: Self = Self { x: 0.5, y: 1.0 };
	pub const BOTTOM_RIGHT:  Self = Self { x: 1.0, y: 1.0 };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(C, u16, align(4))]
pub enum WidgetDim {
	Fixed(u16),
	Hug,
	#[default]
	Fill,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(C, align(8))]
pub struct WidgetSize {
	pub w: WidgetDim,
	pub h: WidgetDim,
}

impl WidgetSize {
	pub const fn new(w: WidgetDim, h: WidgetDim) -> Self {
		Self { w, h }
	}

	pub const fn fill() -> Self {
		Self {
			w: WidgetDim::Fill,
			h: WidgetDim::Fill,
		}
	}

	pub const fn fixed(w: u16, h: u16) -> Self {
		Self {
			w: WidgetDim::Fixed(w),
			h: WidgetDim::Fixed(h),
		}
	}

	pub const fn hug() -> Self {
		Self {
			w: WidgetDim::Hug,
			h: WidgetDim::Hug,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(C, align(8))]
pub struct WidgetPadding {
	pub t: i16,
	pub r: i16,
	pub b: i16,
	pub l: i16,
}

impl WidgetPadding {
	/// top, right, bottom, left
	#[inline]
	pub const fn trbl(t: i16, r: i16, b: i16, l: i16) -> Self {
		Self { t, r, b, l }
	}

	/// all the same value
	#[inline]
	pub const fn all(x: i16) -> Self {
		Self { t: x, r: x, b: x, l: x }
	}

	/// horizontal, vertical
	#[inline]
	pub const fn hv(h: i16, v: i16) -> Self {
		Self { t: v, r: h, b: v, l: h }
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct WidgetFlags(u32);

#[rustfmt::skip]
#[allow(unused)]
impl WidgetFlags {
	pub const NONE:            Self = Self(0);
	pub const DISABLED:        Self = Self(1 << 0);
	pub const CAN_FOCUS:       Self = Self(1 << 1);
	pub const CAN_HOVER:       Self = Self(1 << 2);
	pub const CAN_CLICK:       Self = Self(1 << 3);
	pub const DRAW_TEXT:       Self = Self(1 << 4);
	pub const DRAW_BORDER:     Self = Self(1 << 5);
	pub const DRAW_BACKGROUND: Self = Self(1 << 6);
	pub const DRAW_SPRITE:     Self = Self(1 << 7);
}

impl WidgetFlags {
	pub fn has(&self, flags: Self) -> bool {
		self.0 & flags.0 == flags.0
	}
}

impl BitAnd for WidgetFlags {
	type Output = Self;

	fn bitand(self, rhs: Self) -> Self::Output {
		Self(self.0 & rhs.0)
	}
}

impl BitAndAssign for WidgetFlags {
	fn bitand_assign(&mut self, rhs: Self) {
		self.0 &= rhs.0;
	}
}

impl BitOr for WidgetFlags {
	type Output = Self;

	fn bitor(self, rhs: Self) -> Self::Output {
		Self(self.0 | rhs.0)
	}
}

impl BitOrAssign for WidgetFlags {
	fn bitor_assign(&mut self, rhs: Self) {
		self.0 |= rhs.0;
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum FlexDirection {
	#[default]
	Horizontal,
	Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum WidgetLayout {
	#[default]
	Stacked,
	Flex {
		direction: FlexDirection,
		gap: i16,
	},
}

impl WidgetLayout {
	#[inline]
	pub fn flex(direction: FlexDirection, gap: i16) -> Self {
		Self::Flex { direction, gap }
	}
}

#[derive(Debug, Clone)]
pub enum WidgetSprite {
	Simple(SpritesheetId, Sprite),
	NineSlice(SpritesheetId, NineSlicingSprite),
}

/// Userland widget properties
#[derive(Debug, Clone, Default)]
pub struct WidgetProps {
	pub key: WidgetKey,

	// feature combinations
	pub flags: WidgetFlags,
	pub color: Color,
	pub text: Option<Text>,
	pub border_color: Color,
	pub border_width: u16,
	pub mask_and: Option<Color>,
	pub mask_or: Option<Color>,
	pub acf: Option<AlphaCompFn>,
	pub sprite: Option<WidgetSprite>,
	pub rotate: Rotate,

	// declarative layout data
	pub anchor: Anchor,
	pub origin: Anchor,
	pub pos: Pos,
	pub draw_offset: Pos,
	pub size: WidgetSize,
	pub padding: WidgetPadding,
	pub layout: WidgetLayout,
}

#[allow(unused)]
impl WidgetProps {
	#[inline]
	pub fn new(key: WidgetKey) -> Self {
		Self { key, ..Self::default() }
	}

	#[inline]
	pub const fn with_flags(mut self, flags: WidgetFlags) -> Self {
		self.flags = flags;
		self
	}

	#[inline]
	pub const fn with_color(mut self, color: Color) -> Self {
		self.color = color;
		self
	}

	#[inline]
	pub fn with_text(mut self, text: Option<Text>) -> Self {
		self.text = text;
		self
	}

	#[inline]
	pub const fn with_border_color(mut self, border_color: Color) -> Self {
		self.border_color = border_color;
		self
	}

	#[inline]
	pub const fn with_border_width(mut self, border_width: u16) -> Self {
		self.border_width = border_width;
		self
	}

	#[inline]
	pub const fn with_mask_and(mut self, mask_and: Option<Color>) -> Self {
		self.mask_and = mask_and;
		self
	}

	#[inline]
	pub const fn with_mask_or(mut self, mask_or: Option<Color>) -> Self {
		self.mask_or = mask_or;
		self
	}

	#[inline]
	pub const fn with_acf(mut self, acf: Option<AlphaCompFn>) -> Self {
		self.acf = acf;
		self
	}

	#[inline]
	pub const fn with_sprite(mut self, sprite: Option<WidgetSprite>) -> Self {
		self.sprite = sprite;
		self
	}

	#[inline]
	pub const fn with_rotate(mut self, rotate: Rotate) -> Self {
		self.rotate = rotate;
		self
	}

	#[inline]
	pub const fn with_anchor_origin(mut self, anchor: Anchor, origin: Anchor) -> Self {
		self.anchor = anchor;
		self.origin = origin;
		self
	}

	#[inline]
	pub const fn with_pos(mut self, pos: Pos) -> Self {
		self.pos = pos;
		self
	}

	#[inline]
	pub const fn with_draw_offset(mut self, draw_offset: Pos) -> Self {
		self.draw_offset = draw_offset;
		self
	}

	#[inline]
	pub const fn with_size(mut self, size: WidgetSize) -> Self {
		self.size = size;
		self
	}

	#[inline]
	pub const fn with_padding(mut self, padding: WidgetPadding) -> Self {
		self.padding = padding;
		self
	}

	#[inline]
	pub const fn with_layout(mut self, layout: WidgetLayout) -> Self {
		self.layout = layout;
		self
	}
}

#[derive(Default)]
pub struct UiContext {
	// I was too lazy to use an actual arena.
	widgets: Vec<RefCell<Widget>>,
	first_freed: Option<WidgetId>,
	keys: HashMap<WidgetKey, WidgetId>,

	viewport_size: Size,
	current_frame: u64,
}

impl UiContext {
	pub const ROOT_WIDGET: WidgetId = Self::id_from_index(0);

	const fn index_from_id(id: WidgetId) -> usize {
		id.0.get() - 1
	}

	const fn id_from_index(index: usize) -> WidgetId {
		// SAFETY: The system cannot access more memory than usize, therefore a usize bound to a number of items cannot overflow.
		WidgetId(unsafe { NonZeroUsize::new_unchecked(index + 1) })
	}

	pub fn new(viewport_size: Size) -> Self {
		Self {
			viewport_size,
			..Self::default()
		}
	}

	pub fn build_widget(&mut self, props: WidgetProps) -> WidgetReaction {
		match self.keys.get(&props.key) {
			Some(&id) => {
				let mut widget = self.widget_mut(id);

				if widget.freed {
					if let Some(prev) = widget.prev {
						let mut prev = self.widget_mut(prev);
						prev.next = widget.next;
						widget.prev = None;
					}

					if let Some(next) = widget.next {
						let mut next = self.widget_mut(next);
						next.prev = widget.prev;
						widget.next = None;
					}
				}

				let widget = widget.deref_mut();
				widget.parent = None;
				widget.prev = None;
				widget.next = None;
				widget.first_child = None;
				widget.last_child = None;
				widget.children_count = 0;
				widget.props = props;

				WidgetReaction {
					id,
					hovered: widget.hovered,
					pressed: widget.pressed,
					clicked: widget.clicked,
				}
			}
			None => {
				let id = Self::id_from_index(self.widgets.len());
				self.keys.insert(props.key, id);
				self.widgets.push(RefCell::new(Widget {
					parent: None,
					prev: None,
					next: None,
					first_child: None,
					last_child: None,
					children_count: 0,

					props,

					last_frame_touched: self.current_frame,
					freed: false,

					hovered: false,
					pressed: false,
					clicked: false,

					solved_rect: Rect::ZERO,
					solved_min_size: Size::ZERO,
				}));

				WidgetReaction {
					id,
					hovered: false,
					pressed: false,
					clicked: false,
				}
			}
		}
	}

	pub fn add_child(&self, wid: WidgetId, child_id: WidgetId) {
		debug_assert_ne!(wid, child_id, "Cannot add a widget as its own child");

		let mut w = self.widget_mut(wid);
		w.children_count += 1;

		if let Some(last_child) = w.last_child {
			self.widget_mut(last_child).next = Some(child_id);
			self.widget_mut(child_id).prev = Some(last_child);
			w.last_child = Some(child_id);
		} else {
			w.first_child = Some(child_id);
			w.last_child = Some(child_id);
		}

		self.widget_mut(child_id).parent = Some(wid);
	}

	fn free_untouched_widgets_rec(&mut self, wid: WidgetId) {
		let (last_frame_touched, child) = {
			let w = self.widget(wid);
			(w.last_frame_touched, w.first_child)
		};

		if last_frame_touched != self.current_frame {
			// free widget

			{
				let mut w = self.widget_mut(wid);
				w.freed = true;
				w.prev = None;
				w.next = self.first_freed;
			}

			self.first_freed = Some(wid);
		}

		let mut child = child;
		while let Some(ch) = child {
			self.free_untouched_widgets_rec(ch);
			child = self.widget(ch).next;
		}
	}

	pub fn free_untouched_widgets(&mut self) {
		self.free_untouched_widgets_rec(Self::ROOT_WIDGET);
	}

	fn draw_widgets_rec(&mut self, draw_cmds: &mut Vec<DrawCommand>, wid: WidgetId) {
		{
			let widget = self.widget(wid);
			let props = &widget.props;

			let acf = props.acf.unwrap_or(alphacomp::over);

			let mut solved_rect = widget.solved_rect;
			solved_rect.x += widget.props.draw_offset.x;
			solved_rect.y += widget.props.draw_offset.y;

			if let Some(mask_and) = props.mask_and {
				draw_cmds.push(DrawCommand::MaskAnd(mask_and));
			}

			if let Some(mask_or) = props.mask_or {
				draw_cmds.push(DrawCommand::MaskOr(mask_or));
			}

			if props.flags.has(WidgetFlags::DRAW_SPRITE) {
				match props.sprite {
					Some(WidgetSprite::Simple(sheet_id, sprite)) => {
						draw_cmds.push(DrawCommand::Sprite {
							pos: solved_rect.pos(),
							rotate: widget.props.rotate,
							sheet_id,
							sprite,
							acf,
						});
					}
					Some(WidgetSprite::NineSlice(sheet_id, nss)) => {
						draw_cmds.push(DrawCommand::NineSlicingSprite {
							rect: solved_rect,
							sheet_id,
							nss,
							acf,
						});
					}
					None => {}
				}
			}

			if props.flags.has(WidgetFlags::DRAW_BACKGROUND) {
				draw_cmds.push(DrawCommand::Fill {
					rect: solved_rect,
					color: props.color,
					acf,
				});
			}

			if props.flags.has(WidgetFlags::DRAW_BORDER) {
				draw_cmds.push(DrawCommand::Stroke {
					rect: solved_rect,
					color: props.border_color,
					stroke_width: 1,
					acf,
				});
			}

			if props.flags.has(WidgetFlags::DRAW_TEXT) {
				if let Some(text) = &widget.props.text {
					draw_cmds.push(DrawCommand::Text {
						text: text.text().clone(),
						pos: solved_rect.pos(),
						acf,
					});
				}
			}

			if props.mask_and.is_some() {
				draw_cmds.push(DrawCommand::MaskAnd(Color::WHITE));
			}

			if props.mask_or.is_some() {
				draw_cmds.push(DrawCommand::MaskOr(Color::TRANSPARENT));
			}
		}

		let mut child = self.widget(wid).first_child;
		while let Some(ch) = child {
			self.draw_widgets_rec(draw_cmds, ch);
			child = self.widget(ch).next;
		}
	}

	pub fn draw_widgets(&mut self, draw_cmds: &mut Vec<DrawCommand>) {
		draw_cmds.push(DrawCommand::BeginComposite);
		draw_cmds.push(DrawCommand::Clear);
		self.draw_widgets_rec(draw_cmds, Self::ROOT_WIDGET);
		draw_cmds.push(DrawCommand::EndComposite(alphacomp::over));
	}

	fn react_rec(&mut self, mouse: &Mouse, wid: WidgetId) -> bool {
		{
			let mut widget = self.widget_mut(wid);
			if widget.props.flags.has(WidgetFlags::DISABLED) {
				widget.hovered = false;
				widget.pressed = false;
				widget.clicked = false;
				return false;
			}
		}

		let mut any_child_hovered = false;
		let mut child = self.widget(wid).first_child;
		while let Some(ch) = child {
			any_child_hovered |= self.react_rec(mouse, ch);
			child = self.widget(ch).next;
		}

		let mut widget = self.widget_mut(wid);
		let can_hover = widget.props.flags.has(WidgetFlags::CAN_HOVER);
		let can_click = widget.props.flags.has(WidgetFlags::CAN_CLICK);

		let pressed_prev = widget.pressed;
		let hovered = !any_child_hovered && widget.solved_rect.contains(mouse.x, mouse.y);

		widget.hovered = can_hover && hovered;
		widget.pressed = can_click
			&& match hovered {
				true => mouse.l_pressed_start() || (mouse.l_pressed() && pressed_prev),
				false => mouse.l_pressed() && pressed_prev,
			};
		widget.clicked = can_click && hovered && mouse.l_pressed_end() && pressed_prev;

		widget.hovered
	}

	pub fn react(&mut self, mouse: &Mouse) {
		// oh no, not React D:
		self.react_rec(mouse, Self::ROOT_WIDGET);
	}

	pub fn widget(&self, wid: WidgetId) -> Ref<'_, Widget> {
		self.widgets[Self::index_from_id(wid)].borrow()
	}

	pub fn widget_mut(&self, wid: WidgetId) -> RefMut<'_, Widget> {
		self.widgets[Self::index_from_id(wid)].borrow_mut()
	}
}

#[derive(Debug, Clone, Default)]
pub struct Mouse {
	pub x: f32,
	pub y: f32,
	pub l_pressed: (bool, bool),
	pub r_pressed: (bool, bool),
	pub m_pressed: (bool, bool),
}

#[allow(unused)]
impl Mouse {
	#[inline]
	pub const fn l_pressed(&self) -> bool {
		self.l_pressed.0
	}

	#[inline]
	pub const fn l_pressed_start(&self) -> bool {
		self.l_pressed.0 && !self.l_pressed.1
	}

	#[inline]
	pub const fn l_pressed_end(&self) -> bool {
		!self.l_pressed.0 && self.l_pressed.1
	}

	#[inline]
	pub const fn r_pressed(&self) -> bool {
		self.r_pressed.0
	}

	#[inline]
	pub const fn r_pressed_start(&self) -> bool {
		self.r_pressed.0 && !self.r_pressed.1
	}

	#[inline]
	pub const fn r_pressed_end(&self) -> bool {
		!self.r_pressed.0 && self.r_pressed.1
	}

	#[inline]
	pub const fn m_pressed(&self) -> bool {
		self.m_pressed.0
	}

	#[inline]
	pub const fn m_pressed_start(&self) -> bool {
		self.m_pressed.0 && !self.m_pressed.1
	}

	#[inline]
	pub const fn m_pressed_end(&self) -> bool {
		!self.m_pressed.0 && self.m_pressed.1
	}
}
