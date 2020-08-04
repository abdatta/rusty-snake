extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateEvent, PressEvent};
use piston::{window::WindowSettings, Button, Key};

enum Dir { Left, Right, Up, Down }

pub struct App {
    gl: GlGraphics,
    dir: Dir,
    px: f64,
    py: f64
}

impl App {
    fn new(opengl: OpenGL) -> App {
        App {
            gl: GlGraphics::new(opengl),
            dir: Dir::Right,
            px: 50.0,
            py: 50.0
        }
    }
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const SIZE: f64 = 20.0;

        let square = rectangle::square(0.0, 0.0, SIZE);
        let (x, y) = (self.px, self.py);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c
                .transform
                .trans(x - SIZE / 2.0, y - SIZE / 2.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    fn move_by(&mut self, d: f64) {
        match self.dir {
            Dir::Left => self.px -= d,
            Dir::Up => self.py -= d,
            Dir::Right => self.px += d,
            Dir::Down => self.py += d
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [900, 500])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new(opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.move_by(args.dt * 100.0);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            app.dir = match key {
                Key::Left => Dir::Left,
                Key::Up => Dir::Up,
                Key::Right => Dir::Right,
                Key::Down => Dir::Down,
                _ => app.dir
            }
        };

    }
}