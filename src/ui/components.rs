use crate::math::pos::pos;
use crate::render::color::alphacomp::{self, AlphaCompFn};
use crate::render::color::Color;
use crate::render::sprite::{NineSlicingSprite, Sprite};
use crate::render::{SpritesheetId, Text};
use crate::ui::WidgetSprite;
use crate::wk;

use super::{
	Anchor, FlexDirection, UiContext, Widget, WidgetDim, WidgetFlags, WidgetId, WidgetKey, WidgetLayout, WidgetPadding, WidgetProps, WidgetReaction, WidgetSize
};

impl UiContext {
	pub fn text(&mut self, key: WidgetKey, text: Text, anchor: Anchor, origin: Anchor) -> WidgetReaction {
		self.build_widget(WidgetProps {
			key,

			flags: WidgetFlags::DRAW_TEXT,
			text: Some(text),

			anchor,
			origin,
			size: WidgetSize::hug(),

			..WidgetProps::default()
		})
	}

	pub fn sprite(
		&mut self,
		key: WidgetKey,
		sheet_id: SpritesheetId,
		sprite: Sprite,
		anchor: Anchor,
		origin: Anchor,
		acf: Option<AlphaCompFn>,
	) -> WidgetReaction {
		self.build_widget(WidgetProps {
			key,

			flags: WidgetFlags::DRAW_SPRITE,
			sprite: Some(WidgetSprite::Simple(sheet_id, sprite)),
			acf,

			anchor,
			origin,
			size: WidgetSize::hug(),

			..WidgetProps::default()
		})
	}

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

		let button = self.build_widget(WidgetProps {
			key,

			flags: Wf::CAN_FOCUS | Wf::CAN_HOVER | Wf::CAN_CLICK | Wf::DRAW_BACKGROUND,

			anchor,
			origin,
			size,

			..WidgetProps::default()
		});

		let inner_sprite = self.sprite(
			wk!([key]),
			sheet_id,
			sprite,
			Anchor::CENTER,
			Anchor::CENTER,
			Some(alphacomp::xor),
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

		let button = self.build_widget(WidgetProps {
			key,

			flags: Wf::CAN_FOCUS | Wf::CAN_HOVER | Wf::CAN_CLICK | Wf::DRAW_SPRITE,
			sprite: Some(normal_nss),

			anchor,
			origin,
			padding,
			size,

			..WidgetProps::default()
		});

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
		let display = self.build_widget(WidgetProps {
			key,
			flags: WidgetFlags::DRAW_SPRITE,
			size: WidgetSize::hug(),
			sprite: Some(WidgetSprite::NineSlice(sheet_id, display_box)),
			layout: WidgetLayout::Flex {
				direction: FlexDirection::Horizontal,
				gap: 2,
			},
			padding: WidgetPadding::hv(3, 2),
			..WidgetProps::default()
		});

		let mut after_first_digit = false;
		for (i, d) in [(2, (n / 100) % 10), (1, (n / 10) % 10), (0, n % 10)] {
			let digit_holder = self.build_widget(WidgetProps {
				key: wk!([key] i),
				flags: WidgetFlags::DRAW_SPRITE,
				size: WidgetSize::hug(),
				sprite: Some(WidgetSprite::Simple(sheet_id, placeholder_sprite)),
				..WidgetProps::default()
			});

			if after_first_digit || d > 0 {
				let digit = self.build_widget(WidgetProps {
					key: wk!([key] i),
					flags: WidgetFlags::DRAW_SPRITE,
					size: WidgetSize::hug(),
					sprite: Some(WidgetSprite::Simple(sheet_id, digit_sprites[d])),
					..WidgetProps::default()
				});
				self.add_child(digit_holder.id(), digit.id());

				after_first_digit = true;
			}

			self.add_child(display.id(), digit_holder.id());
		}

		display
	}
}
