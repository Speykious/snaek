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
	pub fn text(&mut self, key: WidgetKey, text: Text, anchor: Anchor, origin: Anchor) -> (WidgetId, WidgetReaction) {
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
		hover_nss: (SpritesheetId, NineSlicingSprite),
	) -> (WidgetId, WidgetReaction) {
		use WidgetFlags as Wf;

		let (button_id, button) = self.build_widget(WidgetProps {
			key,

			flags: Wf::CAN_FOCUS | Wf::CAN_HOVER | Wf::DRAW_NINE_SLICE,
			nss: Some(normal_nss),

			anchor: Anchor::CENTER,
			origin: Anchor::CENTER,
			padding: WidgetPadding::all(2),
			size,

			..WidgetProps::default()
		});

		if button.pressed {
			self.widget_mut(button_id).props.nss = Some(hover_nss);
		}

		let (text_id, _) = self.text(wk!([key]), text, Anchor::CENTER, Anchor::CENTER);
		self.add_child(button_id, text_id);

		(button_id, button)
	}
}
