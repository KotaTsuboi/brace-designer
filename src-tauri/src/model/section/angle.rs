use super::Polyline;
use super::Section;
use crate::unit::LengthUnit::*;
use crate::value::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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

    fn gauge(&self) -> Length {
        let unit = MilliMeter;

        match self {
            Self::L80x80x6 => Length::new(45.0, unit),
            Self::L100x100x10 => Length::new(55.0, unit),
        }
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

    fn shape_in_m(&self) -> Polyline {
        let unit = Meter;

        let a = self.a().get_value_in(unit);
        let b = self.b().get_value_in(unit);
        let t = self.t().get_value_in(unit);
        let g = self.gauge().get_value_in(unit);

        Polyline {
            start_point: (0.0, -g),
            next_points: vec![(a, -g), (a, t - g), (t, t - g), (t, b - g), (0.0, b - g)],
        }
    }

    fn gauge_list(&self) -> Vec<Length> {
        let unit = MilliMeter;

        match self {
            Self::L80x80x6 => vec![Length::new(0.0, unit)],
            Self::L100x100x10 => vec![Length::new(0.0, unit)],
        }
    }

    fn thickness(&self) -> Length {
        self.t()
    }

    fn breadth(&self) -> Length {
        self.a()
    }
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
