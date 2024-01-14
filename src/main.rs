use std::f32::consts::PI;

use animations::{ Parabola, Renderable, Point };

fn main() {
    let mut parabola = match Parabola::new(200, None) {
        Ok(x) => x,
        Err(_) => panic!("invalid parabola config"),
    };

    parabola.config_animation(25, 5, 100,100);

    parabola.start_animation();

}
