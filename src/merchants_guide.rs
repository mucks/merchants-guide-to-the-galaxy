use crate::{error::Error, result::Result, roman::VecRoman};
use std::collections::HashMap;

use crate::roman::Roman;

/*
The Merchants guide handles the input of the user

*/
pub struct MerchantsGuide {
    // used to store sentences that describe credit values
    // 'glob prok Silver is 34 Credits'
    sentence_credits: HashMap<String, i32>,
    // used to store the roman values that are assigned to specific words
    // 'glob is I'
    word_roman_values: HashMap<String, Roman>,
}

impl MerchantsGuide {
    pub fn new() -> Self {
        Self {
            sentence_credits: HashMap::new(),
            word_roman_values: HashMap::new(),
        }
    }

    // accept input and decide between statement and question
    pub fn handle_input(&mut self, input: &str) -> Result<Option<String>> {
        if input.contains("how") || input.contains('?') {
            let s = self.handle_question(input)?;
            Ok(Some(s))
        } else {
            self.handle_statement(input)?;
            Ok(None)
        }
    }

    // handles a statement (input of values)
    fn handle_statement(&mut self, input: &str) -> Result<()> {
        // a statement must contain is to be valid
        if !input.contains("is") {
            return Err(Error::Custom(
                "input does not contain is and therefore is not valid".into(),
            ));
        }

        let split: Vec<&str> = input.split(" is ").collect();

        if split.len() != 2 {
            return Err(Error::Custom(format!(
                "'{}': no value after the 'is' keyword, split: '{:?}'",
                input, split
            )));
        }
        let key = split[0];
        let value = split[1];

        // if the sentence contains credits add it to the sentence_credits storage
        if value.contains("Credits") {
            let credits_amount: i32 = value.replace("Credits", "").trim().parse()?;
            self.sentence_credits.insert(key.into(), credits_amount);
        } else {
            // if the sentence does not contain credits add it to the roman values storage
            let roman = Roman::try_from(value)?;
            if key.split_whitespace().count() > 1 {
                return Err(Error::InvalidInput(input.into()));
            }
            if self.word_roman_values.contains_key(key) {
                return Err(Error::InvalidInput(format!(
                    "word '{}' has already been assigned!",
                    key
                )));
            }
            self.word_roman_values.insert(key.into(), roman);
        }
        Ok(())
    }

    //converts the input words into roman values
    fn words_to_roman(&self, words: Vec<&str>) -> Result<Vec<Roman>> {
        let mut roman = vec![];
        for word in words {
            if let Some(r) = self.word_roman_values.get(word) {
                roman.push(r.to_owned());
            } else {
                return Err(Error::InvalidInput(format!("{} is not defined", word)));
            }
        }
        Ok(roman)
    }

    // handle any question that asks 'how much is'
    fn handle_how_much(&mut self, input: &str) -> Result<String> {
        let text = Self::format_question(input, "how much is ");
        let words: Vec<&str> = text.split_whitespace().collect();
        let roman_values = self.words_to_roman(words)?;
        let sum = roman_values.sum()?;
        Ok(format!("{} is {}", text, sum))
    }

    // get all words from text that don't exist in the storage
    fn get_unknown_words(&self, text: &str) -> Vec<String> {
        text.split_whitespace()
            .map(|w| w.into())
            .filter(|w| !self.word_roman_values.contains_key(w))
            .collect()
    }

    // get the credit value for a specific word
    fn get_word_credit_value(&self, word: &str) -> Result<f32> {
        match self.sentence_credits.iter().find(|(k, _)| k.contains(word)) {
            Some((k, v)) => {
                let mut roman_values = vec![];
                for w in k.split_whitespace() {
                    if let Some(r) = self.word_roman_values.get(w) {
                        roman_values.push(r.to_owned());
                    };
                }
                let sum = roman_values.sum()?;
                // calculate the credit value of the individual word
                Ok(*v as f32 / sum as f32)
            }
            None => Err(Error::InvalidInput(format!(
                "word {} is not defined!",
                word
            ))),
        }
    }

    fn get_word_credits(&self, text: &str) -> Result<HashMap<String, f32>> {
        let word_credits = self
            .get_unknown_words(text)
            .iter()
            .filter_map(|word| match self.get_word_credit_value(word) {
                Ok(v) => Some((word.to_string(), v)),
                Err(_) => None,
            })
            .collect();
        Ok(word_credits)
    }

    fn format_question(input: &str, q: &str) -> String {
        input.replace(q, "").replace('?', "").trim_end().to_string()
    }

    fn get_credit_sum(&self, input: &str, text: &str) -> Result<f32> {
        let word_credits = self.get_word_credits(text)?;

        //TODO: try to shorten this
        let mut credit_values = vec![];
        //calculate sentence credit value
        for word in text
            .split_whitespace()
            .filter(|word| self.word_roman_values.get(&word.to_string()).is_none())
        {
            let credit = word_credits
                .get(word)
                .ok_or_else(|| Error::InvalidInput(format!("unknown word {} not found", word)))?;
            credit_values.push((word, credit));
        }

        // When there's only one credit word handle the function successfully
        if credit_values.len() == 1 {
            let credit_sum: f32 = *credit_values[0].1;
            return Ok(credit_sum);
        }

        // if there are no credit indicators return error
        if credit_values.is_empty() {
            return Err(Error::InvalidInput(format!(
                "no credit indicator word found in question:  {}",
                input
            )));
        }

        // if there are more than 1 credit indicators also return an error
        Err(Error::InvalidInput(format!(
            "too many credit indicator words found in question:\n'{}'\n these words are '{}'",
            input,
            credit_values
                .iter()
                .map(|w| w.0)
                .collect::<Vec<&str>>()
                .join(",")
        )))
    }

    // handle question that contain 'how many Credits is ... ?'
    fn handle_how_many(&mut self, input: &str) -> Result<String> {
        if !input.contains("how many Credits is ") {
            return Err(Error::InvalidInput(input.into()));
        }
        let text = Self::format_question(input, "how many Credits is ");
        let roman_values: Vec<Roman> = text
            .split_whitespace()
            .filter_map(|word| self.word_roman_values.get(word))
            .map(|r| r.to_owned())
            .collect();

        let credit_sum = self.get_credit_sum(input, &text)?;
        let roman_sum = roman_values.sum()?;
        let sentence_value = roman_sum as f32 * credit_sum;

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
