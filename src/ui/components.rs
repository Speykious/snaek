use crate::math::pos::pos;
use crate::render::color::alphacomp::{self, AlphaCompFn};
use crate::render::color::Color;
use crate::render::sprite::{NineSlicingSprite, Sprite};
use crate::render::{SpritesheetId, Text};
use crate::ui::WidgetSprite;
use crate::wk;

use super::{
	Anchor, UiContext, Widget, WidgetDim, WidgetFlags, WidgetKey, WidgetPadding, WidgetProps, WidgetReaction,
	WidgetSize,
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
		text: Text,
		size: WidgetSize,
		normal_nss: WidgetSprite,
		hover_nss: WidgetSprite,
	) -> WidgetReaction {
		use WidgetFlags as Wf;

		let button = self.build_widget(WidgetProps {
			key,

			flags: Wf::CAN_FOCUS | Wf::CAN_HOVER | Wf::CAN_CLICK | Wf::DRAW_SPRITE,
			sprite: Some(normal_nss),

			anchor: Anchor::CENTER,
			origin: Anchor::CENTER,
			padding: WidgetPadding::all(2),
			size,

			..WidgetProps::default()
		});

		let inner_text = self.text(wk!([key]), text, Anchor::CENTER, Anchor::CENTER);
		self.add_child(button.id(), inner_text.id());

		if button.pressed() && button.hovered() {
			let mut w_btn = self.widget_mut(button.id());
			w_btn.props.sprite = Some(hover_nss);
			w_btn.props.draw_offset = pos(1, 1);

			let mut w_txt = self.widget_mut(inner_text.id());
			w_txt.props.draw_offset = pos(1, 1);
		}

		button
	}
}
