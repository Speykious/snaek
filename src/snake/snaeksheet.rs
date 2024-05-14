use crate::math::rect::Rect;
use crate::render::sprite::{NineSlicingSprite, Sprite};

#[derive(Debug)]
#[repr(C)]
pub struct SnaekSheet {
	/// Mouse cursor
	pub cursor: Sprite,

	/// Head of the snake
	pub snake_head: Sprite,
	/// When the snake goes straight
	pub snake_straight: Sprite,
	/// When the snake goes gay (it turns left/right)
	pub snake_gay: Sprite,
	/// End of the snake
	pub snake_end: Sprite,
	/// Tongue of the snake
	pub snake_tongue: Sprite,

	/// Yellow banana
	pub banana_yellow: Sprite,
	/// Red banana
	pub banana_red: Sprite,
	/// Cyan banana
	pub banana_cyan: Sprite,

	/// playfield box
	pub box_playfield: NineSlicingSprite,
	/// big carved box
	pub box_big_carved: NineSlicingSprite,
	/// number display box
	pub box_num_display: NineSlicingSprite,
	/// text input box
	pub box_text_input: NineSlicingSprite,
	/// embossed box
	pub box_embossed: NineSlicingSprite,
	/// carved box
	pub box_carved: NineSlicingSprite,
	/// green box
	pub box_green: NineSlicingSprite,
	/// red box
	pub box_red: NineSlicingSprite,

	/// carved separator line
	pub carved_sep_line: Sprite,

	/// Snaek game icon
	pub snaek_icon: Sprite,

	/// Minimize button icon
	pub icon_minimize: Sprite,
	/// Close button icon
	pub icon_close: Sprite,

	/// Play/debug button's play icon
	pub icon_play: Sprite,
	/// Play/debug button's pause icon
	pub icon_debug: Sprite,
	/// Restart button icon
	pub icon_restart: Sprite,

	/// Exclamation mark on the small number display
	pub num_bang: Sprite,
	/// Colon on the small number display
	pub num_colon: Sprite,
	/// digits on the small number display
	pub nums: [Sprite; 10],

	/// digit placeholder on the big number display
	pub bignum_placeholder: Sprite,
	/// digits on the big number display
	pub bignums: [Sprite; 10],
}

#[rustfmt::skip]
pub fn snaek_sheet() -> SnaekSheet {
	SnaekSheet {
		cursor:             Sprite::new(Rect::from_xywh( 24,   0,  4,  6)),

		snake_head:         Sprite::new(Rect::from_xywh( 14,   0,  7,  7)),
		snake_straight:     Sprite::new(Rect::from_xywh(  7,   0,  7,  7)),
		snake_gay:          Sprite::new(Rect::from_xywh(  0,   0,  7,  7)),
		snake_end:          Sprite::new(Rect::from_xywh(  0,   7,  7,  7)),
		snake_tongue:       Sprite::new(Rect::from_xywh( 21,   2,  3,  3)),

		banana_yellow:      Sprite::new(Rect::from_xywh(  7,   7,  7,  7)),
		banana_red:         Sprite::new(Rect::from_xywh( 14,   7,  7,  7)),
		banana_cyan:        Sprite::new(Rect::from_xywh( 21,   7,  7,  7)),

		box_playfield:      NineSlicingSprite::new(Rect::from_xywh(  9,  14,  9,  9),  4,  5,  4,  5),
		box_big_carved:     NineSlicingSprite::new(Rect::from_xywh( 18,  14,  5,  5),  2,  3,  2,  3),
		box_num_display:    NineSlicingSprite::new(Rect::from_xywh( 23,  14,  3,  3),  1,  2,  1,  2),
		box_text_input:     NineSlicingSprite::new(Rect::from_xywh( 26,  14,  3,  3),  1,  2,  1,  2),
		box_embossed:       NineSlicingSprite::new(Rect::from_xywh( 23,  17,  3,  3),  1,  2,  1,  2),
		box_carved:         NineSlicingSprite::new(Rect::from_xywh( 26,  17,  3,  3),  1,  2,  1,  2),
		box_green:          NineSlicingSprite::new(Rect::from_xywh( 23,  20,  3,  3),  1,  2,  1,  2),
		box_red:            NineSlicingSprite::new(Rect::from_xywh( 26,  20,  3,  3),  1,  2,  1,  2),

		carved_sep_line:    Sprite::new(Rect::from_xywh( 19,  20,  1,  2)),

		snaek_icon:         Sprite::new(Rect::from_xywh( 29,  15,  6,  6)),

		icon_minimize:      Sprite::new(Rect::from_xywh(  0,  14,  5,  1)),
		icon_close:         Sprite::new(Rect::from_xywh(  1,  16,  3,  3)),

		icon_play:          Sprite::new(Rect::from_xywh(  1,  19,  4,  4)),
		icon_debug:         Sprite::new(Rect::from_xywh(  5,  19,  4,  4)),
		icon_restart:       Sprite::new(Rect::from_xywh(  5,  15,  4,  4)),

		num_bang:           Sprite::new(Rect::from_xywh(  0,  23,  1,  5)),
		num_colon:          Sprite::new(Rect::from_xywh(  2,  23,  1,  5)),

		nums:             [ Sprite::new(Rect::from_xywh(  4,  23,  3,  5)),
		                    Sprite::new(Rect::from_xywh(  7,  23,  3,  5)),
		                    Sprite::new(Rect::from_xywh( 10,  23,  3,  5)),
		                    Sprite::new(Rect::from_xywh( 13,  23,  3,  5)),
		                    Sprite::new(Rect::from_xywh( 16,  23,  3,  5)),
		                    Sprite::new(Rect::from_xywh( 19,  23,  3,  5)),
		                    Sprite::new(Rect::from_xywh( 22,  23,  3,  5)),
		                    Sprite::new(Rect::from_xywh( 25,  23,  3,  5)),
		                    Sprite::new(Rect::from_xywh( 28,  23,  3,  5)),
		                    Sprite::new(Rect::from_xywh( 31,  23,  3,  5)) ],

		bignum_placeholder: Sprite::new(Rect::from_xywh( 28,   0,  8, 14)),

		bignums:          [ Sprite::new(Rect::from_xywh( 36,   0,  8, 14)),
		                    Sprite::new(Rect::from_xywh( 44,   0,  8, 14)),
		                    Sprite::new(Rect::from_xywh( 52,   0,  8, 14)),
		                    Sprite::new(Rect::from_xywh( 60,   0,  8, 14)),
		                    Sprite::new(Rect::from_xywh( 68,   0,  8, 14)),
		                    Sprite::new(Rect::from_xywh( 36,  14,  8, 14)),
		                    Sprite::new(Rect::from_xywh( 44,  14,  8, 14)),
		                    Sprite::new(Rect::from_xywh( 52,  14,  8, 14)),
		                    Sprite::new(Rect::from_xywh( 60,  14,  8, 14)),
		                    Sprite::new(Rect::from_xywh( 68,  14,  8, 14)) ],
	}
}
