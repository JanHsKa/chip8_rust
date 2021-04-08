use crate::display::{
    FONTPATH1, FONTPATH2, 
    FONTPATH3, FONTPATH4, 
    FONTSIZE_LINE, FONTSIZE_KEYPAD,
    layout_constants::{
        LINE_PADDING, HIGHLIGHT_PADDING}
};
use std::result::Result;
use sdl2::{
    rect::Rect, ttf::{Sdl2TtfContext, Font}, 
    render::{TextureQuery, TextureCreator, WindowCanvas}, 
    pixels::Color, video::WindowContext};


pub struct KeypadRenderer {
    //keymap: HashMap<>
}

impl KeypadRenderer {
    pub fn new() -> KeypadRenderer {
        KeypadRenderer {

        }
    }

    pub fn draw_keypad(&mut self, keys: Vec<String>, x: i32, y: i32, 
        canvas: &mut WindowCanvas, ttf_context: &mut Sdl2TtfContext) -> Result<(), String> {

        let font = ttf_context.load_font(FONTPATH3, FONTSIZE_KEYPAD).unwrap();
        let texture_creator = canvas.texture_creator();

        Ok(())
    }

    fn draw_key(&mut self, character: &str, canvas: &mut WindowCanvas, font: &Font,
        texture_creator: &TextureCreator<WindowContext>, text: &mut String) -> Result<(), String> {

        let surface = font
            .render(character)
            .blended(Color::WHITE)
            .unwrap();

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        let TextureQuery { width, height, .. } = texture.query();
        
        let target = Rect::new(
            LINE_PADDING,
            2,
            width,
            height,
        );
        
        canvas.copy(&texture, None, target)?;

        Ok(())
    }

    pub fn draw_lines(&mut self, lines: &mut Vec<String>,
        canvas: &mut WindowCanvas, ttf_context: &mut Sdl2TtfContext) -> Result<(), String> {

        //self.draw_lines_with_x(lines, canvas, ttf_context, self.display_x)?;

        Ok(())
    }
}