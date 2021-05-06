use crate::constants::DEF_COLOR;

/// Button Styles
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Current,
    Transparent,
    Light,
    Dark,
    Gray(u16),
    Red(u16),
    Yellow(u16),
    Green(u16),
    Blue(u16),
    Indigo(u16),
    Purple(u16),
    Pink(u16),
}

impl Color {
    pub const ALL: [Color; 12] = [
        Color::Current,
        Color::Transparent,
        Color::Light,
        Color::Dark,
        Color::Gray(DEF_COLOR),
        Color::Red(DEF_COLOR),
        Color::Yellow(DEF_COLOR),
        Color::Green(DEF_COLOR),
        Color::Blue(DEF_COLOR),
        Color::Indigo(DEF_COLOR),
        Color::Purple(DEF_COLOR),
        Color::Pink(DEF_COLOR),
    ];

    pub fn is_light(&self) -> bool {
        use Color::*;

        match self {
            Current | Transparent | Light => true,
            Dark => false,
            Gray(var) | Red(var) | Yellow(var) | Green(var) |
            Blue(var) | Indigo(var) | Purple(var) | Pink(var) 
                => if *var <= 300 { true } else { false }
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::Current
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        use Color::*;

        match self {
            Current => "current".to_owned(),
            Transparent => "transparent".to_owned(),
            Light => "white".to_owned(),
            Dark => "black".to_owned(),
            Gray(var) => format!("gray-{}", var),
            Red(var) => format!("red-{}", var),
            Yellow(var) => format!("yellow-{}", var),
            Green(var) => format!("green-{}", var),
            Blue(var) => format!("blue-{}", var),
            Indigo(var) => format!("indigo-{}", var),
            Purple(var) => format!("purple-{}", var),
            Pink(var) => format!("pink-{}", var),
        }
    }
}