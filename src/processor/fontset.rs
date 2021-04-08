pub const FONTSET_LOW_SIZE: usize = 80;
pub const FONTSET_HIGH_START: usize = FONTSET_LOW_SIZE;
pub const FONTSET_HIGH_SIZE: usize = 160;

pub const FONTSET_LOW: [u8; FONTSET_LOW_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub const FONTSET_HIGH: [u8; FONTSET_HIGH_SIZE] = [
    0x3C, 0x7E, 0xE7, 0xC3, 0xC3, 0xC3, 0xC3, 0xE7,
	0x7E, 0x3C, 0x18, 0x38, 0x58, 0x18, 0x18, 0x18,
	0x18, 0x18, 0x18, 0x3C, 0x3E, 0x7F, 0xC3, 0x06,
	0x0C, 0x18, 0x30, 0x60, 0xFF, 0xFF, 0x3C, 0x7E,
	0xC3, 0x03, 0x0E, 0x0E, 0x03, 0xC3, 0x7E, 0x3C,
	0x06, 0x0E, 0x1E, 0x36, 0x66, 0xC6, 0xFF, 0xFF,
	0x06, 0x06, 0xFF, 0xFF, 0xC0, 0xC0, 0xFC, 0xFE,
	0x03, 0xC3, 0x7E, 0x3C, 0x3E, 0x7C, 0xC0, 0xC0,
	0xFC, 0xFE, 0xC3, 0xC3, 0x7E, 0x3C, 0xFF, 0xFF,
	0x03, 0x06, 0x0C, 0x18, 0x30, 0x60, 0x60, 0x60,
	0x3C, 0x7E, 0xC3, 0xC3, 0x7E, 0x7E, 0xC3, 0xC3,
	0x7E, 0x3C, 0x3C, 0x7E, 0xC3, 0xC3, 0x7F, 0x3F,
	0x03, 0x03, 0x3E, 0x7C, 0x00, 0x00, 0x00, 0x00,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];