
use animations::Ellipse;

fn main() {
    let ellipse = match Ellipse::new(200, None) {
        Ok(x) => x,
        Err(_) => panic!("invalid ellipse config"),
    };

    // ellipse.config_animation(50, 5, -1,150);

    ellipse.start_animation();

}
