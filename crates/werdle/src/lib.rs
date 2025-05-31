use rand::seq::IndexedRandom;
use std::collections::HashSet;
use thiserror::Error;

static ALL_WORDS: &str = include_str!("../resources/werds.txt");
const WORD_LENGTH: usize = 5;
const MAX_TRIES: usize = 6;

fn werdz() -> Vec<&'static str> {
    ALL_WORDS.lines().collect()
}

#[derive(Error, Debug)]
pub enum WerdleError {
    #[error("Guess is the wrong length")]
    GuessLengthError,
}

#[derive(Debug, Clone)]
pub enum GuessCharState {
    WrongChar,
    WrongPlace,
    RightChar,
}

#[derive(Debug, Default, Clone)]
pub struct GuessResult {
    pub result: Vec<(char, GuessCharState)>,
}

impl GuessResult {
    pub fn new() -> Self {
        Self { result: Vec::new() }
    }

    pub fn add(&mut self, c: char, s: GuessCharState) -> &Self {
        self.result.push((c, s));
        self
    }
}

#[derive(Debug, Default)]
pub struct Game {
    // dict: Vec<&'static str>,
    werd: String,
    letterz: HashSet<char>,
    guesses: Vec<String>,
    lastresult: Option<GuessResult>,
}

impl Game {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let dict = werdz();
        let werd = *dict.choose(&mut rng).unwrap_or(&"");
        Self {
            // dict,
            werd: werd.to_string().to_ascii_uppercase(),
            letterz: HashSet::new(),
            guesses: Vec::new(),
            lastresult: None,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.is_correct() || self.no_more_tries()
    }

    pub fn is_correct(&self) -> bool {
        match self.guesses.last() {
            Some(guess) => *guess == self.werd,
            None => false,
        }
    }

    pub fn no_more_tries(&self) -> bool {
        self.guesses.len() >= MAX_TRIES
    }

    pub fn guess(&mut self, guess: &str) -> Result<GuessResult, WerdleError> {
        if guess.len() != WORD_LENGTH {
            return Err(WerdleError::GuessLengthError);
        }

        let mut result = GuessResult::new();
        guess
            .to_ascii_uppercase()
            .chars()
            .enumerate()
            .for_each(|(i, c)| {
                let char_result = if self.werd.chars().nth(i).unwrap_or('!') == c {
                    GuessCharState::RightChar
                } else if self.werd.chars().any(|wc| wc == c) {
                    GuessCharState::WrongPlace
                } else {
                    GuessCharState::WrongChar
                };
                // if matches!(char_result, GuessCharState::WrongChar) {
                self.letterz.insert(c);
                // }
                result.result.push((c, char_result));
            });

        self.guesses.push(guess.to_ascii_uppercase());
        self.lastresult = Some(result.clone());
        Ok(result)
    }

    pub fn guessed_letters(&self) -> String {
        let mut s = String::new();
        let mut l = self.letterz.iter().collect::<Vec<&char>>();
        l.sort();
        let len = l.len();
        for (i, c) in l.iter().enumerate() {
            s.push(**c);
            if i + 1 < len {
                s.push(' ');
            }
        }
        s
    }

    pub fn unguessed_letters(&self) -> String {
        let lz: HashSet<char> = ('A'..='Z').collect();
        let mut s = String::new();
        let guessed_letters = self.letterz.iter().copied().collect::<HashSet<char>>();
        let unguessed = lz.difference(&guessed_letters);
        let mut u = unguessed.copied().collect::<Vec<char>>();
        u.sort();
        let len = u.len();
        for (i, c) in u.iter().enumerate() {
            s.push(*c);
            if i + 1 < len {
                s.push(' ');
            }
        }
        s
    }

    pub fn last_guess(&self) -> Option<GuessResult> {
        self.lastresult.clone()
    }

    pub fn guesses_left(&self) -> usize {
        MAX_TRIES - self.guesses.len()
    }

    pub fn print_werd(&self) {
        println!("Werd is `{}`", &self.werd);
    }

    pub fn werd(&self) -> &str {
        self.werd.as_str()
    }
}
