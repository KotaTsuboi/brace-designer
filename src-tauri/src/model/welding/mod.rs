use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::value::length::Length;

pub mod command;

#[derive(Default)]
pub struct Welding {
    welding_type: WeldingType,
    welding_length: Length,
    welding_size: Length,
}

#[derive(Default, EnumIter)]
pub enum WeldingType {
    #[default]
    I,
    II,
    III,
}

impl WeldingType {
    pub fn new(name: &str) -> Option<Self> {
        Self::iter().find(|section| section.name() == name)
    }

    fn name(&self) -> String {
        match self {
            Self::I => "I".to_string(),
            Self::II => "II".to_string(),
            Self::III => "III".to_string(),
        }
    }
}
