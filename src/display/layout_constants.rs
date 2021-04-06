use sdl2::pixels::Color;
use crate::processor::memory_constants;
use crate::lazy_static;


lazy_static! {
    pub static ref WINDOW_BACKGROUND: Color = Color::RGB(25,35,45); 
    pub static ref GAME_BACKGROUND: Color = Color::RGB(40, 40, 40); 
    pub static ref GAME_FOREGROUND: Color = Color::RGB(170, 255, 170);
    pub static ref DARK_OUTLINE: Color = Color::RGB(5, 10, 15);
    pub static ref BRIGHT_OUTLINE: Color = Color::RGB(60, 80, 90);
    pub static ref GAME_PIXEL_SET: Color = Color::RGB(170, 255, 170);
    pub static ref GAME_PIXEL_UNSET: Color = Color::RGB(40, 40, 40);
}

//original 15
//low scale 9
pub const PIXEL_SCALE: usize = 15;
pub const EDGE_SIZE: i32 = 12;
pub const OUTLINE: i32 = 4;
pub const PADDING: i32 = 4;
//original 1350 / 850
//low scale 800 / 550
pub const WINDOW_WIDTH: u32 = 1350;
pub const WINDOW_HEIGHT: u32 = 890;

pub const GAME_WIDTH: u32 = (memory_constants::COLUMNS * PIXEL_SCALE) as u32;
pub const GAME_HEIGHT: u32 = (memory_constants::ROWS * PIXEL_SCALE) as u32;
pub const GAME_START_X: i32 = EDGE_SIZE + OUTLINE;
pub const GAME_START_Y: i32 = EDGE_SIZE + OUTLINE;

pub const MEMORY_HEIGHT: u32 = WINDOW_HEIGHT - 3 * EDGE_SIZE as u32 - 4 * OUTLINE as u32 - GAME_HEIGHT;
pub const MEMORY_WIDTH: u32 = OPCODE_WIDTH;
pub const MEMORY_START_X: i32 = OPCODE_START_X;
pub const MEMORY_START_Y: i32 = 2 * EDGE_SIZE + 3 * OUTLINE + GAME_HEIGHT as i32;

pub const INFO_HEIGHT: u32 = MEMORY_HEIGHT;
pub const INFO_WIDTH: u32 = MEMORY_WIDTH;
pub const INFO_START_X: i32 = GAME_START_X;
pub const INFO_START_Y: i32 = MEMORY_START_Y;

pub const OPCODE_HEIGHT: u32 = GAME_HEIGHT;
pub const OPCODE_WIDTH: u32 = WINDOW_WIDTH - 3 * EDGE_SIZE as u32 - 4 * OUTLINE as u32 - GAME_WIDTH;
pub const OPCODE_START_X: i32 = EDGE_SIZE * 2 + OUTLINE * 3 + GAME_WIDTH as i32;
pub const OPCODE_START_Y: i32 = GAME_START_Y;

pub const STACK_HEIGHT: u32 = MEMORY_HEIGHT;
pub const STACK_WIDTH: u32 = MEMORY_WIDTH / 2 + 3 * EDGE_SIZE as u32;
// WINDOW_WIDTH - INFO_WIDTH - MEMORY_WIDTH - 5 * EDGE_SIZE as u32 - 6 * OUTLINE as u32;
pub const STACK_START_X: i32 = MEMORY_START_X - STACK_WIDTH as i32 - EDGE_SIZE - 2 * OUTLINE;
pub const STACK_START_Y: i32 = MEMORY_START_Y;






