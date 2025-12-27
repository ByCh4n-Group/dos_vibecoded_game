use tetra::graphics::{self, Color};
use tetra::graphics::text::{Font, Text};
use tetra::math::Vec2;
use tetra::Context;
use std::time::Duration;

pub struct BiosState {
    text: Text,
    timer: Duration,
    lines: Vec<String>,
    current_line_index: usize,
    memory_check: u32,
    finished: bool,
}

impl BiosState {
    pub fn new(ctx: &mut Context) -> tetra::Result<BiosState> {
        // Note: You need a font file at resources/DOS_font.ttf
        // If you don't have one, the game will crash.
        // You can download a free DOS font like "Perfect DOS VGA 437"
        let font = Font::vector(ctx, "./resources/DOS_font.ttf", 20.0)?; 
        let text = Text::new("", font);
        
        let lines = vec![
            "Award Modular BIOS v4.51PG, An Energy Star Ally".to_string(),
            "Copyright (C) 1984-98, Award Software, Inc.".to_string(),
            "".to_string(),
            "PENTIUM-MMX CPU at 233MHz".to_string(),
            "Memory Test: ".to_string(),
        ];

        Ok(BiosState {
            text,
            timer: Duration::from_secs(0),
            lines,
            current_line_index: 0,
            memory_check: 0,
            finished: false,
        })
    }

    pub fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.timer += tetra::time::get_delta_time(ctx);

        if self.current_line_index < self.lines.len() {
             if self.lines[self.current_line_index].starts_with("Memory Test:") {
                if self.memory_check < 65536 {
                    self.memory_check += 1024; // Fast memory check
                } else {
                    self.current_line_index += 1;
                    self.lines.push("".to_string());
                    self.lines.push("Award Plug and Play BIOS Extension v1.0A".to_string());
                    self.lines.push("Copyright (C) 1998, Award Software, Inc.".to_string());
                    self.lines.push("Detecting HDD Primary Master ... QUANTUM FIREBALL".to_string());
                    self.lines.push("Detecting HDD Primary Slave  ... None".to_string());
                    self.lines.push("Detecting HDD Secondary Master ... CD-ROM DRIVE".to_string());
                    self.lines.push("Detecting HDD Secondary Slave  ... None".to_string());
                    self.lines.push("".to_string());
                    self.lines.push("Press DEL to enter SETUP".to_string());
                    self.lines.push("06/15/1998-i440BX-8671-2A69KD4FC-00".to_string());
                }
             } else if self.timer.as_millis() > 200 {
                self.current_line_index += 1;
                self.timer = Duration::from_secs(0);
            }
        } else {
             if self.timer.as_secs() > 3 {
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
            
            let display_text = if line.starts_with("Memory Test:") {
                format!("Memory Test: {}K OK", self.memory_check)
            } else {
                line.clone()
            };

            self.text.set_content(display_text);
            self.text.draw(ctx, Vec2::new(10.0, y_offset));
            y_offset += 24.0;
        }

        Ok(())
    }
    
    pub fn is_finished(&self) -> bool {
        self.finished
    }
}
