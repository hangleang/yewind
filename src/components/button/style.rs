/// Button Styles
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Style {
    Filled,
    Text,
    Outline,
    Gradient,
    Relief,
}

impl Style {
    pub const ALL: [Style; 5] = [
        Style::Filled,
        Style::Text,
        Style::Outline,
        Style::Gradient,
        Style::Relief,
    ];

    pub fn has_bg_color(&self) -> bool {
        match self {
            Style::Text | Style::Outline => false,
            _ => true
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::Filled
    }
}

impl ToString for Style {
    fn to_string(&self) -> String {
        use Style::*;

        String::from(match self {
            Filled => "bg",
            Text => "text",
            Outline => "border",
            Gradient => "bg-gradient-to-r",
            Relief => "border-b-4"
        })
    }
}