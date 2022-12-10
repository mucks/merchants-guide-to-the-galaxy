use std::collections::HashMap;
use std::fmt::Display;

use crate::error::Error;
use crate::result::Result;

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
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

impl Display for Roman {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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
impl std::ops::Sub for Roman {
    type Output = Result<i32>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.can_subtract(&rhs) {
            Ok(rhs as i32 - self as i32)
        } else {
            Err(Error::InvalidRomanValues(format!(
                "{} can't be subtracted from {}",
                self, rhs
            )))
        }
    }
}

pub trait VecRoman {
    fn sum(&self) -> Result<i32>;
    fn validate(&self) -> Result<()>;
    fn from_str(s: &str) -> Result<Vec<Roman>>;
}

impl VecRoman for Vec<Roman> {
    fn validate(&self) -> Result<()> {
        use Roman::*;
        let mut map = HashMap::new();
        let mut previous_r: Option<Roman> = None;

        for r in self {
            if previous_r != Some(r.to_owned()) {
                map = HashMap::new();
            }
            if let Some(r_count) = map.get_mut(r) {
                *r_count += 1;

                // I, X, C, M can be repeated 3 times but not more
                if [I, X, C, M].contains(r) && *r_count > 3 {
                    return Err(Error::InvalidRomanValues(format!(
                        "{:?} was repeated more than 3 times",
                        r
                    )));
                }

                // D, L, V can never be repeated
                if [D, L, V].contains(r) && *r_count > 1 {
                    return Err(Error::InvalidRomanValues(format!(
                        "{:?} was repeated more than 1 time",
                        r
                    )));
                }
            } else {
                map.insert(r, 1);
            }
            previous_r = Some(r.to_owned());
        }

        Ok(())
    }
    fn sum(&self) -> Result<i32> {
        self.validate()?;

        let mut total = 0;

        let mut count = 0;

        while count < self.len() {
            let current = self[count];

            if let Some(next) = self.get(count + 1) {
                // If the value 'previous' that's 2 steps behind the value 'next' is smaller than the roman value is invalid
                if count >= 1 {
                    let prev = self[count - 1];
                    if prev < *next {
                        return Err(Error::InvalidRomanValues(format!(
                            "{} 2 steps before {} is smaller",
                            prev, next
                        )));
                    }
                    if prev > current && *next >= prev {
                        return Err(Error::InvalidRomanValues(format!(
                            "conversion error for: {}{}{}",
                            prev, current, next
                        )));
                    }
                }

                // If the current value is smaller than the next try to subtract it and throw error if the conversion is invalid
                if current < *next {
                    let v = (current - *next)?;
                    total += v;
                    count += 2;
                } else {
                    total += current as i32;
                    count += 1;
                }
            } else {
                total += current as i32;
                count += 1;
            }
        }

        Ok(total)
    }

    fn from_str(s: &str) -> Result<Self> {
        let mut roman = Vec::new();
        for c in s.chars() {
            let r = Roman::try_from(c.to_string().as_str())?;
            roman.push(r);
        }
        Ok(roman)
    }
}

#[cfg(test)]
mod test {
    use super::{Roman, VecRoman};
    use crate::result::Result;

    fn test_roman_conversion(s: &str, v: i32) -> Result<()> {
        let roman = <Vec<Roman>>::from_str(s)?;
        assert_eq!(roman.sum()?, v);
        Ok(())
    }

    #[test]
    fn test_roman_conversions() -> Result<()> {
        for (s, v) in [
            ("III", 3),
            ("II", 2),
            ("IV", 4),
            ("LIX", 59),
            ("MMMDCCXXIV", 3724),
        ] {
            test_roman_conversion(s, v)?;
        }
        Ok(())
    }

    fn test_roman_conversion_fail(s: &str) -> Result<i32> {
        let roman = <Vec<Roman>>::from_str(s)?;
        roman.sum()
    }

    #[test]
    fn test_roman_conversion_fails() -> Result<()> {
        for s in ["IIII", "IIV", "LXLX"] {
            if let Ok(sum) = test_roman_conversion_fail(s) {
                panic!("{} should be invalid, resulted in {}", s, sum);
            }
        }
        Ok(())
    }
}
