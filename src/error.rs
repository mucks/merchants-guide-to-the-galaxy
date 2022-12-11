// Error Enum to handle different errors later in the appliaction that can occur

#[derive(Debug)]
pub enum Error {
    Custom(String),
    ParseIntError(std::num::ParseIntError),
    InvalidRomanValues(String),
    InvalidInput(String),
}
impl From<std::num::ParseIntError> for Error {
    fn from(parse_int_error: std::num::ParseIntError) -> Self {
        Self::ParseIntError(parse_int_error)
    }
}

impl Error {
    pub fn into_generic(self) -> String {
        "I have no idea what you are talking about".into()
    }
}
