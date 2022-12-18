use crate::unit::LengthUnit::*;
use crate::value::*;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub trait Section: Send + Sync {
    fn area(&self) -> Area;
    fn name(&self) -> String;
    fn shape_in_mm(&self) -> Polyline;
    fn gauge_list(&self) -> Vec<Length>;
    fn num_bolt_col(&self) -> u32 {
        self.gauge_list().len() as u32
    }
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

pub struct CTSteel {
    h: Length,
    b: Length,
    tw: Length,
    tf: Length,
    r: Length,
}

impl Default for CTSteel {
    fn default() -> Self {
        Self::CT_100X200X8X12
    }
}

impl CTSteel {
    const CT_100X200X8X12: Self = Self::new(100.0, 200.0, 8.0, 12.0, 13.0);
    const CT_125X250X9X14: Self = Self::new(125.0, 250.0, 9.0, 14.0, 13.0);

    const fn new(h: f64, b: f64, tw: f64, tf: f64, r: f64) -> Self {
        Self {
            h: Length::new(h, MilliMeter),
            b: Length::new(b, MilliMeter),
            tw: Length::new(tw, MilliMeter),
            tf: Length::new(tf, MilliMeter),
            r: Length::new(r, MilliMeter),
        }
    }

    pub fn from_str(name: &str) -> Option<Self> {
        Self::iter().find(|section| section.name() == name)
    }

    pub fn iter() -> <Vec<Self> as IntoIterator>::IntoIter {
        vec![Self::CT_100X200X8X12, Self::CT_125X250X9X14].into_iter()
    }
}

impl Section for CTSteel {
    fn name(&self) -> String {
        let unit = MilliMeter;
        let name = format!(
            "CT-{}x{}x{}x{}",
            self.h.get_value_in(unit),
            self.b.get_value_in(unit),
            self.tw.get_value_in(unit),
            self.tf.get_value_in(unit),
        );
        name
    }

    fn area(&self) -> Area {
        self.h * self.tf + (self.b - self.tf) * self.tw
    }

    fn gauge_list(&self) -> Vec<Length> {
        let b = self.b.get_value_in(MilliMeter);

        match b {
            100.0 => {
                return vec![
                    Length::new(30.0, MilliMeter),
                    Length::new(-30.0, MilliMeter),
                ]
            }
            125.0 => {
                return vec![
                    Length::new(37.5, MilliMeter),
                    Length::new(-37.5, MilliMeter),
                ]
            }
            150.0 => {
                return vec![
                    Length::new(45.0, MilliMeter),
                    Length::new(-45.0, MilliMeter),
                ]
            }
            175.0 => {
                return vec![
                    Length::new(52.5, MilliMeter),
                    Length::new(-52.5, MilliMeter),
                ]
            }
            200.0 => {
                return vec![
                    Length::new(60.0, MilliMeter),
                    Length::new(-60.0, MilliMeter),
                ]
            }
            250.0 => {
                return vec![
                    Length::new(75.0, MilliMeter),
                    Length::new(-75.0, MilliMeter),
                ]
            }
            300.0 => {
                return vec![
                    Length::new(115.0, MilliMeter),
                    Length::new(75.0, MilliMeter),
                    Length::new(-75.0, MilliMeter),
                    Length::new(-115.0, MilliMeter),
                ]
            }
            350.0 => {
                return vec![
                    Length::new(140.0, MilliMeter),
                    Length::new(70.0, MilliMeter),
                    Length::new(-70.0, MilliMeter),
                    Length::new(-140.0, MilliMeter),
                ]
            }
            400.0 => {
                return vec![
                    Length::new(160.0, MilliMeter),
                    Length::new(70.0, MilliMeter),
                    Length::new(-70.0, MilliMeter),
                    Length::new(-160.0, MilliMeter),
                ]
            }
            _ => panic!("B = {} is invalid", b),
        }
    }

    fn shape_in_mm(&self) -> Polyline {
        let unit = MilliMeter;

        let h = self.h.get_value_in(unit);
        let b = self.b.get_value_in(unit);
        let tw = self.tw.get_value_in(unit);
        let tf = self.tf.get_value_in(unit);

        Polyline {
            start_point: (0.0, b / 2.0),
            next_points: vec![
                (tf, b / 2.0),
                (tf, tw / 2.0),
                (h, tw / 2.0),
                (h, -tw / 2.0),
                (tf, -tw / 2.0),
                (tf, -b / 2.0),
                (0.0, -b / 2.0),
            ],
        }
    }

    fn thickness(&self) -> Length {
        self.tf
    }
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

    fn a_mm(&self) -> f64 {
        match self {
            Self::L80x80x6 => 80.0,
            Self::L100x100x10 => 100.0,
        }
    }

    fn b_mm(&self) -> f64 {
        match self {
            Self::L80x80x6 => 80.0,
            Self::L100x100x10 => 100.0,
        }
    }

    fn t_mm(&self) -> f64 {
        match self {
            Self::L80x80x6 => 6.0,
            Self::L100x100x10 => 10.0,
        }
    }

    fn a(&self) -> Length {
        Length::new(self.a_mm(), MilliMeter)
    }

    fn b(&self) -> Length {
        Length::new(self.b_mm(), MilliMeter)
    }

    fn t(&self) -> Length {
        Length::new(self.t_mm(), MilliMeter)
    }
}

impl Section for AngleSteel {
    fn area(&self) -> Area {
        match self {
            Self::L80x80x6 => Area::new(9.327, CentiMeter),
            Self::L100x100x10 => Area::new(19.00, CentiMeter),
        }
    }

    fn name(&self) -> String {
        match self {
            Self::L80x80x6 => "L-80x80x6".to_string(),
            Self::L100x100x10 => "L-100x100x10".to_string(),
        }
    }

    fn shape_in_mm(&self) -> Polyline {
        let unit = MilliMeter;

        let a = self.a().get_value_in(unit);
        let b = self.b().get_value_in(unit);
        let t = self.t().get_value_in(unit);

        Polyline {
            start_point: (0.0, 0.0),
            next_points: vec![(a, 0.0), (a, t), (t, t), (t, b), (0.0, b)],
        }
    }

    fn gauge_list(&self) -> Vec<Length> {
        let unit = MilliMeter;

        match self {
            Self::L80x80x6 => vec![Length::new(45.0, unit)],
            Self::L100x100x10 => vec![Length::new(55.0, unit)],
        }
    }

    fn thickness(&self) -> Length {
        self.t()
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

    fn h_mm(&self) -> f64 {
        match self {
            Self::C100x50x5x7_5 => 100.0,
        }
    }

    fn b_mm(&self) -> f64 {
        match self {
            Self::C100x50x5x7_5 => 50.0,
        }
    }

    fn t1_mm(&self) -> f64 {
        match self {
            Self::C100x50x5x7_5 => 5.0,
        }
    }

    fn t2_mm(&self) -> f64 {
        match self {
            Self::C100x50x5x7_5 => 7.5,
        }
    }

    fn h(&self) -> Length {
        Length::new(self.h_mm(), MilliMeter)
    }

    fn b(&self) -> Length {
        Length::new(self.b_mm(), MilliMeter)
    }

    fn t1(&self) -> Length {
        Length::new(self.t1_mm(), MilliMeter)
    }

    fn t2(&self) -> Length {
        Length::new(self.t2_mm(), MilliMeter)
    }
}

impl Section for ChannelSteel {
    fn area(&self) -> Area {
        match self {
            Self::C100x50x5x7_5 => Area::new(11.92, CentiMeter),
        }
    }

    fn name(&self) -> String {
        match self {
            Self::C100x50x5x7_5 => "[-100x50x5x7.5".to_string(),
        }
    }

    fn shape_in_mm(&self) -> Polyline {
        let unit = MilliMeter;

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

    fn gauge_list(&self) -> Vec<Length> {
        let unit = MilliMeter;

        match self {
            Self::C100x50x5x7_5 => vec![Length::new(0.0, unit)],
        }
    }

    fn thickness(&self) -> Length {
        self.t1()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Polyline {
    start_point: (f64, f64),
    next_points: Vec<(f64, f64)>,
}

#[test]
fn test_angle_gauge() {
    let data: Vec<(AngleSteel, f64)> = vec![
        (AngleSteel::L80x80x6, 45.0),
        (AngleSteel::L100x100x10, 55.0),
    ];

    for tuple in data {
        let actual = tuple.0.gauge_list();
        let expected = vec![Length::new(tuple.1, MilliMeter)];
        assert_eq!(actual, expected);
    }
}

#[test]
fn test_channel_gauge() {
    let data: Vec<(ChannelSteel, Option<f64>)> = vec![(ChannelSteel::C100x50x5x7_5, None)];

    for tuple in data {
        let actual = tuple.0.gauge_list();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}
