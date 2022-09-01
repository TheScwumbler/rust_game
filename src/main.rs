extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics,
    rotation: f64,

    wx: f64,
    wy: f64,

    x: f64,
    y: f64,
    len: f64,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 100.0);
        let rotation = self.rotation;

        self.wx = args.window_size[0] / 2.0;
        self.wy = args.window_size[1] / 2.0;
        let (x, y) = (self.wx + self.x, self.wy + self.y);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // rotate 2 radians pers econd
        self.rotation += 2.0 * args.dt;

        if self.rotation > 360.0 {
            self.rotation -= 360.0;
        }

        // move square 10 pixels per second 
        let (x, y) = (self.x, self.y);

        self.x += 15.0 * args.dt;
        self.y += 15.0 * args.dt;
    }
}

fn main() {

    let opengl = OpenGL::V3_2;

    let mut win: Window = WindowSettings::new("Square (ball)", [800, 600])
        .graphics_api(opengl)
        .exit_on_esc(true) // change later
        .build()
        .unwrap();
        
    let mut app: App = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        wx: 0.0, wy: 0.0,
        x: 0.0, y: 0.0, 
        len: 50.0,
    };

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut win) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        else if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }

}
