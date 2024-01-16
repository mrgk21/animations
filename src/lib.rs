use std::{thread::sleep, time::Duration};
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

pub struct AnimationConfig {
    points: u8,
    velocity: u32, // measured in renders per second
    total_time: i32,
    terminal_width: usize,
    resolution: usize, // divisions per width
}

impl Default for AnimationConfig {
    fn default() -> Self {
        AnimationConfig {
            velocity: 50,
            total_time: -1,
            points: 2,
            terminal_width: 100,
            resolution: 70,
        }
    }
}

pub trait Renderable {
    fn calc_point_location(&self, point: &Point, optional: Option<Vec<f32>>) -> Point;
    fn render(&self, init_location: &Vec<Point>, optional: Option<Vec<f32>>) -> Vec<Point>;
    fn width(&mut self, terminal_width: usize) -> &mut Self;
    fn resolution(&mut self, resolution: usize) -> &mut Self;
    fn velocity(&mut self, velocity: u32) -> &mut Self;
    fn points(&mut self, points: u8) -> &mut Self;
    fn total_time(&mut self, total_time: i32) -> &mut Self;
    fn config(&mut self, config: AnimationConfig) -> &mut Self;
    fn get_time_offset(&self) -> f32;
    fn get_point_offset(&self) -> f32;
}

pub trait Shape {
    fn new() -> Self;
    fn params(&mut self, params: Vec<f32>) -> &mut Self;
}

pub trait Field {
    fn print(&self, points: &Vec<Point>);
    fn start_animation(&self);
}

pub struct Ellipse {
    x_offset: f32,
    y_offset: f32,
    constant: f32,
    config: AnimationConfig,
}


impl Shape for Ellipse {
    fn new() -> Self {
        Ellipse {
            constant: 1.0,
            x_offset: 75.0,
            y_offset: 200.0,
            config: AnimationConfig::default(),
        }
    }

    fn params(&mut self, params: Vec<f32>) -> &mut Self {
        self.x_offset = *params.get(0).unwrap();
        self.y_offset = *params.get(1).unwrap();
        self.constant = *params.get(2).unwrap();
        self
    }

}

impl Renderable for Ellipse {

    fn width(&mut self, terminal_width: usize) -> &mut Self {
        self.config.terminal_width = terminal_width;
        self
     }

    fn velocity(&mut self, velocity: u32) -> &mut Self {
        self.config.velocity = velocity;
        self
    }

    fn resolution(&mut self, resolution: usize) -> &mut Self {
        if resolution > self.config.terminal_width {
            panic!("Resolution cannot be more than, terminal width: {}", self.config.terminal_width);
        }
        self.config.resolution = resolution;
        self
    }

    fn points(&mut self, points: u8) -> &mut Self {
        self.config.points = points;
        self
    }

    fn total_time(&mut self, total_time: i32) -> &mut Self {
        self.config.total_time = total_time;
        self
    }

    fn config(&mut self, config: AnimationConfig) -> &mut Self {
        self.config = config;
        self
    }
    
    fn get_time_offset(&self) -> f32 {
        1.0 / ((self.config.velocity) as f32)
    }

    fn get_point_offset(&self) -> f32 {
        ((self.config.terminal_width) as f32) / ((self.config.resolution) as f32)
    }

    // calculate the next position of the point (direction: 1 is l-t-r and 0 is r-t-l)
    fn calc_point_location(&self, point: &Point, optional: Option<Vec<f32>>) -> Point {

        let mut direction = point.direction;
        let reverse = if let Some(x) = optional {
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

        x = if x > self.x_offset {
            self.x_offset
        } else if x < 0.0 {
            0.0
        } else {
            x
        };

        let res = ((x/self.x_offset)).powi(2);
        let res = self.constant - res;
        let res = f32::sqrt(res); 
        let res = (1.0-(direction * res * reverse)) * self.y_offset;
        return Point {x_index: x, x_pos: res, direction};
    }

    // create the position vectors from calculations
    fn render(&self, init_location: &Vec<Point>, optional: Option<Vec<f32>>) -> Vec<Point> {
        let reverse = optional.unwrap().get(0).unwrap().clone();

        let last_point = self.calc_point_location(init_location.last().unwrap(), Some(vec![reverse]));
        let mut result_vec: Vec<Point> = init_location[1..].to_vec();
        result_vec.push(last_point);

        return result_vec;
    }

}


impl Field for Ellipse {

      // print out the position vectors
    fn print(&self, points: &Vec<Point>) {
        let width = self.config.terminal_width;
        let scaling_factor: f32 = width as f32 / (2.0 * self.y_offset);
        let mut buff: Vec<char> = vec![' '; width];

        for item in points.iter() {
            let norm_ind = item.x_pos.floor() * scaling_factor;
            let norm_ind = f32::floor(norm_ind) as usize;
            let norm_ind = if norm_ind >= width {
                width-1
            } else {
                norm_ind
            };

            buff[norm_ind] = '*';
        }

        let buff_str: String = buff.iter().collect();
        match stdout().flush()  {
            Ok(_) => println!("{}", buff_str),
            Err(_) => panic!("could not flush stdout error")
        };
    }

    fn start_animation(&self) {
        let mut duration_tracker: f32 = 0.0;
        let mut field_locations = vec![Point{x_index: 0.0, x_pos: 0.0, direction: 1.0}; self.config.points as usize];

        let time_offset = self.get_time_offset();
        let nspf = f32::floor(time_offset * 1_000_000_000 as f32)  as u32;

        let mut reverse: f32 = -1.0;

        let overshoot_factor = 0.00001 * 2.0 * self.y_offset;

        loop {
            if duration_tracker > self.config.total_time as f32 && self.config.total_time != -1{
                break;
            }

            let last_elem = field_locations.last().unwrap().x_pos;
            if last_elem >= (2.0 * self.y_offset) - overshoot_factor  || last_elem <= 0.0 + overshoot_factor {
                reverse = reverse * -1.0;
            }

            field_locations = self.render(&field_locations, Some(vec![reverse]));
            self.print(&field_locations);
            sleep(Duration::new(0, nspf));
            duration_tracker += time_offset;
        }
    }
}