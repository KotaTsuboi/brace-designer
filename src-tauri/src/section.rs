use crate::unit::LengthUnit::*;
use crate::value::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub trait Section: Default {
    fn area(&self) -> Area;
    fn name(&self) -> String;
    fn new(name: String) -> Option<Self>;
}

#[derive(EnumIter, Default)]
pub enum AngleSteel {
    #[default]
    L80x80x6,
    L100x100x10,
}

impl AngleSteel {
    pub fn a(&self) -> Length {
        match self {
            Self::L80x80x6 => Length::new(80.0, MilliMeter),
            Self::L100x100x10 => Length::new(100.0, MilliMeter),
        }
    }

    pub fn b(&self) -> Length {
        match self {
            Self::L80x80x6 => Length::new(80.0, MilliMeter),
            Self::L100x100x10 => Length::new(100.0, MilliMeter),
        }
    }

    pub fn t(&self) -> Length {
        match self {
            Self::L80x80x6 => Length::new(6.0, MilliMeter),
            Self::L100x100x10 => Length::new(10.0, MilliMeter),
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
            Self::L80x80x6 => String::from("L-80x80x6"),
            Self::L100x100x10 => String::from("L-100x100x10"),
        }
    }

    fn new(name: String) -> Option<Self> {
        for section in Self::iter() {
            if section.name() == name {
                return Some(section);
            }
        }

        None
    }
}

#[derive(EnumIter, Default)]
pub enum ChannelSteel {
    #[default]
    C100x50x5x7_5,
}

impl ChannelSteel {
    fn h(&self) -> Length {
        match self {
            Self::C100x50x5x7_5 => Length::new(100.0, MilliMeter),
        }
    }

    fn b(&self) -> Length {
        match self {
            Self::C100x50x5x7_5 => Length::new(50.0, MilliMeter),
        }
    }

    fn t1(&self) -> Length {
        match self {
            Self::C100x50x5x7_5 => Length::new(5.0, MilliMeter),
        }
    }

    fn t2(&self) -> Length {
        match self {
            Self::C100x50x5x7_5 => Length::new(7.5, MilliMeter),
        }
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
            Self::C100x50x5x7_5 => String::from("[-100x50x5x7.5"),
        }
    }

    fn new(name: String) -> Option<Self> {
        for section in Self::iter() {
            if section.name() == name {
                return Some(section);
            }
        }

        None
    }
}
