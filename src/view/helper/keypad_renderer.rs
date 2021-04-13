use crate::defines::layout_constants::{LINE_PADDING};
use crate::view::{FONTPATH1, FONTPATH2, FONTPATH3, FONTPATH4, FONTSIZE_KEYPAD, FONTSIZE_LINE};
use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{TextureCreator, TextureQuery, WindowCanvas},
    ttf::{Font, Sdl2TtfContext},
    video::WindowContext,
};
use std::result::Result;

pub struct KeypadRenderer {
    //keymap: HashMap<>
}

impl Default for KeypadRenderer {
    fn default() -> Self {
        KeypadRenderer::new()
    }
}

impl KeypadRenderer {
    pub fn new() -> KeypadRenderer {
        KeypadRenderer {}
    }

    pub fn draw_keypad(
        &mut self,
        _keys: Vec<String>,
        _x: i32,
        _y: i32,
        canvas: &mut WindowCanvas,
        ttf_context: &mut Sdl2TtfContext,
    ) -> Result<(), String> {
        let _font = ttf_context.load_font(FONTPATH3, FONTSIZE_KEYPAD).unwrap();
        let _texture_creator = canvas.texture_creator();

        Ok(())
    }

    fn draw_key(
        &mut self,
        character: &str,
        canvas: &mut WindowCanvas,
        font: &Font,
        texture_creator: &TextureCreator<WindowContext>,
        _text: &mut String,
    ) -> Result<(), String> {
        let surface = font.render(character).blended(Color::WHITE).unwrap();

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        let TextureQuery { width, height, .. } = texture.query();

        let target = Rect::new(LINE_PADDING, 2, width, height);

        canvas.copy(&texture, None, target)?;

        Ok(())
    }

    pub fn draw_lines(
        &mut self,
        _lines: &mut Vec<String>,
        _canvas: &mut WindowCanvas,
        _ttf_context: &mut Sdl2TtfContext,
    ) -> Result<(), String> {
        //self.draw_lines_with_x(lines, canvas, ttf_context, self.display_x)?;

        Ok(())
    }
}
