
use animations::{Ellipse, Shape, Field, Renderable};

fn main() {
    let mut ellipse = Ellipse::new();
    ellipse
        .velocity(100) 
        .resolution(50)
        .points(5);

    ellipse.start_animation();

}
