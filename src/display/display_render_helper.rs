use crate::display::{
    FONTPATH1, FONTPATH2, 
    FONTPATH3, FONTPATH4, FONTSIZE,
    layout_constants::{
        LINE_PADDING, HIGHLIGHT_PADDING}
};
use std::result::Result;
use sdl2::{
    rect::Rect, ttf::{Sdl2TtfContext, Font}, 
    render::{TextureQuery, TextureCreator, WindowCanvas}, 
    pixels::Color, video::WindowContext};

pub struct DisplayRenderHelper {
    display_x: i32,
    display_y: i32,
    display_width: u32,
    display_heigth: u32,
}

impl DisplayRenderHelper {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> DisplayRenderHelper {
        DisplayRenderHelper {
            display_x: x,
            display_y: y,
            display_width: width,
            display_heigth: height,
        }
    }

    pub fn set_x(&mut self, x: i32) {
        self.display_x = x;
    }

    pub fn draw_rectangle(&mut self, canvas: &mut WindowCanvas, y: i32, color: Color) -> Result<(), String> {
        let rectangle = self.get_rectangle(y);
        canvas.set_draw_color(color);
        canvas.draw_rect(rectangle)?;

        Ok(())
    }

    pub fn fill_rectangle(&mut self, canvas: &mut WindowCanvas, y: i32, color: Color) -> Result<(), String> {
        let rectangle = self.get_rectangle(y);
        canvas.set_draw_color(color);
        canvas.fill_rect(rectangle)?;

        Ok(())
    }

    fn get_rectangle(&mut self, y: i32) -> Rect{
        Rect::new(
            self.display_x,
            self.display_y + HIGHLIGHT_PADDING + y * (FONTSIZE as i32 + LINE_PADDING),
             self.display_width, 
             2 * (LINE_PADDING / 2) as u32 + FONTSIZE as u32)
    }

    pub fn draw_lines(&mut self, lines: &mut Vec<String>,
        canvas: &mut WindowCanvas, ttf_context: &mut Sdl2TtfContext) -> Result<(), String> {

        self.draw_lines_with_x(lines, canvas, ttf_context, self.display_x)?;

        Ok(())
    }

    pub fn draw_lines_with_x(&mut self, lines: &mut Vec<String>,
        canvas: &mut WindowCanvas, ttf_context: &mut Sdl2TtfContext, x: i32) -> Result<(), String> {

        let font = ttf_context.load_font(FONTPATH3, FONTSIZE).unwrap();
        //font.set_style(sdl2::ttf::FontStyle::BOLD);

        let texture_creator = canvas.texture_creator();
        for (i, iter) in lines.iter_mut().enumerate() {
            self.render_line(canvas, &font, &texture_creator, iter, i, x)?;
        }

        Ok(())
    }

    fn render_line(&mut self, canvas: &mut WindowCanvas, font: &Font,
        texture_creator: &TextureCreator<WindowContext>, text: &mut String, row: usize, x: i32) -> Result<(), String> {

        let surface = font
            .render((*text).as_str())
            .blended(Color::WHITE)
            .unwrap();

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
    
        let TextureQuery { width, height, .. } = texture.query();
    
        let target = Rect::new(
            x + LINE_PADDING,
            self.display_y + LINE_PADDING + ((FONTSIZE + LINE_PADDING as u16) * row as u16) as i32,
            width,
            height,
        );
    
        canvas.copy(&texture, None, target)?;

        Ok(())
    }
}