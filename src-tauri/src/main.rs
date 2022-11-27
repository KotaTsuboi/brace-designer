#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Default)]
struct Brace<T>
where
    T: Section + Default,
{
    section: Mutex<T>,
}

trait Section: Default {
    fn area(&self) -> Area;
    fn name(&self) -> String;
    fn new(name: String) -> Option<Self>;
}

#[derive(EnumIter, Default)]
enum AngleSteel {
    #[default]
    L80x80x6,
    L100x100x10,
}

impl AngleSteel {
    fn a(&self) -> Length {
        match self {
            Self::L80x80x6 => Length::new(80.0, LengthUnit::MilliMeter),
            Self::L100x100x10 => Length::new(100.0, LengthUnit::MilliMeter),
        }
    }

    fn b(&self) -> Length {
        match self {
            Self::L80x80x6 => Length::new(80.0, LengthUnit::MilliMeter),
            Self::L100x100x10 => Length::new(100.0, LengthUnit::MilliMeter),
        }
    }

    fn t(&self) -> Length {
        match self {
            Self::L80x80x6 => Length::new(6.0, LengthUnit::MilliMeter),
            Self::L100x100x10 => Length::new(10.0, LengthUnit::MilliMeter),
        }
    }
}

impl Section for AngleSteel {
    fn area(&self) -> Area {
        match self {
            Self::L80x80x6 => Area::new(9.327, LengthUnit::CentiMeter),
            Self::L100x100x10 => Area::new(19.00, LengthUnit::CentiMeter),
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
enum ChannelSteel {
    #[default]
    C100x50x5x7_5,
}

impl ChannelSteel {
    fn h(&self) -> Length {
        match self {
            Self::C100x50x5x7_5 => Length::new(100.0, LengthUnit::MilliMeter),
        }
    }

    fn b(&self) -> Length {
        match self {
            Self::C100x50x5x7_5 => Length::new(50.0, LengthUnit::MilliMeter),
        }
    }

    fn t1(&self) -> Length {
        match self {
            Self::C100x50x5x7_5 => Length::new(5.0, LengthUnit::MilliMeter),
        }
    }

    fn t2(&self) -> Length {
        match self {
            Self::C100x50x5x7_5 => Length::new(7.5, LengthUnit::MilliMeter),
        }
    }
}

impl Section for ChannelSteel {
    fn area(&self) -> Area {
        match self {
            Self::C100x50x5x7_5 => Area::new(11.92, LengthUnit::CentiMeter),
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

enum LengthUnit {
    Meter,
    CentiMeter,
    MilliMeter,
}

impl LengthUnit {
    fn rate(&self) -> f64 {
        match self {
            LengthUnit::Meter => 1e+0,
            LengthUnit::CentiMeter => 1e-2,
            LengthUnit::MilliMeter => 1e-3,
        }
    }
}

struct Length {
    m: f64,
}

impl Length {
    const DIM: i32 = 1;

    fn new(l: f64, unit: LengthUnit) -> Self {
        Self {
            m: l * unit.rate().powi(Self::DIM),
        }
    }

    fn get_value_in(&self, unit: LengthUnit) -> f64 {
        self.m / unit.rate().powi(Self::DIM)
    }
}

struct Area {
    m2: f64,
}

impl Area {
    const DIM: i32 = 2;

    fn new(a: f64, unit: LengthUnit) -> Area {
        Area {
            m2: a * unit.rate().powi(Self::DIM),
        }
    }

    fn get_value_in(&self, unit: LengthUnit) -> f64 {
        self.m2 / unit.rate().powi(Self::DIM)
    }
}

#[tauri::command]
fn list_sections() -> Vec<String> {
    let mut list: Vec<String> = Vec::new();

    for section in AngleSteel::iter() {
        list.push(section.name());
    }

    for section in ChannelSteel::iter() {
        list.push(section.name());
    }

    list
}

#[tauri::command]
fn is_angle_section(name: String) -> bool {
    AngleSteel::new(name).is_some()
}

#[tauri::command]
fn set_angle_section(name: String, brace: tauri::State<Brace<AngleSteel>>) {
    let angle = AngleSteel::new(name).unwrap();
    let mut section = brace.section.lock().unwrap();
    *section = angle;
}

#[tauri::command]
fn get_angle_a_as_mm(brace: tauri::State<Brace<AngleSteel>>) -> f64 {
    brace
        .section
        .lock()
        .unwrap()
        .a()
        .get_value_in(LengthUnit::MilliMeter)
}

#[tauri::command]
fn get_angle_b_as_mm(brace: tauri::State<Brace<AngleSteel>>) -> f64 {
    brace
        .section
        .lock()
        .unwrap()
        .b()
        .get_value_in(LengthUnit::MilliMeter)
}

#[tauri::command]
fn get_angle_t_as_mm(brace: tauri::State<Brace<AngleSteel>>) -> f64 {
    brace
        .section
        .lock()
        .unwrap()
        .t()
        .get_value_in(LengthUnit::MilliMeter)
}

#[tauri::command]
fn is_channel_section(name: String) -> bool {
    ChannelSteel::new(name).is_some()
}

#[tauri::command]
fn set_channel_section(name: String, brace: tauri::State<Brace<ChannelSteel>>) {
    let channel = ChannelSteel::new(name).unwrap();
    let mut section = brace.section.lock().unwrap();
    *section = channel;
}

fn main() {
    tauri::Builder::default()
        .manage(Brace::<AngleSteel>::default())
        .invoke_handler(tauri::generate_handler![
            list_sections,
            is_angle_section,
            set_angle_section,
            get_angle_a_as_mm,
            get_angle_b_as_mm,
            get_angle_t_as_mm,
            is_channel_section,
            set_channel_section,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
