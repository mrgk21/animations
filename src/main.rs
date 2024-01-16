
use animations::{Ellipse, Shape, Field, Renderable};

fn main() {
    let mut ellipse = Ellipse::new();
    ellipse
        .velocity(5) 
        .resolution(10)
        .points(2);

    ellipse.start_animation();

}
