use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::default;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::num::NonZeroUsize;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, DerefMut};

use crate::math::pos::Pos;
use crate::math::rect::Rect;
use crate::math::size::Size;
use crate::render::DrawCommand;
use crate::wk;

pub mod ascii_sheet;
pub mod components;
pub mod layout;

pub use ascii_sheet::{ascii_sheet, AsciiSheet};

/// ID of a widget.
///
/// The root of all widgets, which is the window itself, has an ID of 0. So all top-level widgets have a `parent_id` of 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WidgetId(NonZeroUsize);

/// A key that uniquely identifies a widget.
///
/// It contains
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
	( $( [ $($key:expr),* ] )? $($n:expr),* ) => {
		$crate::ui::WidgetKeyHasher::new(::std::hash::DefaultHasher::default())
			.hash_loc(module_path!(), line!(), column!())
			$(.hash_key($key))*
			$(.hash_n($n))*
			.finish()
	};
}

#[derive(Debug, Clone)]
pub struct Widget {
	id: WidgetId,
	key: WidgetKey,

	// tree links
	// It's side-stepping-the-borrow-checker time!
	// note: `next` and `prev` get reused as an intrusive free list when the widget is "freed".
	parent: Option<WidgetId>,
	prev: Option<WidgetId>,
	next: Option<WidgetId>,
	first_child: Option<WidgetId>,
	last_child: Option<WidgetId>,
	children_count: usize,

	// feature combinations
	flags: WidgetFlags,

	// declarative layout data
	anchor: Anchor,
	origin: Anchor,
	offset: Pos,
	size: WidgetSize,
	padding: WidgetPadding,

	layout: WidgetLayout,

	// persistent state
	/// If a widget hasn't been touched in the current frame,
	/// we "free" it (using a free list)
	last_frame_touched: u64,
	freed: bool,

	// Layout state calculated each frame
	solved_rect: Rect,
	solved_min_size: Size,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Anchor {
	pub x: f32,
	pub y: f32,
}

#[rustfmt::skip]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(C, align(8))]
pub struct WidgetPadding {
	pub t: i16,
	pub r: i16,
	pub b: i16,
	pub l: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct WidgetFlags(u32);

#[rustfmt::skip]
impl WidgetFlags {
	pub const NONE:            Self = Self(0);
	pub const CAN_FOCUS:       Self = Self(1 << 0);
	pub const CAN_HOVER:       Self = Self(1 << 1);
	pub const DRAW_TEXT:       Self = Self(1 << 2);
	pub const DRAW_BORDER:     Self = Self(1 << 3);
	pub const DRAW_BACKGROUND: Self = Self(1 << 4);
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

#[derive(Debug, Clone)]
pub struct WidgetBuilder {
	pub key: WidgetKey,
	pub flags: WidgetFlags,
	pub anchor: Anchor,
	pub origin: Anchor,
	pub offset: Pos,
	pub size: WidgetSize,
	pub padding: WidgetPadding,
	pub layout: WidgetLayout,
}

impl WidgetBuilder {
	#[inline]
	pub(super) fn reset(&self, w: &mut Widget) {
		w.parent = None;
		w.prev = None;
		w.next = None;
		w.first_child = None;
		w.last_child = None;

		w.flags = self.flags;

		w.anchor = self.anchor;
		w.origin = self.origin;
		w.offset = self.offset;
		w.size = self.size;

		w.layout = self.layout;
	}

	#[inline]
	pub(super) const fn build(self, id: WidgetId, current_frame: u64) -> Widget {
		Widget {
			id,
			key: self.key,

			parent: None,
			prev: None,
			next: None,
			first_child: None,
			last_child: None,
			children_count: 0,

			flags: self.flags,

			anchor: self.anchor,
			origin: self.origin,
			offset: self.offset,
			size: self.size,
			padding: self.padding,

			layout: self.layout,

			last_frame_touched: current_frame,
			freed: false,

			solved_rect: Rect::ZERO,
			solved_min_size: Size::ZERO,
		}
	}
}

#[derive(Default)]
pub struct UiContext {
	// I was too lazy to use an actual arena.
	widgets: Vec<RefCell<Widget>>,
	first_freed: Option<WidgetId>,
	keys: HashMap<WidgetKey, WidgetId>,

	draw_commands: Vec<DrawCommand>,

	viewport_size: Size,
	mouse_pos: Pos,
	current_frame: u64,
}

impl UiContext {
	pub const ROOT_WIDGET: WidgetId = Self::id_from_index(0);

	const fn index_from_id(id: WidgetId) -> usize {
		id.0.get() - 1
	}

	const fn id_from_index(index: usize) -> WidgetId {
		WidgetId(unsafe { NonZeroUsize::new_unchecked(index + 1) })
	}

	pub fn new(viewport_size: Size) -> Self {
		Self {
			viewport_size,
			..Self::default()
		}
	}

	pub fn build_widget(&mut self, builder: WidgetBuilder) -> WidgetId {
		match self.keys.get(&builder.key) {
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

				builder.reset(widget.deref_mut());
				widget.last_frame_touched = self.current_frame;
				id
			}
			None => {
				let id = Self::id_from_index(self.widgets.len());
				self.keys.insert(builder.key, id);
				self.widgets.push(RefCell::new(builder.build(id, self.current_frame)));
				id
			}
		}
	}

	pub fn add_child(&self, wid: WidgetId, child_id: WidgetId) {
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
		let (last_frame_touched, child, next) = {
			let mut w = self.widget_mut(wid);
			(w.last_frame_touched, w.first_child, w.next)
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

		if let Some(child) = child {
			self.free_untouched_widgets_rec(child);
		}

		if let Some(next) = next {
			self.free_untouched_widgets_rec(next);
		}
	}

	pub fn free_untouched_widgets(&mut self) {
		self.free_untouched_widgets_rec(Self::ROOT_WIDGET);
	}

	pub fn push_draw(&mut self, cmd: DrawCommand) {
		self.draw_commands.push(cmd);
	}

	pub fn clear_draws(&mut self) {
		self.draw_commands.clear();
	}

	pub fn flush_draws(&mut self, out_cmds: &mut Vec<DrawCommand>) {
		out_cmds.extend(self.draw_commands.drain(..));
	}

	pub fn draw_commands(&self) -> &[DrawCommand] {
		&self.draw_commands
	}

	pub fn widget(&self, wid: WidgetId) -> Ref<'_, Widget> {
		self.widgets[Self::index_from_id(wid)].borrow()
	}

	pub fn widget_mut(&self, wid: WidgetId) -> RefMut<'_, Widget> {
		self.widgets[Self::index_from_id(wid)].borrow_mut()
	}
}
