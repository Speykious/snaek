use crate::math::pos::{pos, Pos};
use crate::render::color::{alphacomp, Color};
use crate::render::sprite::NineSlicingSprite;
use crate::render::{DrawCommand, Text, SpritesheetId};
use crate::wk;

use super::{
	Anchor, UiContext, WidgetDim, WidgetFlags, WidgetId, WidgetKey, WidgetLayout, WidgetPadding, WidgetProps,
	WidgetSize,
};

impl UiContext {
	pub fn frame(
		&mut self,
		key: WidgetKey,
		anchor: Anchor,
		origin: Anchor,
		size: WidgetSize,
		layout: WidgetLayout,
	) -> WidgetId {
		self.build_widget(WidgetProps {
			key,

			anchor,
			origin,
			size,
			layout,

			..WidgetProps::default()
		})
	}

	pub fn text(&mut self, key: WidgetKey, text: Text, anchor: Anchor, origin: Anchor) -> WidgetId {
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
	) -> WidgetId {
		use WidgetFlags as Wf;

		let button_id = self.build_widget(WidgetProps {
			key,

			flags: Wf::CAN_FOCUS | Wf::CAN_HOVER | Wf::DRAW_NINE_SLICE,
			nss: Some(normal_nss),

			anchor: Anchor::CENTER,
			origin: Anchor::CENTER,
			padding: WidgetPadding::all(2),
			size,

			..WidgetProps::default()
		});

		let text_id = self.text(wk!([key]), text, Anchor::CENTER, Anchor::CENTER);
		self.add_child(button_id, text_id);

		button_id
	}
}
