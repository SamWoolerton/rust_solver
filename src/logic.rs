use itertools::Itertools;
use std::collections::HashMap;

type Word<'a> = &'a str;
type WordList<'a> = Vec<Word<'a>>;

#[derive(PartialEq, Debug)]
pub struct CheckedGuess<'a> {
    guess: Word<'a>,
    fully_correct: usize,
    partially_correct: usize,
}

#[derive(PartialEq, Debug)]
struct Guess<'a> {
    text: Word<'a>,
    has_dups: bool,
}

type Lookup = HashMap<char, usize>;

#[derive(PartialEq, Debug)]
struct Answer<'a> {
    text: Word<'a>,
    char_counts: Lookup,
}

struct CharDetails {
    char: char,
    index: usize,
    exact: bool,
}

type GuessEntropy<'a> = (Word<'a>, usize);

type ScoreArr = [bool; 5];

pub fn is_correct_guess(checked_guess: CheckedGuess) -> bool {
    checked_guess.fully_correct == 5
}

pub fn example() {
    println!("Hello from logic file!");

fn score_guess<'a>(guess: Word<'a>, answer: Word) -> CheckedGuess<'a> {
    let processed_guess = preprocess_guess(guess);
    score_guess_impl(processed_guess, answer)
}

#[test]
fn test_score_guess() {
    assert_eq!(score_guess("guess", "guess"), {
        CheckedGuess {
            guess: "guess",
            fully_correct: 5,
            partially_correct: 0,
        }
    });
    assert_eq!(score_guess("creed", "bleed"), {
        CheckedGuess {
            guess: "creed",
            fully_correct: 3,
            partially_correct: 0,
        }
    });
    assert_eq!(score_guess("guess", "trace"), {
        CheckedGuess {
            guess: "guess",
            fully_correct: 0,
            partially_correct: 1,
        }
    });
    assert_eq!(score_guess("beech", "crest"), {
        CheckedGuess {
            guess: "beech",
            fully_correct: 1,
            partially_correct: 1,
        }
    });
    assert_eq!(score_guess("crest", "trees"), {
        CheckedGuess {
            guess: "crest",
            fully_correct: 2,
            partially_correct: 2,
        }
    });
    assert_eq!(score_guess("creed", "pleat"), {
        CheckedGuess {
            guess: "creed",
            fully_correct: 1,
            partially_correct: 0,
        }
    });
}

fn preprocess_guess<'a>(guess: Word<'a>) -> Guess<'a> {
    let mut has_dups = false;
    for (a, b) in guess.chars().sorted().tuple_windows() {
        if a == b {
            has_dups = true
        }
    }

    Guess {
        text: guess,
        has_dups,
    }
}

#[test]
fn test_preprocess_guess() {
    assert_eq!(preprocess_guess("tests"), {
        Guess {
            text: "tests",
            has_dups: true,
        }
    });
    assert_eq!(preprocess_guess("clear"), {
        Guess {
            text: "clear",
            has_dups: false,
        }
    });
}

fn preprocess_answer(answer: Word) -> Answer {
    Answer {
        text: answer,
        char_counts: answer.chars().counts(),
    }
}

#[test]
fn test_preprocess_answer() {
    assert_eq!(preprocess_answer("tests"), {
        Answer {
            text: "tests",
            char_counts: HashMap::from([('t', 2), ('s', 2), ('e', 1)]),
        }
    });
}

fn score_guess_impl<'a>(guess: Guess<'a>, answer: Word) -> CheckedGuess<'a> {
    let fully_correct_arr = check_word_exact(&guess, answer);

    let fully_correct = fully_correct_arr.iter().filter(|b| **b).count();

    let partially_correct = if fully_correct == 5 {
        0
    } else {
        check_word_partial(&guess, answer, fully_correct_arr)
            .iter()
            .filter(|b| **b)
            .count()
    };

    CheckedGuess {
        guess: guess.text,
        fully_correct,
        partially_correct,
    }
}

fn check_word_exact(guess: &Guess, answer: Word) -> ScoreArr {
    let mut guess_iter = guess.text.chars();
    let mut answer_iter = answer.chars();

    [false, false, false, false, false].map(|_| guess_iter.next() == answer_iter.next())
}

#[test]
fn test_check_word_exact() {
    assert_eq!(
        check_word_exact(&preprocess_guess("tests"), "check"),
        [false, false, false, false, false]
    );
    assert_eq!(
        check_word_exact(&preprocess_guess("tests"), "teams"),
        [true, true, false, false, true]
    );
}

fn check_word_partial(guess: &Guess, answer: Word, exact_arr: ScoreArr) -> ScoreArr {
    if guess.has_dups {
        check_word_partial_dups(guess, answer, exact_arr)
    } else {
        check_word_partial_no_dups(guess, answer, exact_arr)
    }
}

fn check_word_partial_no_dups(guess: &Guess, answer: Word, exact_arr: ScoreArr) -> ScoreArr {
    let mut guess_iter = guess.text.chars();

    // not exact and letter in answer
    // safe to unwrap as ScoreArr and Word are both always length 5
    exact_arr.map(|exact| !exact && answer.contains(guess_iter.next().unwrap()))
}

// More complex now that guess letters can be duplicated
// For each letter in guess, see if there are enough of the same letter available in answer to be a partial match
// Exclude exact matches from answer letters count as they're not available
// Letters are compared by the order they appear, so the first instance of a duplicate may be a partial match while the next might not
// See tests for example output
fn check_word_partial_dups(guess: &Guess, answer: Word, exact_arr: ScoreArr) -> ScoreArr {
    let mut guess_iter = guess.text.chars();
    let answer_processed = preprocess_answer(answer);

    // remove exact matches for guess from generic answer frequencies
    let mut answer_frequencies = answer_processed.char_counts.clone();
    for exact in exact_arr {
        // safe to unwrap as ScoreArr and Word are both length 5
        let c = guess_iter.next().unwrap();
        if exact {
            answer_frequencies.entry(c).and_modify(|count| *count -= 1);
        }
    }
    // reset iterator
    guess_iter = guess.text.chars();
    // track how many of each character we've seen so far
    let mut counts: HashMap<char, usize> = HashMap::with_capacity(5);
    exact_arr.map(|exact| {
        // always need to increment this, even if returning immediately because it's an exact match - otherwise will get out of sync
        let c = guess_iter.next().unwrap();

        if exact {
            return false;
        }

        // only handle the counts map after early return from exact
        counts.entry(c).and_modify(|count| *count += 1).or_insert(1);

        let dup_index = *counts.get(&c).unwrap_or(&1);
        let answer_count = *answer_frequencies.get(&c).unwrap_or(&0);

        return dup_index <= answer_count;
    })
}
