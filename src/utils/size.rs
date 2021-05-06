/// Button Sizes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Size {
    Default,
    XSmall,
    Small,
    Large,
    XLarge,
}

impl Size {
    pub const ALL: [Size; 5] = [
        Size::XSmall,
        Size::Small,
        Size::Default,
        Size::Large,
        Size::XLarge,
    ];
}

impl Default for Size {
    fn default() -> Self {
        Self::Default
    }
}

impl ToString for Size {
    fn to_string(&self) -> String {
        use Size::*;

        String::from(match self {
            Default => "base",
            XSmall => "xs",
            Small => "sm",
            Large => "lg",
            XLarge => "xl",
        })
    }
}