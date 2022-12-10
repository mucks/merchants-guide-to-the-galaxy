use crate::{error::Error, result::Result, roman::VecRoman};
use std::collections::HashMap;

use crate::roman::Roman;

pub struct MerchantsGuide {
    sentence_credits: HashMap<String, i32>,
    word_values: HashMap<String, Roman>,
    word_credits: HashMap<String, f32>,
}

impl MerchantsGuide {
    pub fn new() -> Self {
        Self {
            sentence_credits: HashMap::new(),
            word_values: HashMap::new(),
            word_credits: HashMap::new(),
        }
    }

    pub fn handle_input(&mut self, input: &str) -> Result<Option<String>> {
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
                let roman = Roman::try_from(value)?;
                if key.split_whitespace().count() > 1 {
                    return Err(Error::InvalidInput(input.into()));
                }
                if self.word_values.contains_key(key) {
                    return Err(Error::InvalidInput(format!(
                        "word '{}' has already been assigned!",
                        key
                    )));
                }
                self.word_values.insert(key.into(), roman);
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
                return Err(Error::Custom(format!("{} is not defined", word)));
            }
        }
        Ok(roman)
    }

    fn handle_how_much(&mut self, input: &str) -> Result<String> {
        let text = input
            .replace("how much is ", "")
            .replace('?', "")
            .trim_end()
            .to_string();
        let words: Vec<&str> = text.split_whitespace().collect();
        //println!("words: {:?}", words);
        let roman_values = self.words_to_roman(words)?;
        //println!("roman_values: {:?}", roman_values);
        let sum = roman_values.sum()?;
        Ok(format!("{} is {}", text, sum))
    }

    fn get_unknown_words(&self, text: &str) -> Vec<String> {
        text.split_whitespace()
            .map(|w| w.into())
            .filter(|w| !self.word_values.contains_key(w))
            .collect()
    }

    fn handle_how_many(&mut self, input: &str) -> Result<String> {
        let text = input
            .replace("how many Credits is ", "")
            .replace('?', "")
            .trim_end()
            .to_string();

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
                    let sum = roman_values.sum()?;
                    // calculate the credit value of the individual word
                    self.word_credits
                        .insert(unknown_word.clone(), *v as f32 / sum as f32);
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
                        .ok_or_else(|| Error::Custom(format!("unknown word {} not found", word)))?,
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

        let sum = roman_values.sum()?;
        let credit_sum: f32 = credit_values.into_iter().map(|a| a.1).sum();

        let sentence_value = sum as f32 * credit_sum;

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

#[cfg(test)]
mod test {
    use crate::result::Result;

    use super::MerchantsGuide;

    #[test]
    fn test_invalid_inputs() -> Result<()> {
        let mut g = MerchantsGuide::new();

        assert!(g.handle_input("glob is II").is_err());
        assert!(g.handle_input("teesh is III").is_err());
        g.handle_input("phisk is I")?;
        assert!(g.handle_input("phisk is V").is_err());
        assert!(g.handle_input("test test2 is V").is_err());
        assert!(g.handle_input("how much is te ter tem ?").is_err());
        assert!(g.handle_input("how many is phisk?").is_err());

        Ok(())
    }

    #[test]
    fn test_input() -> Result<()> {
        let mut g = MerchantsGuide::new();

        g.handle_input("glob is I")?;
        g.handle_input("prok is V")?;
        g.handle_input("pish is X")?;
        g.handle_input("tegj is L")?;
        g.handle_input("glob glob Silver is 34 Credits")?;
        g.handle_input("glob prok Gold is 57800 Credits")?;
        g.handle_input("pish pish Iron is 3910 Credits")?;

        assert_eq!(
            g.handle_input("how much is pish tegj glob glob ?")?,
            Some("pish tegj glob glob is 42".into())
        );

        assert_eq!(
            g.handle_input("how many Credits is glob prok Silver ?")?,
            Some("glob prok Silver is 68 Credits".into())
        );

        assert_eq!(
            g.handle_input("how many Credits is glob prok Gold ?")?,
            Some("glob prok Gold is 57800 Credits".into())
        );

        assert_eq!(
            g.handle_input("how many Credits is glob prok Iron ?")?,
            Some("glob prok Iron is 782 Credits".into())
        );
        if let Err(err) = g
            .handle_input("how much wood could a woodchuck chuck if a woodchuck could chuck wood ?")
        {
            assert_eq!(
                err.into_generic(),
                "I have no idea what you are talking about".to_string()
            );
        } else {
            panic!("how much wood could a woodchuck chuck if a woodchuck could chuck wood ? should fail");
        }

        Ok(())
    }
}
