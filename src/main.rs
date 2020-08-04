extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::collections::VecDeque;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateEvent, PressEvent};
use piston::{window::WindowSettings, Button, Key};
use graphics::{types, rectangle, clear};

#[derive(Clone, Copy, PartialEq)]
enum Dir { Left, Right, Up, Down }

pub struct App {
    gl: GlGraphics,
    bends: VecDeque<f64>,
    size: f64,
    dir: Dir,
    px: f64,
    py: f64
}

impl App {
    fn new(opengl: OpenGL) -> App {
        let mut vec = VecDeque::new();
        vec.push_back(50.0);
        App {
            gl: GlGraphics::new(opengl),
            bends: vec,
            size: 8.0,
            dir: Dir::Right,
            px: 50.0,
            py: 50.0
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        let size: f64 = self.size;

        let mut prev = (self.px, self.py, self.dir);
        let rects = self.bends.iter().rev().map(|p| -> types::Rectangle {
            let next = match prev.2 {
                Dir::Left | Dir::Right => (*p, prev.1, Dir::Down),
                Dir::Up | Dir::Down => (prev.0 , *p, Dir::Left)
            };
            let rect = get_rect((prev.0, prev.1), (next.0, next.1), size);
            prev = next;
            rect
        });

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            // Draw a box rotating around the middle of the screen.
            for rect in rects {
                rectangle(RED, rect, c.transform, gl);
            }
        });
    }

    fn move_by(&mut self, d: f64) {
        match self.dir {
            Dir::Left => self.px -= d,
            Dir::Up => self.py -= d,
            Dir::Right => self.px += d,
            Dir::Down => self.py += d
        };
    }

    fn change_dir(&mut self, to: Dir) {
        match to {
            Dir::Left | Dir:: Right => {
                if self.dir == Dir::Up || self.dir == Dir::Down {
                    self.bends.push_back(self.px);
                    self.dir = to;
                }
            },
            Dir::Up | Dir::Down => {
                if self.dir == Dir::Left || self.dir == Dir::Right {
                    self.bends.push_back(self.py);
                    self.dir = to;
                }
            }
        };
    }
}

fn get_rect(corner1: (f64, f64), corner2: (f64, f64), size: f64) -> types::Rectangle {
    let mut shift = (size, size);
    if corner1.0 > corner2.0 {
        shift.0 = -size;
    }
    if corner1.1 > corner2.1 {
        shift.1 = -size;
    }
    rectangle::rectangle_by_corners(corner1.0 - shift.0 / 2.0, corner1.1 - shift.1 / 2.0, corner2.0 + shift.0 / 2.0, corner2.1 + shift.1 / 2.0)
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
            app.change_dir(match key {
                Key::Left => Dir::Left,
                Key::Up => Dir::Up,
                Key::Right => Dir::Right,
                Key::Down => Dir::Down,
                _ => app.dir
            })
        };

    }
}