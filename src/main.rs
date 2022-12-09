use std::collections::HashMap;

#[derive(Debug)]
enum Error {
    Custom(String),
    ParseIntError(std::num::ParseIntError),
}

type Result<T> = std::result::Result<T, Error>;

impl From<std::num::ParseIntError> for Error {
    fn from(parse_int_error: std::num::ParseIntError) -> Self {
        Self::ParseIntError(parse_int_error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Roman {
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
        if self < rhs {
            if self.can_subtract(&rhs) {
                //println!("{:?} - {:?}", rhs, self);
                rhs as i32 - self as i32
            } else {
                panic!("can't subtract {:?} from {:?}", rhs, self)
            }
        } else {
            //println!("{:?} + {:?}", self, rhs);
            rhs as i32 + self as i32
        }
    }
}

fn sum_roman_values(roman_values: Vec<Roman>) -> i32 {
    let mut total = 0;

    for i in (0..roman_values.len() - 1).step_by(2) {
        let a = roman_values[i];
        let b = roman_values[i + 1];
        total += a + b;
        //println!("total: {}", total);
    }

    // if the length is uneven add the last element to the total
    if roman_values.len() % 2 != 0 {
        total += roman_values[roman_values.len() - 1] as i32;
    }
    total
}

struct MerchantsGuide {
    sentence_credits: HashMap<String, i32>,
    word_values: HashMap<String, Roman>,
    word_credits: HashMap<String, i32>,
}

impl MerchantsGuide {
    fn new() -> Self {
        Self {
            sentence_credits: HashMap::new(),
            word_values: HashMap::new(),
            word_credits: HashMap::new(),
        }
    }

    fn handle_input(&mut self, input: &str) -> Result<Option<String>> {
        if input.contains("how") || input.contains('?') {
            let s = self.handle_question(input)?;
            Ok(Some(s))
        } else {
            self.handle_statement(input)?;
            Ok(None)
        }
    }

    fn handle_statement(&mut self, input: &str) -> Result<()> {
        if input.contains("is") {
            let split: Vec<&str> = input.split(" is ").collect();

            if split.len() != 2 {
                return Err(Error::Custom(format!(
                    "'{}': no value after the 'is' keyword, split: '{:?}'",
                    input, split
                )));
            }
            let key = split[0];
            let value = split[1];

            if value.contains("Credits") {
                let credits_amount: i32 = value.replace("Credits", "").trim().parse()?;
                self.sentence_credits.insert(key.into(), credits_amount);
            } else {
                match Roman::try_from(value) {
                    Ok(roman) => {
                        self.word_values.insert(key.into(), roman);
                    }
                    Err(e) => println!("{:?}", e),
                }
            }
        }
        Ok(())
    }
    fn words_to_roman(&self, words: Vec<&str>) -> Result<Vec<Roman>> {
        let mut roman = vec![];
        for word in words {
            if let Some(r) = self.word_values.get(word) {
                roman.push(r.to_owned());
            } else {
                return Err(Error::Custom(format!("{} not found in word_values", word)));
            }
        }
        Ok(roman)
    }

    fn handle_how_much(&mut self, input: &str) -> Result<String> {
        let text = input.replace("how much is ", "").replace(" ?", "");
        let words: Vec<&str> = text.split_whitespace().collect();
        //println!("words: {:?}", words);
        let roman_values = self.words_to_roman(words)?;
        //println!("roman_values: {:?}", roman_values);
        let sum = sum_roman_values(roman_values);
        Ok(format!("{} is {}", text, sum))
    }

    fn get_unknown_words(&self, text: &str) -> Vec<String> {
        text.split_whitespace()
            .map(|w| w.into())
            .filter(|w| !self.word_values.contains_key(w))
            .collect()
    }

    fn handle_how_many(&mut self, input: &str) -> Result<String> {
        let text = input.replace("how many Credits is ", "").replace(" ?", "");
        let unknown_words = self.get_unknown_words(&text);

        // get the credit values for all unknown words
        for unknown_word in unknown_words {
            for (k, v) in &self.sentence_credits {
                if k.contains(&unknown_word) {
                    let mut roman_values = vec![];
                    for w in k.split_whitespace() {
                        if let Some(r) = self.word_values.get(w) {
                            roman_values.push(r.to_owned());
                        };
                    }
                    let sum = sum_roman_values(roman_values);
                    // calculate the credit value of the individual word
                    self.word_credits.insert(unknown_word.clone(), v / sum);
                }
            }
        }

        let mut roman_values = vec![];
        let mut credit_values = vec![];
        //calculate sentence
        for word in text.split_whitespace() {
            match self.word_values.get(word) {
                Some(r) => roman_values.push(*r),
                None => credit_values.push((
                    word,
                    self.word_credits
                        .get(word)
                        .unwrap_or_else(|| panic!("unknown word {} not found", word)),
                )),
            };
        }

        if credit_values.is_empty() {
            return Err(Error::Custom(format!(
                "no credit indicator word found in question:  {}",
                input
            )));
        }
        if credit_values.len() > 1 {
            return Err(Error::Custom(format!(
                "too many credit indicator words found in question:\n'{}'\n these words are '{}'",
                input,
                credit_values
                    .iter()
                    .map(|w| w.0)
                    .collect::<Vec<&str>>()
                    .join(",")
            )));
        }

        let sum = sum_roman_values(roman_values);
        let credit_sum: i32 = credit_values.into_iter().map(|a| a.1).sum();

        let sentence_value = sum * credit_sum;

        Ok(format!("{} is {} Credits", text, sentence_value))
    }

    fn handle_question(&mut self, input: &str) -> Result<String> {
        if input.contains("how much is ") {
            self.handle_how_much(input)
        } else {
            self.handle_how_many(input)
        }
    }
}

fn print_answer(answer: Option<String>) {
    if let Some(msg) = answer {
        println!("{}", msg);
    }
}

fn run() -> Result<()> {
    let mut g = MerchantsGuide::new();

    g.handle_input("glob is I")?;
    g.handle_input("prok is V")?;
    g.handle_input("pish is X")?;
    g.handle_input("tegj is L")?;
    g.handle_input("tonar is C")?;
    g.handle_input("glob glob Silver is 34 Credits")?;
    g.handle_input("glob prok Gold is 57800 Credits")?;
    g.handle_input("pish pish Iron is 3910 Credits")?;
    print_answer(g.handle_input("how much is pish tegj glob glob ?")?);
    print_answer(g.handle_input("how many Credits is glob prok Silver ?")?);
    print_answer(g.handle_input("how many Credits is glob prok tonar Silver ?")?);
    print_answer(g.handle_input("how many Credits is glob prok Gold ?")?);
    print_answer(g.handle_input("how many Credits is glob prok Iron ?")?);

    Ok(())
}

fn main() {
    run().unwrap()
}
