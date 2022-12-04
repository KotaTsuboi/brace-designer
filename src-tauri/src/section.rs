use crate::unit::LengthUnit::*;
use crate::value::*;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub trait Section: Send + Sync {
    fn area(&self) -> Area;
    fn name(&self) -> &str;
    fn shape_in_mm(&self) -> Polyline;
}

pub fn get_section(name: &str) -> Option<Box<dyn Section>> {
    if let Some(section) = AngleSteel::new(name) {
        return Some(Box::new(section));
    }
    if let Some(section) = ChannelSteel::new(name) {
        return Some(Box::new(section));
    }
    None
}

#[derive(EnumIter, Default)]
pub enum AngleSteel {
    #[default]
    L80x80x6,
    L100x100x10,
}

impl AngleSteel {
    pub fn new(name: &str) -> Option<Self> {
        Self::iter().find(|section| section.name() == name)
    }

    pub fn a(&self) -> Length {
        match self {
            Self::L80x80x6 => Length::new(80.0, &MilliMeter),
            Self::L100x100x10 => Length::new(100.0, &MilliMeter),
        }
    }

    pub fn b(&self) -> Length {
        match self {
            Self::L80x80x6 => Length::new(80.0, &MilliMeter),
            Self::L100x100x10 => Length::new(100.0, &MilliMeter),
        }
    }

    pub fn t(&self) -> Length {
        match self {
            Self::L80x80x6 => Length::new(6.0, &MilliMeter),
            Self::L100x100x10 => Length::new(10.0, &MilliMeter),
        }
    }
}

impl Section for AngleSteel {
    fn area(&self) -> Area {
        match self {
            Self::L80x80x6 => Area::new(9.327, &CentiMeter),
            Self::L100x100x10 => Area::new(19.00, &CentiMeter),
        }
    }

    fn name(&self) -> &str {
        match self {
            Self::L80x80x6 => "L-80x80x6",
            Self::L100x100x10 => "L-100x100x10",
        }
    }

    fn shape_in_mm(&self) -> Polyline {
        let unit = &MilliMeter;

        let a = self.a().get_value_in(unit);
        let b = self.b().get_value_in(unit);
        let t = self.t().get_value_in(unit);

        Polyline {
            start_point: (0.0, 0.0),
            next_points: vec![(a, 0.0), (a, t), (t, t), (t, b), (0.0, b)],
        }
    }
}

#[derive(EnumIter, Default)]
pub enum ChannelSteel {
    #[default]
    C100x50x5x7_5,
}

impl ChannelSteel {
    pub fn new(name: &str) -> Option<Self> {
        Self::iter().find(|section| section.name() == name)
    }

    fn h(&self) -> Length {
        match self {
            Self::C100x50x5x7_5 => Length::new(100.0, &MilliMeter),
        }
    }

    fn b(&self) -> Length {
        match self {
            Self::C100x50x5x7_5 => Length::new(50.0, &MilliMeter),
        }
    }

    fn t1(&self) -> Length {
        match self {
            Self::C100x50x5x7_5 => Length::new(5.0, &MilliMeter),
        }
    }

    fn t2(&self) -> Length {
        match self {
            Self::C100x50x5x7_5 => Length::new(7.5, &MilliMeter),
        }
    }
}

impl Section for ChannelSteel {
    fn area(&self) -> Area {
        match self {
            Self::C100x50x5x7_5 => Area::new(11.92, &CentiMeter),
        }
    }

    fn name(&self) -> &str {
        match self {
            Self::C100x50x5x7_5 => "[-100x50x5x7.5",
        }
    }

    fn shape_in_mm(&self) -> Polyline {
        let unit = &MilliMeter;

        let h = self.h().get_value_in(unit);
        let b = self.b().get_value_in(unit);
        let t1 = self.t1().get_value_in(unit);
        let t2 = self.t2().get_value_in(unit);

        Polyline {
            start_point: (0.0, 0.0),
            next_points: vec![
                (0.0, h / 2.0),
                (b, h / 2.0),
                (b, h / 2.0 - t2),
                (t1, h / 2.0 - t2),
                (t1, -h / 2.0 + t2),
                (b, -h / 2.0 + t2),
                (b, -h / 2.0),
                (0.0, -h / 2.0),
            ],
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Polyline {
    start_point: (f64, f64),
    next_points: Vec<(f64, f64)>,
}
