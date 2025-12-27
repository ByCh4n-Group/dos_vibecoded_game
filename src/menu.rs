use tetra::graphics::{self, Color, DrawParams};
use tetra::graphics::text::{Font, Text};
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::Rectangle;
use tetra::math::{Vec2, Vec3, Mat4};
use tetra::Context;
use tetra::input::{self, Key};
use std::time::Duration;

pub struct MenuState {
    title_text: Text,
    option_text: Text,
    selected_index: usize,
    options: Vec<String>,
    is_transitioning: bool,
    transition_timer: Duration,
    white_overlay: Mesh,
    pub should_start_game: bool,
}

impl MenuState {
    pub fn new(ctx: &mut Context) -> tetra::Result<MenuState> {
        let font = Font::vector(ctx, "./resources/DOS_font.ttf", 20.0)?;
        let title_text = Text::new("VIBECODED GAME MENU", font.clone());
        let option_text = Text::new("", font);
        
        let options = vec![
            "Start Game".to_string(),
            "Options".to_string(),
            "Credits".to_string(),
            "Exit to DOS".to_string(),
        ];

        let white_overlay = Mesh::rectangle(ctx, ShapeStyle::Fill, Rectangle::new(0.0, 0.0, 640.0, 480.0))?;

        Ok(MenuState {
            title_text,
            option_text,
            selected_index: 0,
            options,
            is_transitioning: false,
            transition_timer: Duration::from_secs(0),
            white_overlay,
            should_start_game: false,
        })
    }

    pub fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if self.is_transitioning {
            self.transition_timer += tetra::time::get_delta_time(ctx);
            if self.transition_timer.as_secs_f32() >= 2.5 {
                self.should_start_game = true;
            }
            return Ok(());
        }

        if input::is_key_pressed(ctx, Key::Up) {
            if self.selected_index > 0 {
                self.selected_index -= 1;
            } else {
                self.selected_index = self.options.len() - 1;
            }
        }
        
        if input::is_key_pressed(ctx, Key::Down) {
            if self.selected_index < self.options.len() - 1 {
                self.selected_index += 1;
            } else {
                self.selected_index = 0;
            }
        }

        if input::is_key_pressed(ctx, Key::Enter) {
            match self.selected_index {
                0 => { // Start Game
                    self.is_transitioning = true;
                },
                3 => { // Exit
                    std::process::exit(0);
                },
                _ => {}
            }
        }

        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // MS-DOS Blue Background
        graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.67)); // #0000AA
        
        // Calculate 3D transform for transition
        let mut scale_factor = 1.0;
        let center = Vec3::new(320.0, 240.0, 0.0);
        
        if self.is_transitioning {
            let t = self.transition_timer.as_secs_f32();
            // Exponential zoom effect
            scale_factor = 1.0 + t * t * t * 2.0; 
        }

        // Create a scaling matrix around the center
        // Translate to origin -> Scale -> Translate back
        let transform = Mat4::<f32>::translation_3d(center)
            * Mat4::scaling_3d(Vec3::new(scale_factor, scale_factor, 1.0))
            * Mat4::translation_3d(-center);

        // Helper closure to apply transform
        let apply_transform = |pos: Vec2<f32>| -> Vec2<f32> {
            let vec3_pos = Vec3::new(pos.x, pos.y, 0.0);
            let transformed = transform.mul_point(vec3_pos);
            Vec2::new(transformed.x, transformed.y)
        };

        // Title
        let title_width = self.title_text.get_bounds(ctx).unwrap().width;
        let title_pos = Vec2::new((640.0 - title_width) / 2.0, 50.0);
        let transformed_title_pos = apply_transform(title_pos);
        
        self.title_text.draw(ctx, DrawParams::new()
            .position(transformed_title_pos)
            .scale(Vec2::new(scale_factor, scale_factor))
        );
        
        // Options
        let mut y_offset = 150.0;
        for (i, option) in self.options.iter().enumerate() {
            let prefix = if i == self.selected_index { "> " } else { "  " };
            let content = format!("{}{}", prefix, option);
            
            self.option_text.set_content(content);
            
            let color = if i == self.selected_index {
                Color::rgb(1.0, 1.0, 0.0)
            } else {
                Color::WHITE
            };
            
            let option_pos = Vec2::new(200.0, y_offset);
            let transformed_option_pos = apply_transform(option_pos);

            self.option_text.draw(ctx, DrawParams::new()
                .position(transformed_option_pos)
                .color(color)
                .scale(Vec2::new(scale_factor, scale_factor))
            );
            
            y_offset += 30.0;
        }
        
        // Footer
        self.option_text.set_content("Use ARROW KEYS to select, ENTER to confirm");
        let footer_pos = Vec2::new(50.0, 400.0);
        let transformed_footer_pos = apply_transform(footer_pos);
        
        self.option_text.draw(ctx, DrawParams::new()
            .position(transformed_footer_pos)
            .scale(Vec2::new(scale_factor, scale_factor))
        );

        if self.is_transitioning {
            let alpha = (self.transition_timer.as_secs_f32() / 2.5).min(1.0);
            // Exposure ramp simulation: Draw white with increasing alpha
            self.white_overlay.draw(ctx, DrawParams::new().color(Color::rgba(1.0, 1.0, 1.0, alpha)));
        }

        Ok(())
    }
}
