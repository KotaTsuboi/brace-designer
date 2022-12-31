pub mod angle;
pub mod channel;
pub mod command;
pub mod ct;

use self::angle::AngleSteel;
use self::channel::ChannelSteel;
use self::ct::CTSteel;
use crate::value::*;
use serde::{Deserialize, Serialize};

pub trait Section: Send + Sync {
    fn area(&self) -> Area;
    fn name(&self) -> String;
    fn shape_in_m(&self) -> Polyline;
    fn gauge_list(&self) -> Vec<Length>;
    fn gauge_width(&self) -> Length {
        *self.gauge_list().first().unwrap() - *self.gauge_list().last().unwrap()
    }
    fn num_bolt_col(&self) -> u32 {
        self.gauge_list().len() as u32
    }
    fn breadth(&self) -> Length;
    fn thickness(&self) -> Length;
}

pub fn get_section(name: &str) -> Option<Box<dyn Section>> {
    if let Some(section) = CTSteel::from_str(name) {
        return Some(Box::new(section));
    }
    if let Some(section) = AngleSteel::new(name) {
        return Some(Box::new(section));
    }
    if let Some(section) = ChannelSteel::new(name) {
        return Some(Box::new(section));
    }
    None
}

#[derive(Serialize, Deserialize)]
pub struct Polyline {
    start_point: (f64, f64),
    next_points: Vec<(f64, f64)>,
}

impl Polyline {
    pub fn new(mut points: Vec<(f64, f64)>) -> Self {
        if points.len() < 2 {
            panic!("Length of points is less than 2: {}", points.len());
        }

        Polyline {
            start_point: points[0],
            next_points: points.drain(1..).collect(),
        }
    }
}
