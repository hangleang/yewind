/// Button Shapes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    Default,
    Round,
    Circle,
}

impl Shape {
    pub const ALL: [Shape; 3] = [
        Shape::Default,
        Shape::Round,
        Shape::Circle
    ];
}

impl Default for Shape {
    fn default() -> Self {
        Self::Default
    }
}

impl ToString for Shape {
    fn to_string(&self) -> String {
        use Shape::*;

        String::from(match self {
            Default => "rounded",
            Round => "rounded-md",
            Circle => "rounded-full",
        })
    }
}