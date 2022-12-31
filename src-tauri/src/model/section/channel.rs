use super::Polyline;
use super::Section;
use crate::unit::LengthUnit::*;
use crate::value::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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

    fn shape_in_m(&self) -> Polyline {
        let unit = Meter;

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

    fn breadth(&self) -> Length {
        self.h()
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
