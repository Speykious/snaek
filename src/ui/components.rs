use crate::math::pos::pos;
use crate::render::pixel::alphacomp;
use crate::render::sprite::NineSlicingSprite;
use crate::render::DrawCommand;

use super::{
	Anchor, UiContext, WidgetBuilder, WidgetDim, WidgetFlags, WidgetId, WidgetKey, WidgetLayout,
	WidgetPadding, WidgetSize,
};

impl UiContext {
	pub fn frame(&mut self, key: WidgetKey, anchor: Anchor, origin: Anchor, size: WidgetSize, layout: WidgetLayout) -> WidgetId {
		self.build_widget(WidgetBuilder {
			key,
			flags: WidgetFlags::NONE, // The flags don't do anything rn...
			anchor,
			origin,
			offset: pos(0, 0),
			size,
			padding: WidgetPadding::default(),
			layout,
		})
	}

	pub fn button(
		&mut self,
		key: WidgetKey,
		text: &str,
		normal_nss: NineSlicingSprite,
		hover_nss: NineSlicingSprite,
	) -> WidgetId {
		let wid = self.build_widget(WidgetBuilder {
			key,
			flags: WidgetFlags::NONE, // The flags don't do anything rn...
			anchor: Anchor::CENTER,
			origin: Anchor::CENTER,
			offset: pos(0, 0),
			size: WidgetSize {
				w: WidgetDim::Fixed(30),
				h: WidgetDim::Fixed(9),
			},
			padding: WidgetPadding::default(),
			layout: WidgetLayout::Stacked,
		});

		let rect = self.widget(wid).solved_rect;

		self.push_draw(DrawCommand::NineSlicingSprite {
			rect,
			nss: normal_nss,
			acf: alphacomp::over,
		});
		self.push_draw(DrawCommand::Text {
			text: text.to_string(),
			pos: rect.pos() + pos(2, 2),
			acf: alphacomp::over,
		});

		wid
	}
}
