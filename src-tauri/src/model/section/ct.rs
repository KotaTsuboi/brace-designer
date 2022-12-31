use super::Polyline;
use super::Section;
use crate::unit::LengthUnit::*;
use crate::value::*;

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

    fn shape_in_m(&self) -> Polyline {
        let unit = Meter;

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

    fn breadth(&self) -> Length {
        self.b
    }
}
