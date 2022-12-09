use crate::error::Error;
use crate::result::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum Roman {
    I = 1,
    V = 5,
    X = 10,
    L = 50,
    C = 100,
    D = 500,
    M = 1000,
}

impl Roman {
    fn can_subtract(&self, b: &Roman) -> bool {
        use Roman::*;
        match self {
            I => [V, X].contains(b),
            X => [L, C].contains(b),
            C => [D, M].contains(b),
            _ => false,
        }
    }
}

impl TryFrom<&str> for Roman {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        let roman = match value.trim() {
            "I" => Self::I,
            "V" => Self::V,
            "X" => Self::X,
            "L" => Self::L,
            "C" => Self::C,
            "D" => Self::D,
            "M" => Self::M,
            _ => return Err(Error::Custom(format!("invalid roman value: {}", value))),
        };
        Ok(roman)
    }
}

impl std::ops::Add for Roman {
    type Output = i32;

    fn add(self, rhs: Self) -> Self::Output {
        if self < rhs && self.can_subtract(&rhs) {
            return rhs as i32 - self as i32;
        }
        rhs as i32 + self as i32
    }
}

pub trait RomanCalc {
    fn sum(&self) -> Result<i32>;
}

impl RomanCalc for Vec<Roman> {
    fn sum(&self) -> Result<i32> {
        let mut total = 0;

        for i in (0..self.len() - 1).step_by(2) {
            let a = self[i];
            let b = self[i + 1];
            total += a + b;
            //println!("total: {}", total);
        }

        // if the length is uneven add the last element to the total
        if self.len() % 2 != 0 {
            total += self[self.len() - 1] as i32;
        }
        Ok(total)
    }
}
