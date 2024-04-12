use crate::math::rect::Rect;
use crate::render::{NineSlicingSprite, Sprite, SpritesheetId};

#[derive(Debug)]
#[repr(C)]
pub struct AsciiCharsSpritesheet {
	/// Space character (` `)
	pub space: Sprite,

	/// Upper A letter (`A`)
	pub upper_a: Sprite,
	/// Upper B letter (`B`)
	pub upper_b: Sprite,
	/// Upper C letter (`C`)
	pub upper_c: Sprite,
	/// Upper D letter (`D`)
	pub upper_d: Sprite,
	/// Upper E letter (`E`)
	pub upper_e: Sprite,
	/// Upper F letter (`F`)
	pub upper_f: Sprite,
	/// Upper G letter (`G`)
	pub upper_g: Sprite,
	/// Upper H letter (`H`)
	pub upper_h: Sprite,
	/// Upper I letter (`I`)
	pub upper_i: Sprite,
	/// Upper J letter (`J`)
	pub upper_j: Sprite,
	/// Upper K letter (`K`)
	pub upper_k: Sprite,
	/// Upper L letter (`L`)
	pub upper_l: Sprite,
	/// Upper M letter (`M`)
	pub upper_m: Sprite,
	/// Upper N letter (`N`)
	pub upper_n: Sprite,
	/// Upper O letter (`O`)
	pub upper_o: Sprite,
	/// Upper P letter (`P`)
	pub upper_p: Sprite,
	/// Upper Q letter (`Q`)
	pub upper_q: Sprite,
	/// Upper R letter (`R`)
	pub upper_r: Sprite,
	/// Upper S letter (`S`)
	pub upper_s: Sprite,
	/// Upper T letter (`T`)
	pub upper_t: Sprite,
	/// Upper U letter (`U`)
	pub upper_u: Sprite,
	/// Upper V letter (`V`)
	pub upper_v: Sprite,
	/// Upper W letter (`W`)
	pub upper_w: Sprite,
	/// Upper X letter (`X`)
	pub upper_x: Sprite,
	/// Upper Y letter (`Y`)
	pub upper_y: Sprite,
	/// Upper Z letter (`Z`)
	pub upper_z: Sprite,

	/// Lower A letter (`a`)
	pub lower_a: Sprite,
	/// Lower B letter (`b`)
	pub lower_b: Sprite,
	/// Lower C letter (`c`)
	pub lower_c: Sprite,
	/// Lower D letter (`d`)
	pub lower_d: Sprite,
	/// Lower E letter (`e`)
	pub lower_e: Sprite,
	/// Lower F letter (`f`)
	pub lower_f: Sprite,
	/// Lower G letter (`g`)
	pub lower_g: Sprite,
	/// Lower H letter (`h`)
	pub lower_h: Sprite,
	/// Lower I letter (`i`)
	pub lower_i: Sprite,
	/// Lower J letter (`j`)
	pub lower_j: Sprite,
	/// Lower K letter (`k`)
	pub lower_k: Sprite,
	/// Lower L letter (`l`)
	pub lower_l: Sprite,
	/// Lower M letter (`m`)
	pub lower_m: Sprite,
	/// Lower N letter (`n`)
	pub lower_n: Sprite,
	/// Lower O letter (`o`)
	pub lower_o: Sprite,
	/// Lower P letter (`p`)
	pub lower_p: Sprite,
	/// Lower Q letter (`q`)
	pub lower_q: Sprite,
	/// Lower R letter (`r`)
	pub lower_r: Sprite,
	/// Lower S letter (`s`)
	pub lower_s: Sprite,
	/// Lower T letter (`t`)
	pub lower_t: Sprite,
	/// Lower U letter (`u`)
	pub lower_u: Sprite,
	/// Lower V letter (`v`)
	pub lower_v: Sprite,
	/// Lower W letter (`w`)
	pub lower_w: Sprite,
	/// Lower X letter (`x`)
	pub lower_x: Sprite,
	/// Lower Y letter (`y`)
	pub lower_y: Sprite,
	/// Lower Z letter (`z`)
	pub lower_z: Sprite,

	/// Digit zero (`0`)
	pub digit_0: Sprite,
	/// Digit one (`1`)
	pub digit_1: Sprite,
	/// Digit two (`2`)
	pub digit_2: Sprite,
	/// Digit three (`3`)
	pub digit_3: Sprite,
	/// Digit four (`4`)
	pub digit_4: Sprite,
	/// Digit five (`5`)
	pub digit_5: Sprite,
	/// Digit six (`6`)
	pub digit_6: Sprite,
	/// Digit seven (`7`)
	pub digit_7: Sprite,
	/// Digit eight (`8`)
	pub digit_8: Sprite,
	/// Digit nine (`9`)
	pub digit_9: Sprite,

	/// Exclamation mark character (`!`)
	pub exclamation_mark: Sprite,
	/// Question mark character (`?`)
	pub question_mark: Sprite,
	/// Colon character (`:`)
	pub colon: Sprite,
	/// Semicolon character (`;`)
	pub semicolon: Sprite,
	/// Comma character (`,`)
	pub comma: Sprite,
	/// Period character (`.`)
	pub period: Sprite,
	/// Star character (`*`)
	pub star: Sprite,
	/// Hashtag character (`#`)
	pub hashtag: Sprite,
	/// Single quote character (`'`)
	pub single_quote: Sprite,
	/// Double quote character (`"`)
	pub double_quote: Sprite,
	/// Left bracket character (`[`)
	pub bracket_l: Sprite,
	/// Right bracket character (`]`)
	pub bracket_r: Sprite,
	/// Left parenthesis character (`(`)
	pub parens_l: Sprite,
	/// Right parenthesis character (`)`)
	pub parens_r: Sprite,
	/// Left brace character (`{`)
	pub brace_l: Sprite,
	/// Right brace character (`}`)
	pub brace_r: Sprite,
	/// Less-than character (`<`)
	pub less_than: Sprite,
	/// Greater-than character (`>`)
	pub greater_than: Sprite,
	/// Minus character (`-`)
	pub minus: Sprite,
	/// Plus character (`+`)
	pub plus: Sprite,
	/// Slash character (`/`)
	pub slash: Sprite,
	/// Equals character (`=`)
	pub equals: Sprite,
	/// Underscore character (`_`)
	pub underscore: Sprite,
}

#[rustfmt::skip]
pub fn ascii_chars_spritesheet(id: SpritesheetId) -> AsciiCharsSpritesheet {
    AsciiCharsSpritesheet {
		space:            Sprite::new(id, Rect::from_xywh(  0, 0, 2, 6)),

        upper_a:          Sprite::new(id, Rect::from_xywh(  3, 0, 2, 6)),
        upper_b:          Sprite::new(id, Rect::from_xywh(  8, 0, 4, 6)),
        upper_c:          Sprite::new(id, Rect::from_xywh( 13, 0, 4, 6)),
        upper_d:          Sprite::new(id, Rect::from_xywh( 18, 0, 4, 6)),
        upper_e:          Sprite::new(id, Rect::from_xywh( 23, 0, 4, 6)),
        upper_f:          Sprite::new(id, Rect::from_xywh( 27, 0, 3, 6)),
        upper_g:          Sprite::new(id, Rect::from_xywh( 31, 0, 3, 6)),
        upper_h:          Sprite::new(id, Rect::from_xywh( 36, 0, 4, 6)),
        upper_i:          Sprite::new(id, Rect::from_xywh( 41, 0, 4, 6)),
        upper_j:          Sprite::new(id, Rect::from_xywh( 45, 0, 3, 6)),
        upper_k:          Sprite::new(id, Rect::from_xywh( 49, 0, 3, 6)),
        upper_l:          Sprite::new(id, Rect::from_xywh( 54, 0, 4, 6)),
        upper_m:          Sprite::new(id, Rect::from_xywh( 58, 0, 3, 6)),
        upper_n:          Sprite::new(id, Rect::from_xywh( 64, 0, 5, 6)),
        upper_o:          Sprite::new(id, Rect::from_xywh( 69, 0, 4, 6)),
        upper_p:          Sprite::new(id, Rect::from_xywh( 74, 0, 4, 6)),
        upper_q:          Sprite::new(id, Rect::from_xywh( 79, 0, 4, 6)),
        upper_r:          Sprite::new(id, Rect::from_xywh( 84, 0, 4, 6)),
        upper_s:          Sprite::new(id, Rect::from_xywh( 89, 0, 4, 6)),
        upper_t:          Sprite::new(id, Rect::from_xywh( 94, 0, 4, 6)),
        upper_u:          Sprite::new(id, Rect::from_xywh( 98, 0, 3, 6)),
        upper_v:          Sprite::new(id, Rect::from_xywh(103, 0, 4, 6)),
        upper_w:          Sprite::new(id, Rect::from_xywh(109, 0, 5, 6)),
        upper_x:          Sprite::new(id, Rect::from_xywh(115, 0, 5, 6)),
        upper_y:          Sprite::new(id, Rect::from_xywh(121, 0, 5, 6)),
        upper_z:          Sprite::new(id, Rect::from_xywh(126, 0, 4, 6)),

        lower_a:          Sprite::new(id, Rect::from_xywh(131, 0, 4, 6)),
        lower_b:          Sprite::new(id, Rect::from_xywh(136, 0, 4, 6)),
        lower_c:          Sprite::new(id, Rect::from_xywh(140, 0, 3, 6)),
        lower_d:          Sprite::new(id, Rect::from_xywh(144, 0, 3, 6)),
        lower_e:          Sprite::new(id, Rect::from_xywh(149, 0, 4, 6)),
        lower_f:          Sprite::new(id, Rect::from_xywh(153, 0, 3, 6)),
        lower_g:          Sprite::new(id, Rect::from_xywh(156, 0, 2, 6)),
        lower_h:          Sprite::new(id, Rect::from_xywh(160, 0, 3, 6)),
        lower_i:          Sprite::new(id, Rect::from_xywh(164, 0, 3, 6)),
        lower_j:          Sprite::new(id, Rect::from_xywh(166, 0, 1, 6)),
        lower_k:          Sprite::new(id, Rect::from_xywh(170, 0, 3, 6)),
        lower_l:          Sprite::new(id, Rect::from_xywh(174, 0, 3, 6)),
        lower_m:          Sprite::new(id, Rect::from_xywh(176, 0, 1, 6)),
        lower_n:          Sprite::new(id, Rect::from_xywh(182, 0, 5, 6)),
        lower_o:          Sprite::new(id, Rect::from_xywh(186, 0, 3, 6)),
        lower_p:          Sprite::new(id, Rect::from_xywh(190, 0, 3, 6)),
        lower_q:          Sprite::new(id, Rect::from_xywh(194, 0, 3, 6)),
        lower_r:          Sprite::new(id, Rect::from_xywh(198, 0, 3, 6)),
        lower_s:          Sprite::new(id, Rect::from_xywh(202, 0, 3, 6)),
        lower_t:          Sprite::new(id, Rect::from_xywh(206, 0, 3, 6)),
        lower_u:          Sprite::new(id, Rect::from_xywh(210, 0, 3, 6)),
        lower_v:          Sprite::new(id, Rect::from_xywh(214, 0, 3, 6)),
        lower_w:          Sprite::new(id, Rect::from_xywh(218, 0, 3, 6)),
        lower_x:          Sprite::new(id, Rect::from_xywh(224, 0, 5, 6)),
        lower_y:          Sprite::new(id, Rect::from_xywh(228, 0, 3, 6)),
        lower_z:          Sprite::new(id, Rect::from_xywh(232, 0, 3, 6)),

        digit_0:          Sprite::new(id, Rect::from_xywh(237, 0, 4, 6)),
        digit_1:          Sprite::new(id, Rect::from_xywh(242, 0, 4, 6)),
        digit_2:          Sprite::new(id, Rect::from_xywh(246, 0, 3, 6)),
        digit_3:          Sprite::new(id, Rect::from_xywh(251, 0, 4, 6)),
        digit_4:          Sprite::new(id, Rect::from_xywh(256, 0, 4, 6)),
        digit_5:          Sprite::new(id, Rect::from_xywh(261, 0, 4, 6)),
        digit_6:          Sprite::new(id, Rect::from_xywh(266, 0, 4, 6)),
        digit_7:          Sprite::new(id, Rect::from_xywh(271, 0, 4, 6)),
        digit_8:          Sprite::new(id, Rect::from_xywh(276, 0, 4, 6)),
        digit_9:          Sprite::new(id, Rect::from_xywh(281, 0, 4, 6)),

        exclamation_mark: Sprite::new(id, Rect::from_xywh(286, 0, 4, 6)),
        question_mark:    Sprite::new(id, Rect::from_xywh(288, 0, 1, 6)),
        colon:            Sprite::new(id, Rect::from_xywh(292, 0, 3, 6)),
        semicolon:        Sprite::new(id, Rect::from_xywh(294, 0, 1, 6)),
        comma:            Sprite::new(id, Rect::from_xywh(297, 0, 2, 6)),
        period:           Sprite::new(id, Rect::from_xywh(300, 0, 2, 6)),
        star:             Sprite::new(id, Rect::from_xywh(302, 0, 1, 6)),
        hashtag:          Sprite::new(id, Rect::from_xywh(306, 0, 3, 6)),
        single_quote:     Sprite::new(id, Rect::from_xywh(312, 0, 5, 6)),
        double_quote:     Sprite::new(id, Rect::from_xywh(314, 0, 1, 6)),
        bracket_l:        Sprite::new(id, Rect::from_xywh(318, 0, 3, 6)),
        bracket_r:        Sprite::new(id, Rect::from_xywh(321, 0, 2, 6)),
        parens_l:         Sprite::new(id, Rect::from_xywh(324, 0, 2, 6)),
        parens_r:         Sprite::new(id, Rect::from_xywh(327, 0, 2, 6)),
        brace_l:          Sprite::new(id, Rect::from_xywh(330, 0, 2, 6)),
        brace_r:          Sprite::new(id, Rect::from_xywh(334, 0, 3, 6)),
        less_than:        Sprite::new(id, Rect::from_xywh(338, 0, 3, 6)),
        greater_than:     Sprite::new(id, Rect::from_xywh(342, 0, 3, 6)),
        minus:            Sprite::new(id, Rect::from_xywh(346, 0, 3, 6)),
        plus:             Sprite::new(id, Rect::from_xywh(350, 0, 3, 6)),
        slash:            Sprite::new(id, Rect::from_xywh(354, 0, 3, 6)),
        equals:           Sprite::new(id, Rect::from_xywh(358, 0, 3, 6)),
        underscore:       Sprite::new(id, Rect::from_xywh(362, 0, 3, 6)),
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Snaeksheet {
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

	/// Play/pause button's play icon
	pub icon_play: Sprite,
	/// Play/pause button's pause icon
	pub icon_pause: Sprite,
	/// Restart button icon
	pub icon_restart: Sprite,

	/// Exclamation mark on the small number display
	pub num_bang: Sprite,
	/// Colon on the small number display
	pub num_colon: Sprite,
	/// digit zero on the small number display
	pub num_0: Sprite,
	/// digit one on the small number display
	pub num_1: Sprite,
	/// digit two on the small number display
	pub num_2: Sprite,
	/// digit three on the small number display
	pub num_3: Sprite,
	/// digit four on the small number display
	pub num_4: Sprite,
	/// digit five on the small number display
	pub num_5: Sprite,
	/// digit six on the small number display
	pub num_6: Sprite,
	/// digit seven on the small number display
	pub num_7: Sprite,
	/// digit eight on the small number display
	pub num_8: Sprite,
	/// digit nine on the small number display
	pub num_9: Sprite,

	/// digit placeholder on the big number display
	pub bignum_placeholder: Sprite,
	/// digit zero on the big number display
	pub bignum_0: Sprite,
	/// digit one on the big number display
	pub bignum_1: Sprite,
	/// digit two on the big number display
	pub bignum_2: Sprite,
	/// digit three on the big number display
	pub bignum_3: Sprite,
	/// digit four on the big number display
	pub bignum_4: Sprite,
	/// digit five on the big number display
	pub bignum_5: Sprite,
	/// digit six on the big number display
	pub bignum_6: Sprite,
	/// digit seven on the big number display
	pub bignum_7: Sprite,
	/// digit eight on the big number display
	pub bignum_8: Sprite,
	/// digit nine on the big number display
	pub bignum_9: Sprite,
}

#[rustfmt::skip]
pub fn snaeksheet(id: SpritesheetId) -> Snaeksheet {
    Snaeksheet {
        cursor:             Sprite::new(id, Rect::from_xywh( 24,   0,  4,  6)),

        snake_head:         Sprite::new(id, Rect::from_xywh( 14,   0,  7,  7)),
        snake_straight:     Sprite::new(id, Rect::from_xywh(  7,   0,  7,  7)),
        snake_gay:          Sprite::new(id, Rect::from_xywh(  0,   0,  7,  7)),
        snake_end:          Sprite::new(id, Rect::from_xywh(  0,   7,  7,  7)),
        snake_tongue:       Sprite::new(id, Rect::from_xywh( 21,   2,  3,  3)),

        banana_yellow:      Sprite::new(id, Rect::from_xywh(  7,   7,  7,  7)),
        banana_red:         Sprite::new(id, Rect::from_xywh( 14,   7,  7,  7)),
        banana_cyan:        Sprite::new(id, Rect::from_xywh( 21,   7,  7,  7)),

        box_playfield:      NineSlicingSprite::new(id, Rect::from_xywh(  9,  14,  9,  9),  4,  5,  4,  5),
        box_big_carved:     NineSlicingSprite::new(id, Rect::from_xywh( 18,  14,  5,  5),  2,  3,  2,  3),
        box_num_display:    NineSlicingSprite::new(id, Rect::from_xywh( 23,  14,  3,  3),  1,  2,  1,  2),
        box_text_input:     NineSlicingSprite::new(id, Rect::from_xywh( 26,  14,  3,  3),  1,  2,  1,  2),
        box_embossed:       NineSlicingSprite::new(id, Rect::from_xywh( 23,  17,  3,  3),  1,  2,  1,  2),
        box_carved:         NineSlicingSprite::new(id, Rect::from_xywh( 26,  17,  3,  3),  1,  2,  1,  2),
        box_green:          NineSlicingSprite::new(id, Rect::from_xywh( 23,  20,  3,  3),  1,  2,  1,  2),
        box_red:            NineSlicingSprite::new(id, Rect::from_xywh( 26,  20,  3,  3),  1,  2,  1,  2),

        carved_sep_line:    Sprite::new(id, Rect::from_xywh( 19,  20,  1,  2)),

        snaek_icon:         Sprite::new(id, Rect::from_xywh( 29,  15,  6,  6)),

        icon_minimize:      Sprite::new(id, Rect::from_xywh(  0,  14,  5,  1)),
        icon_close:         Sprite::new(id, Rect::from_xywh(  1,  16,  3,  3)),

        icon_play:          Sprite::new(id, Rect::from_xywh(  1,  19,  4,  4)),
        icon_pause:         Sprite::new(id, Rect::from_xywh(  5,  19,  4,  4)),
        icon_restart:       Sprite::new(id, Rect::from_xywh(  5,  15,  4,  4)),

        num_bang:           Sprite::new(id, Rect::from_xywh(  0,  23,  1,  5)),
        num_colon:          Sprite::new(id, Rect::from_xywh(  2,  23,  1,  5)),
        num_0:              Sprite::new(id, Rect::from_xywh(  4,  23,  3,  5)),
        num_1:              Sprite::new(id, Rect::from_xywh(  7,  23,  3,  5)),
        num_2:              Sprite::new(id, Rect::from_xywh( 10,  23,  3,  5)),
        num_3:              Sprite::new(id, Rect::from_xywh( 13,  23,  3,  5)),
        num_4:              Sprite::new(id, Rect::from_xywh( 16,  23,  3,  5)),
        num_5:              Sprite::new(id, Rect::from_xywh( 19,  23,  3,  5)),
        num_6:              Sprite::new(id, Rect::from_xywh( 22,  23,  3,  5)),
        num_7:              Sprite::new(id, Rect::from_xywh( 25,  23,  3,  5)),
        num_8:              Sprite::new(id, Rect::from_xywh( 28,  23,  3,  5)),
        num_9:              Sprite::new(id, Rect::from_xywh( 31,  23,  3,  5)),

        bignum_placeholder: Sprite::new(id, Rect::from_xywh( 28,   0,  8, 14)),
        bignum_0:           Sprite::new(id, Rect::from_xywh( 36,   0,  8, 14)),
        bignum_1:           Sprite::new(id, Rect::from_xywh( 44,   0,  8, 14)),
        bignum_2:           Sprite::new(id, Rect::from_xywh( 52,   0,  8, 14)),
        bignum_3:           Sprite::new(id, Rect::from_xywh( 60,   0,  8, 14)),
        bignum_4:           Sprite::new(id, Rect::from_xywh( 68,   0,  8, 14)),
        bignum_5:           Sprite::new(id, Rect::from_xywh( 36,  14,  8, 14)),
        bignum_6:           Sprite::new(id, Rect::from_xywh( 44,  14,  8, 14)),
        bignum_7:           Sprite::new(id, Rect::from_xywh( 52,  14,  8, 14)),
        bignum_8:           Sprite::new(id, Rect::from_xywh( 60,  14,  8, 14)),
        bignum_9:           Sprite::new(id, Rect::from_xywh( 68,  14,  8, 14)),
    }
}
