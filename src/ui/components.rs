use std::time::Duration;

use crate::math::pos::pos;
use crate::render::color::alphacomp::{self, AlphaCompFn};
use crate::render::color::Color;
use crate::render::sprite::{NineSlicingSprite, Sprite};
use crate::render::{SpritesheetId, Text};
use crate::ui::WidgetSprite;
use crate::wk;

use super::{
	Anchor, FlexDirection, UiContext, Widget, WidgetDim, WidgetFlags, WidgetId, WidgetKey, WidgetLayout, WidgetPadding,
	WidgetProps, WidgetReaction, WidgetSize,
};

impl WidgetProps {
	#[inline]
	pub fn text(key: WidgetKey, text: Text) -> Self {
		Self {
			key,

			flags: WidgetFlags::DRAW_TEXT,
			text: Some(text),
			size: WidgetSize::hug(),

			..WidgetProps::default()
		}
	}

	#[inline]
	pub fn sprite(key: WidgetKey, sprite: WidgetSprite) -> Self {
		Self {
			key,

			flags: WidgetFlags::DRAW_SPRITE,
			sprite: Some(sprite),
			size: WidgetSize::hug(),

			..WidgetProps::default()
		}
	}

	#[inline]
	pub fn simple_sprite(key: WidgetKey, sheet_id: SpritesheetId, sprite: Sprite) -> Self {
		Self::sprite(key, WidgetSprite::Simple(sheet_id, sprite))
	}

	#[inline]
	pub fn nine_slice_sprite(key: WidgetKey, sheet_id: SpritesheetId, sprite: NineSlicingSprite) -> Self {
		Self::sprite(key, WidgetSprite::NineSlice(sheet_id, sprite))
	}
}

impl UiContext {
	pub fn btn_icon(
		&mut self,
		key: WidgetKey,
		sheet_id: SpritesheetId,
		sprite: Sprite,
		size: WidgetSize,
		anchor: Anchor,
		origin: Anchor,
		hover_color: Color,
	) -> WidgetReaction {
		use WidgetFlags as Wf;

		let button = self.build_widget(
			WidgetProps::new(key)
				.with_flags(Wf::CAN_FOCUS | Wf::CAN_HOVER | Wf::CAN_CLICK | Wf::DRAW_BACKGROUND)
				.with_anchor_origin(anchor, origin)
				.with_size(size),
		);

		let inner_sprite = self.build_widget(
			WidgetProps::simple_sprite(wk!([key]), sheet_id, sprite)
				.with_anchor_origin(Anchor::CENTER, Anchor::CENTER)
				.with_acf(Some(alphacomp::xor)),
		);
		self.add_child(button.id(), inner_sprite.id());

		if button.hovered() {
			let mut w_btn = self.widget_mut(button.id());
			w_btn.props.color = hover_color;
		}

		button
	}

	pub fn btn_box(
		&mut self,
		key: WidgetKey,
		size: WidgetSize,
		padding: WidgetPadding,
		normal_nss: WidgetSprite,
		hover_nss: WidgetSprite,
		anchor: Anchor,
		origin: Anchor,
		child_id: WidgetId,
	) -> WidgetReaction {
		use WidgetFlags as Wf;

		let button = self.build_widget(
			WidgetProps::new(key)
				.with_flags(Wf::CAN_FOCUS | Wf::CAN_HOVER | Wf::CAN_CLICK | Wf::DRAW_SPRITE)
				.with_sprite(Some(normal_nss))
				.with_anchor_origin(anchor, origin)
				.with_padding(padding)
				.with_size(size),
		);

		self.add_child(button.id(), child_id);

		if button.pressed() && button.hovered() {
			let mut w_btn = self.widget_mut(button.id());
			w_btn.props.sprite = Some(hover_nss);
			w_btn.props.draw_offset = pos(1, 1);

			let mut w_txt = self.widget_mut(child_id);
			w_txt.props.draw_offset = pos(1, 1);
		}

		button
	}

	pub fn big_3digits_display(
		&mut self,
		key: WidgetKey,
		n: usize,
		sheet_id: SpritesheetId,
		display_box: NineSlicingSprite,
		placeholder_sprite: Sprite,
		digit_sprites: &[Sprite; 10],
	) -> WidgetReaction {
		let display = self.build_widget(
			WidgetProps::nine_slice_sprite(key, sheet_id, display_box)
				.with_layout(WidgetLayout::flex(FlexDirection::Horizontal, 2))
				.with_padding(WidgetPadding::hv(3, 2)),
		);

		let mut after_first_digit = false;
		for (i, d) in [(2, (n / 100) % 10), (1, (n / 10) % 10), (0, n % 10)] {
			let digit_holder =
				self.build_widget(WidgetProps::simple_sprite(wk!([key] i), sheet_id, placeholder_sprite));

			if after_first_digit || d > 0 {
				let digit = self.build_widget(WidgetProps::simple_sprite(wk!([key] i), sheet_id, digit_sprites[d]));
				self.add_child(digit_holder.id(), digit.id());

				after_first_digit = true;
			}

			self.add_child(display.id(), digit_holder.id());
		}

		display
	}

	pub fn time_display(
		&mut self,
		key: WidgetKey,
		time: Duration,
		sheet_id: SpritesheetId,
		display_box: NineSlicingSprite,
		colon_sprite: Sprite,
		digit_sprites: &[Sprite; 10],
	) -> WidgetReaction {
		let display = self.build_widget(
			WidgetProps::nine_slice_sprite(key, sheet_id, display_box)
				.with_layout(WidgetLayout::flex(FlexDirection::Horizontal, 1))
				.with_padding(WidgetPadding::all(2)),
		);

		let seconds = time.as_secs();
		let minutes = ((seconds / 60) % 60) as usize;
		let seconds = (seconds % 60) as usize;
		let millis = (time.as_millis() % 1000) as usize;

		for (i, d) in [(1, (minutes / 10) % 10), (0, minutes % 10)] {
			let digit = self.build_widget(WidgetProps::simple_sprite(wk!([key] i), sheet_id, digit_sprites[d]));
			self.add_child(display.id(), digit.id());
		}

		let colon = self.build_widget(WidgetProps::simple_sprite(wk!([key]), sheet_id, colon_sprite));
		self.add_child(display.id(), colon.id());

		for (i, d) in [(1, (seconds / 10) % 10), (0, seconds % 10)] {
			let digit = self.build_widget(WidgetProps::simple_sprite(wk!([key] i), sheet_id, digit_sprites[d]));
			self.add_child(display.id(), digit.id());
		}

		let colon = self.build_widget(WidgetProps {
			key: wk!([key]),
			flags: WidgetFlags::DRAW_SPRITE,
			size: WidgetSize::hug(),
			sprite: Some(WidgetSprite::Simple(sheet_id, colon_sprite)),
			..WidgetProps::default()
		});
		self.add_child(display.id(), colon.id());

		for (i, d) in [(2, (millis / 100) % 10), (1, (millis / 10) % 10), (0, millis % 10)] {
			let digit = self.build_widget(WidgetProps::simple_sprite(wk!([key] i), sheet_id, digit_sprites[d]));
			self.add_child(display.id(), digit.id());
		}

		display
	}
}
