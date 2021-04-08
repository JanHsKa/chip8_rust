
pub const MEMORYSIZE: usize = 4096;
pub const STACKSIZE: usize = 16;
pub const VARIABLES_COUNT: usize = 16;
pub const COLUMNS: usize = 64;
pub const ROWS: usize = 32;
pub const CARRY_FLAG: usize = 0xF;
pub const KEY_COUNT: usize = 16;
pub const MAX_PROGRAM_SIZE: usize = 3584;
pub const PROGRAM_START: usize = 0x200;
pub const SCALE: usize = 15;
pub const PROGRAM_STEP: usize = 2;
pub const GRAPHIC_SIZE: usize = COLUMNS * ROWS;
pub const GRAPHIC_SIZE_HIGH: usize = GRAPHIC_SIZE * 4;
pub const FLAG_REGISTER_SIZE: usize = 8;
pub const SPRITE_WIDTH: usize = 8;
pub const BIG_SPRITE: usize = 16;