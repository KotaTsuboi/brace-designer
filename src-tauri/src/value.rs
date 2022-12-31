use crate::unit::*;
use std::cmp::Ordering;
use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Length {
    value: f64,
    unit: LengthUnit,
}

impl Length {
    pub const fn new(value: f64, unit: LengthUnit) -> Self {
        Self { value, unit }
    }

    pub fn get_value_in(&self, unit: LengthUnit) -> f64 {
        self.value * self.unit.rate() / unit.rate()
    }
}

impl ops::Add<Length> for Length {
    type Output = Length;

    fn add(self, rhs: Length) -> Self::Output {
        Length::new(
            self.get_value_in(self.unit) + rhs.get_value_in(self.unit),
            self.unit,
        )
    }
}

impl ops::Sub<Length> for Length {
    type Output = Length;

    fn sub(self, rhs: Length) -> Self::Output {
        Length::new(
            self.get_value_in(self.unit) - rhs.get_value_in(self.unit),
            self.unit,
        )
    }
}

impl ops::Mul<i32> for Length {
    type Output = Length;

    fn mul(self, rhs: i32) -> Self::Output {
        Length::new(self.get_value_in(self.unit) * rhs as f64, self.unit)
    }
}

impl ops::Mul<f64> for Length {
    type Output = Length;

    fn mul(self, rhs: f64) -> Self::Output {
        Length::new(self.get_value_in(self.unit) * rhs, self.unit)
    }
}

impl ops::Mul<Length> for Length {
    type Output = Area;

    fn mul(self, rhs: Length) -> Self::Output {
        let unit = LengthUnit::default();
        Area::new(self.get_value_in(unit) * rhs.get_value_in(unit), unit)
    }
}

impl PartialEq for Length {
    fn eq(&self, other: &Self) -> bool {
        let unit = LengthUnit::default();
        self.get_value_in(unit) == other.get_value_in(unit)
    }
}

impl Eq for Length {}

impl PartialOrd for Length {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let unit = LengthUnit::default();
        self.get_value_in(unit)
            .partial_cmp(&other.get_value_in(unit))
    }
}

impl Ord for Length {
    fn cmp(&self, other: &Self) -> Ordering {
        let unit = LengthUnit::default();
        self.get_value_in(unit)
            .partial_cmp(&other.get_value_in(unit))
            .unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct Area {
    m2: f64,
}

impl Area {
    const DIM: i32 = 2;

    pub fn new(a: f64, unit: LengthUnit) -> Area {
        if a < 0.0 {
            panic!("Area is negative");
        }

        Area {
            m2: a * unit.rate().powi(Self::DIM),
        }
    }

    pub fn get_value_in(&self, unit: LengthUnit) -> f64 {
        self.m2 / unit.rate().powi(Self::DIM)
    }
}

impl PartialEq for Area {
    fn eq(&self, other: &Self) -> bool {
        let unit = LengthUnit::default();
        self.get_value_in(unit) == other.get_value_in(unit)
    }
}

impl PartialOrd for Area {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let unit = LengthUnit::default();
        self.get_value_in(unit)
            .partial_cmp(&other.get_value_in(unit))
    }
}

impl ops::Mul<f64> for Area {
    type Output = Area;

    fn mul(self, rhs: f64) -> Self::Output {
        Area::new(self.m2 * rhs, LengthUnit::default())
    }
}

impl ops::Add<Area> for Area {
    type Output = Area;

    fn add(self, rhs: Area) -> Self::Output {
        Area::new(self.m2 + rhs.m2, LengthUnit::default())
    }
}

impl ops::Sub<Area> for Area {
    type Output = Area;

    fn sub(self, rhs: Area) -> Self::Output {
        Area::new(self.m2 - rhs.m2, LengthUnit::default())
    }
}

impl ops::Mul<Stress> for Area {
    type Output = Force;

    fn mul(self, rhs: Stress) -> Self::Output {
        Force {
            n: self.m2 * rhs.n_per_m2,
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct Force {
    n: f64,
}

impl Force {
    pub fn new(f: f64, unit: ForceUnit) -> Self {
        Force { n: f * unit.rate() }
    }

    pub fn get_value_in(&self, unit: ForceUnit) -> f64 {
        self.n / unit.rate()
    }
}

impl ops::Mul<f64> for Force {
    type Output = Force;

    fn mul(self, rhs: f64) -> Self::Output {
        Force { n: self.n * rhs }
    }
}

impl ops::Div<Area> for Force {
    type Output = Stress;

    fn div(self, rhs: Area) -> Self::Output {
        let s = self.n / rhs.m2;
        Stress::new(s, ForceUnit::default(), LengthUnit::default())
    }
}

impl ops::Div<Force> for Force {
    type Output = f64;

    fn div(self, rhs: Force) -> Self::Output {
        self.n / rhs.n
    }
}

pub struct Stress {
    n_per_m2: f64,
}

impl Stress {
    pub fn new(s: f64, fu: ForceUnit, lu: LengthUnit) -> Stress {
        Stress {
            n_per_m2: s * fu.rate() / lu.rate().powi(2),
        }
    }

    pub fn get_value_in(&self, fu: ForceUnit, lu: LengthUnit) -> f64 {
        self.n_per_m2 / fu.rate() * lu.rate().powi(2)
    }
}

impl ops::Div<Self> for Stress {
    type Output = f64;

    fn div(self, rhs: Self) -> Self::Output {
        self.n_per_m2 / rhs.n_per_m2
    }
}
