use tetra::graphics::{self, Color};
use tetra::graphics::text::{Font, Text};
use tetra::math::Vec2;
use tetra::Context;
use std::time::Duration;

pub struct BootState {
    text: Text,
    timer: Duration,
    lines: Vec<String>,
    current_line_index: usize,
    finished: bool,
}

impl BootState {
    pub fn new(ctx: &mut Context) -> tetra::Result<BootState> {
        let font = Font::vector(ctx, "./resources/DOS_font.ttf", 20.0)?;
        let text = Text::new("", font);
        
        let lines = vec![
            "Starting MS-DOS...".to_string(),
            "".to_string(),
            "HIMEM is testing extended memory...done.".to_string(),
            "C:\\>echo off".to_string(),
            "C:\\>PROMPT $p$g".to_string(),
            "C:\\>PATH C:\\DOS;C:\\WINDOWS".to_string(),
            "C:\\>SET TEMP=C:\\DOS".to_string(),
            "C:\\>MODE CON CODEPAGE PREPARE=((850) C:\\DOS\\EGA.CPI)".to_string(),
            "C:\\>MODE CON CODEPAGE SELECT=850".to_string(),
            "C:\\>KEYB UK,,C:\\DOS\\KEYBOARD.SYS".to_string(),
            "C:\\>DOSKEY".to_string(),
            "DOSKey installed.".to_string(),
            "".to_string(),
            "C:\\>vibecoded_game.exe".to_string(),
        ];

        Ok(BootState {
            text,
            timer: Duration::from_secs(0),
            lines,
            current_line_index: 0,
            finished: false,
        })
    }

    pub fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.timer += tetra::time::get_delta_time(ctx);

        if self.current_line_index < self.lines.len() {
            if self.timer.as_millis() > 150 {
                self.current_line_index += 1;
                self.timer = Duration::from_secs(0);
            }
        } else {
             if self.timer.as_secs() > 1 {
                 self.finished = true;
             }
        }

        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::BLACK);
        
        let mut y_offset = 10.0;
        for (i, line) in self.lines.iter().enumerate() {
            if i > self.current_line_index {
                break;
            }
            
            self.text.set_content(line);
            self.text.draw(ctx, Vec2::new(10.0, y_offset));
            y_offset += 24.0;
        }

        Ok(())
    }
    
    pub fn is_finished(&self) -> bool {
        self.finished
    }
}
