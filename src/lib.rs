use std::{f32::consts::PI, thread::sleep, time::Duration};
use std::io::{Write, stdout};

#[derive(Debug)]
pub struct Point {
    pub x_index: f32,
    pub x_pos: f32,
    direction: f32, // 1 or -1
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Point {
            x_index: self.x_index,
            x_pos: self.x_pos,
            direction: self.direction
        }
    }
}


struct AnimationConfig {
    points: u8,
    cps: u32,
    total_time: i32,
    terminal_width: usize
}

impl Default for AnimationConfig {
    fn default() -> Self {
        AnimationConfig {
            total_time: 100,
            cps: 10,
            points: 4,
            terminal_width: 100
        }
    }
}

impl Clone for AnimationConfig {
    fn clone(&self) -> Self {
        AnimationConfig {
            cps: self.cps,
            points: self.points,
            total_time: self.total_time,
            terminal_width: self.terminal_width,
        }
    }
}

pub trait Renderable {
    fn calc_point_location(&self, point: &Point, optional: Option<Vec<f32>>) -> Point;
    fn render(&self, init_location: &Vec<Point>, optional: Option<Vec<f32>>) -> Vec<Point>;
    fn print_field(&self, points: &Vec<Point>);
}

pub struct Parabola {
    x_offset: f32,
    y_offset: f32,
    constant: f32,
    config: AnimationConfig,
}

impl Parabola {
    pub fn new(terminal_width: usize, shape_config: Option<Vec<f32>>) -> Result<Parabola, String> {
        if let Some(params) = shape_config {
            if params.len() != 3 {
                return Err(String::from("invalid params"));
            }
            let mut par = Parabola {
                x_offset: *params.get(0).unwrap(),
                y_offset: *params.get(1).unwrap(),
                constant: *params.get(2).unwrap(),
                config: AnimationConfig{
                    terminal_width,
                    ..AnimationConfig::default()
                },
            };
            return Ok(par);
        }
        let mut par = Parabola {
            constant: 1.0,
            x_offset: 75.0,
            y_offset: 50.0,
            config: AnimationConfig::default(),
        };
        return Ok(par);
    }

    pub fn config_animation(&mut self, cps: u32, points: u8, total_time: i32, terminal_width: usize) {
        self.config = AnimationConfig {
            cps,
            points,
            total_time,
            terminal_width,
        };
    }

    fn get_time_offset(&self) -> f32 {
        1.0 / ((self.config.cps) as f32)
    }

    fn get_point_offset(&self) -> f32 {
        (self.config.terminal_width as f32) / ((self.config.cps + self.config.points as u32) as f32)
    }

    pub fn start_animation(&self) {
        let mut duration_tracker: f32 = 0.0;
        let mut field_locations = vec![Point{x_index: 0.0, x_pos: 0.0, direction: 1.0}; self.config.points as usize];

        let time_offset = self.get_time_offset();
        let nspf = f32::floor(time_offset * 1_000_000_000 as f32)  as u32;

        let mut reverse: f32 = -1.0;

        let overshoot_factor = 0.0000005 * 2.0 * self.y_offset;

        loop {
            if duration_tracker > self.config.total_time as f32 && self.config.total_time != -1{
                break;
            }

            let last_elem = field_locations.last().unwrap().x_pos;
            if last_elem >= (2.0 * self.y_offset) - overshoot_factor  || last_elem <= 0.0 + overshoot_factor {
                reverse = reverse * -1.0;
            }

            println!("reverse: {reverse}", );
            field_locations = self.render(&field_locations, Some(vec![reverse]));
            self.print_field(&field_locations);
            sleep(Duration::new(0, nspf));
            duration_tracker += time_offset;
        }
    }
}

impl Renderable for Parabola {
    // calculate the next position of the point (direction: 1 is l-t-r and 0 is r-t-l)
    fn calc_point_location(&self, point: &Point, optional: Option<Vec<f32>>) -> Point {

        let mut direction = point.direction;
        let mut reverse = if let Some(x) = optional {
            *x.get(0).unwrap()
        } else {
            1.0
        };

        let mut x = f32::from(point.x_index);
        
        if x >= self.x_offset {
            direction = -1.0;
        }
        
        if x <= 0.0 {
            direction = 1.0;
        }

        x += direction as f32 * self.get_point_offset();
        
        // parabola will be used later on
        // let res = (point.x_index + self.x_offset).powi(2);
        // let res = res / (-4.0 * self.constant);
        // let res = res - self.y_offset;

        let res = ((x/self.x_offset)).powi(2);
        let res = self.constant - res;
        let res = f32::sqrt(res); 
        let res = (1.0-(direction * res * reverse)) * self.y_offset;
        // println!("ip: {:?}, op: {:?}", point, Point {x_index: x, x_pos: res, direction});
        return Point {x_index: x, x_pos: res, direction};
    }

    // create the position vectors from calculations
    fn render(&self, init_location: &Vec<Point>, optional: Option<Vec<f32>>) -> Vec<Point> {
        let mut result_vec: Vec<Point> = vec![];

        let reverse = optional.unwrap().get(0).unwrap().clone();

        for item in init_location.iter() {
            let point = self.calc_point_location(item, Some(vec![reverse]));

            // println!("{:?}", point);
            result_vec.push(point);
        }

        return result_vec;
    }

    // print out the position vectors
    fn print_field(&self, points: &Vec<Point>) {
        let width = self.config.terminal_width;
        let scaling_factor: f32 = width as f32 / (2.0 * self.x_offset);
        let mut buff: Vec<char> = vec![' '; width];

        for item in points.iter() {
            let norm_ind = item.x_pos.floor() * scaling_factor;
            let norm_ind = f32::floor(norm_ind) as usize;
            let norm_ind = if norm_ind >= width {
                99
            } else {
                norm_ind
            };

            buff[norm_ind] = '*';

        }

        let buff_str: String = buff.iter().collect();

        println!("{}", buff_str);
        // print!("\r");
    }
}
