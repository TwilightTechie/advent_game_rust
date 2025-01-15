extern crate defender;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::event_loop::*;
use piston::input::*;

use defender::App;
use defender::config::GraphicsConfig;

fn main() {
    //setting the resolution
    let mut app = App::new(GraphicsConfig::new("Defender", 960, 786));

    //pool for event from the window
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut app.window.settings) {
        //handle keyboard input
        if let Some(i) = e.press_args() {
            app.input(&i, true);
        }
        if let Some(i) = e.release_args() {
            app.input(&i, false);
        }
        //handle rendering
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
        //handle any updates
        if let Some(u) = e.update_args() {
            app.update(u);
        }
    }
}
