use crate::math::pos::{pos, Pos};
use crate::render::color::{alphacomp, Color};
use crate::render::sprite::NineSlicingSprite;
use crate::render::{DrawCommand, SpritesheetId, Text};
use crate::wk;

use super::{
	Anchor, UiContext, WidgetDim, WidgetFlags, WidgetId, WidgetKey, WidgetLayout, WidgetPadding, WidgetProps,
	WidgetReaction, WidgetSize,
};

impl UiContext {
	pub fn text(&mut self, key: WidgetKey, text: Text, anchor: Anchor, origin: Anchor) -> WidgetReaction {
		self.build_widget(WidgetProps {
			key,

			flags: WidgetFlags::DRAW_TEXT,
			text: Some(text),

			anchor,
			origin,
			size: WidgetSize {
				w: WidgetDim::Hug,
				h: WidgetDim::Hug,
			},

			..WidgetProps::default()
		})
	}

	pub fn button(
		&mut self,
		key: WidgetKey,
		text: Text,
		size: WidgetSize,
		normal_nss: (SpritesheetId, NineSlicingSprite),
	) -> WidgetReaction {
		use WidgetFlags as Wf;

		let button = self.build_widget(WidgetProps {
			key,

			flags: Wf::CAN_FOCUS | Wf::CAN_HOVER | Wf::DRAW_NINE_SLICE,
			nss: Some(normal_nss),

			anchor: Anchor::CENTER,
			origin: Anchor::CENTER,
			padding: WidgetPadding::all(2),
			size,

			..WidgetProps::default()
		});

		let text = self.text(wk!([key]), text, Anchor::CENTER, Anchor::CENTER);
		self.add_child(button.id(), text.id());

		if button.pressed() && button.hovered() {
			let mut w_btn = self.widget_mut(button.id());
			w_btn.props.offset = pos(1, 1);

			let mut w_txt = self.widget_mut(text.id());
			w_txt.props.offset = pos(1, 1);
		}

		button
	}
}
