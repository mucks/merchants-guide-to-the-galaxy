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
