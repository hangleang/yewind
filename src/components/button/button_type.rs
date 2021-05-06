/// Button HTML Types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
    Button,
    Submit,
    Reset,
}

impl Type {
    pub const ALL: [Type; 3] = [
        Type::Button,
        Type::Submit,
        Type::Reset
    ];
}

impl Default for Type {
    fn default() -> Self {
        Self::Button
    }
}

impl ToString for Type {
    fn to_string(&self) -> String {
        use Type::*;

        String::from(match self {
            Button => "button",
            Submit => "submit",
            Reset => "reset",
        })
    }
}