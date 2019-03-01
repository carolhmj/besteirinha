extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use std::path::Path;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

extern crate spritesheet;
use spritesheet::{ Animation, Orientation };

fn main() {
	let opengl = OpenGL::V3_2;

    let mut window : Window = WindowSettings::new("hello world", 
    	[640, 480])
    	.exit_on_esc(true)
    	.opengl(opengl)
    	.build()
    	.unwrap();

    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());

    let mut sheet = spritesheet::SpriteSheet::new(Path::new("assets/Adventurer/adventurer-Sheet.png"));
    let anim = Animation::load_from_json("assets/Adventurer/adventurer.json");

    sheet.add_animations(&mut anim.unwrap());
    sheet.set_frame_size(50.0, 37.0);

    sheet.play("idle");

    let mut pos = (320.0, 480.0-37.0);
    let mut vel = (0.0, 1.0);

    sheet.set_pos(pos.0, pos.1, 0.0);

    while let Some(e) = events.next(&mut window) {
    	use graphics::*;

    	if let Some(args) = e.render_args() {
    		gl.draw(args.viewport(), |c, g| {
    			clear([0.0, 0.0, 0.0, 1.0], g);
    			sheet.render(c.transform, g, &args.ext_dt);
    		});
    	}
    	if let Some(update) = e.update_args() {
    		pos = (pos.0+vel.0, pos.1+vel.1);
            
            if pos.1 > 480.0-37.0 {
                pos.1 = 480.0-37.0;
            }
    		sheet.set_pos(pos.0, pos.1, 0.0);
    	};
    	if let Some(button) = e.press_args() {
    		if let Button::Keyboard(key) = button {
    			match key {
    				Key::Left => {
    					vel.0 = -1.0;
    					sheet.play("walk");
    					sheet.set_orientation_h(Orientation::Flipped);
    				},
                    Key::Right => {
                        vel.0 = 1.0;
                        sheet.play("walk");
                        sheet.set_orientation_h(Orientation::Normal);
                    },
                    Key::Up => {
                        vel.1 = -1.0;
                        sheet.play("jump");
                    },
    				_ => {}
    			}
    		}
    	};
    	if let Some(button) = e.release_args() {
            vel.0 = 0.0;
    		vel.1 = 1.0;
            sheet.cancel(None);
    		sheet.play("idle"); 
    	};
    }		
}
