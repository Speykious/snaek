use crate::math::pos::Pos;
use crate::math::rect::Rect;
use crate::math::size::size;
use crate::math::LayoutRect;

use super::{Anchor, FlexDirection, UiContext, WidgetDim, WidgetId, WidgetLayout};

impl UiContext {
	fn children_fills_count(&self, wid: WidgetId) -> usize {
		let mut count = 0;

		let mut child_id = self.widget(wid).first_child;
		while let Some(child) = child_id {
			let child = self.widget(child);
			if matches!(child.layout, WidgetLayout::Flex { .. }) {
				count += 1;
			}
			child_id = child.next;
		}

		count
	}

	/// Solve minimum sizes of all widgets recursively.
	fn solve_min_sizes_rec(&self, wid: WidgetId) {
		let mut widget = self.widget_mut(wid);

		// solve children's min sizes before parent min sizes
		let mut child_id = widget.first_child;
		while let Some(child) = child_id {
			self.solve_min_sizes_rec(child);
			child_id = self.widget(child).next;
		}

		let solved_min_width = match widget.size.w {
			WidgetDim::Fixed(width) => width,
			WidgetDim::Hug => match (widget.first_child, widget.layout) {
				(Some(child), WidgetLayout::Stacked) => {
					let mut child = self.widget(child);
					let mut min_w = child.solved_min_size.w;

					// take max of min sizes
					while let Some(next_child) = child.next {
						child = self.widget(next_child);
						min_w = min_w.max(child.solved_min_size.w);
					}

					min_w
				}
				(Some(child), WidgetLayout::Flex { direction, gap }) => {
					let mut child = self.widget(child);
					let mut min_w = child.solved_min_size.w;

					match direction {
						FlexDirection::Horizontal => {
							// add min sizes with gaps
							while let Some(next_child) = child.next {
								child = self.widget(next_child);
								min_w += child.solved_min_size.w.saturating_add_signed(gap);
							}
						}
						FlexDirection::Vertical => {
							// take max of min sizes
							while let Some(next_child) = child.next {
								child = self.widget(next_child);
								min_w = min_w.max(child.solved_min_size.w);
							}
						}
					}

					min_w
				}
				(None, _) => 0,
			},
			WidgetDim::Fill => 0,
		};

		let solved_min_height = match widget.size.h {
			WidgetDim::Fixed(height) => height,
			WidgetDim::Hug => match (widget.first_child, widget.layout) {
				(Some(child), WidgetLayout::Stacked) => {
					let mut child = self.widget(child);
					let mut min_h = child.solved_min_size.h;

					// take max of min sizes
					while let Some(next_child) = child.next {
						child = self.widget(next_child);
						min_h = min_h.max(child.solved_min_size.h);
					}

					min_h
				}
				(Some(child), WidgetLayout::Flex { direction, gap }) => {
					let mut child = self.widget(child);
					let mut min_h = child.solved_min_size.h;

					match direction {
						FlexDirection::Horizontal => {
							// take max of min sizes
							while let Some(next_child) = child.next {
								child = self.widget(next_child);
								min_h = min_h.max(child.solved_min_size.h);
							}
						}
						FlexDirection::Vertical => {
							// add min sizes with gaps
							while let Some(next_child) = child.next {
								child = self.widget(next_child);
								min_h += child.solved_min_size.h.saturating_add_signed(gap);
							}
						}
					}

					min_h
				}
				(None, _) => 0,
			},
			WidgetDim::Fill => 0,
		};

		widget.solved_min_size.w = solved_min_width.saturating_add_signed(widget.padding.l + widget.padding.r);
		widget.solved_min_size.h = solved_min_height.saturating_add_signed(widget.padding.t + widget.padding.b);
	}

	/// Solve a widget's rect, based on the parent's solved rect.
	fn solve_rects_rec(&self, wid: WidgetId, parent_solved_rect: Rect) {
		let (current_solved_rect, layout, padding) = {
			let mut widget = self.widget_mut(wid);

			let solved_width = match widget.size.w {
				WidgetDim::Fixed(width) => width,
				WidgetDim::Hug => widget.solved_min_size.w,
				WidgetDim::Fill => parent_solved_rect.w,
			};

			let solved_height = match widget.size.h {
				WidgetDim::Fixed(height) => height,
				WidgetDim::Hug => widget.solved_min_size.h,
				WidgetDim::Fill => parent_solved_rect.h,
			};

			let solved_size = size(solved_width, solved_height);

			let parent_layout_rect = LayoutRect::new(parent_solved_rect, Anchor::TOP_LEFT);
			let current_pos = parent_layout_rect.anchor(widget.anchor);
			let current_rect = Rect::from_pos_size(current_pos, solved_size);

			widget.solved_rect = LayoutRect::new(current_rect, widget.origin).to_rect();

			(widget.solved_rect, widget.layout, widget.padding)
		};

		let inner_solved_rect = Rect {
			x: current_solved_rect.x + padding.l,
			y: current_solved_rect.y + padding.t,
			w: current_solved_rect.w.saturating_add_signed(-(padding.l + padding.r)),
			h: current_solved_rect.h.saturating_add_signed(-(padding.t + padding.b)),
		};

		match layout {
			WidgetLayout::Stacked => {
				let mut child_id = self.widget(wid).first_child;
				while let Some(child) = child_id {
					self.solve_rects_rec(child, current_solved_rect);
					child_id = self.widget(child).next;
				}
			}
			WidgetLayout::Flex { direction, gap } => {
				match direction {
					FlexDirection::Horizontal => {
						let fills_count = self.children_fills_count(wid);

						let filling_width = if fills_count == 0 {
							0
						} else {
							let mut fixed_width: isize = 0;

							let widget = self.widget(wid);
							let mut child_id = widget.first_child;
							while let Some(child) = child_id {
								let child = self.widget(child);
								match child.size.w {
									WidgetDim::Fill => (),
									_ => fixed_width += child.solved_rect.w as isize,
								}
								child_id = child.next;
							}

							let gap_width = gap as isize * widget.children_count.saturating_sub(1) as isize;
							let leftover_width = (inner_solved_rect.w as isize - fixed_width - gap_width).max(0);

							(leftover_width / fills_count as isize) as u16
						};

						let mut x = inner_solved_rect.x;

						let widget = self.widget(wid);
						let mut child_id = widget.first_child;
						while let Some(child) = child_id {
							let (child_w, solved_w, child_next) = {
								let child = self.widget(child);
								(child.size.w, child.solved_rect.w, child.next)
							};

							let child_width = match child_w {
								WidgetDim::Fill => filling_width,
								_ => solved_w,
							};

							let inner_solved_rect = Rect {
								x,
								w: child_width,
								..inner_solved_rect
							};

							self.solve_rects_rec(child, inner_solved_rect);

							x += child_width as i16 + gap;
							child_id = child_next;
						}
					}
					FlexDirection::Vertical => {
						// let fills_count = (children.iter())
						// 	.filter(|child| child.widget.layout().height.is_fill())
						// 	.count();

						// let filling_height = if fills_count == 0 {
						// 	0.
						// } else {
						// 	let fixed_height: f32 = (children.iter())
						// 		.filter_map(|child| solve_height(child.widget.as_ref()))
						// 		.sum();

						// 	let gap_width = flex.gap * children.len().saturating_sub(1) as f32;
						// 	let leftover_height = (inner_layout.height() - fixed_height - gap_width).max(0.);
						// 	leftover_height / fills_count as f32
						// };

						// let mut y = inner_layout.y_start();
						// for child in children.iter_mut() {
						// 	let child_height = solve_height(child.widget.as_ref()).unwrap_or(filling_height);
						// 	let inner_layout = inner_layout.with_height(child_height).with_y(y);
						// 	child.solved_layout = child.widget.solve_layout(&inner_layout);

						// 	y += child_height + flex.gap;
						// }
					}
				}
			}
		}
	}

	pub fn solve_layout(&self) {
		self.solve_min_sizes_rec(Self::ROOT_WIDGET);

		// pretend the parent of the root widget is the framebuffer
		self.solve_rects_rec(Self::ROOT_WIDGET, Rect::from_pos_size(Pos::ZERO, self.viewport_size));
	}
}
