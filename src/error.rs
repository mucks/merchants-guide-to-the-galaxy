#[derive(Debug)]
pub enum Error {
    Custom(String),
    ParseIntError(std::num::ParseIntError),
    InvalidRomanValues(String),
}
impl From<std::num::ParseIntError> for Error {
    fn from(parse_int_error: std::num::ParseIntError) -> Self {
        Self::ParseIntError(parse_int_error)
    }
}

impl Error {
    pub fn print_generic(&self) {
        println!("I have no idea what you are talking about");
    }
}
