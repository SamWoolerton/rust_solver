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

type GuessEntropy<'a> = (Word<'a>, f64);

type ScoreArr = [bool; 5];

// runs in O(N^3) time as it requires evaluating every possible word as a guess against every possible word as an answer, and evaluating a combination requires filtering the word list
pub fn calculate_entropy<'a>(arr: WordList<'a>) -> Vec<GuessEntropy<'a>> {
    let total_options = arr.len();
    let mut agg_map: HashMap<Word, (f64, usize)> = HashMap::new();

    // pre-process to lift out of the inner loop
    let guesses: Vec<Guess> = arr.iter().copied().map(preprocess_guess).collect();
    let answers: Vec<Answer> = arr.iter().copied().map(preprocess_answer).collect();

    for guess in guesses.iter() {
        for answer in answers.iter() {
            let checked_guess = score_guess_impl(guess, answer);
            let score = (filter_words(checked_guess, &answers) as f64) / (total_options as f64);

            agg_map
                .entry(guess.text)
                .and_modify(|(sum, count)| {
                    *sum += score;
                    *count += 1;
                })
                .or_insert((score, 1));
        }
    }

    agg_map
        .into_iter()
        .map(|(guess, (sum, count))| (guess, ((sum / count as f64).log2())))
        .collect()
}

fn filter_words<'a>(g: CheckedGuess, previous_words: &Vec<Answer>) -> usize {
    let processed_guess = preprocess_guess(g.guess);

    previous_words
        .iter()
        .filter(|a| match (g.fully_correct, g.partially_correct) {
            (0, 0) => !a.text.chars().any(|c| g.guess.contains(c)),
            _ => {
                let s = score_guess_impl(&processed_guess, a);
                s.fully_correct == g.fully_correct && s.partially_correct == g.partially_correct
            }
        })
        .count()
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

fn score_guess_impl<'a>(guess: &Guess<'a>, answer: &Answer) -> CheckedGuess<'a> {
    let fully_correct_arr = check_word_exact(guess, answer.text);

    let fully_correct = fully_correct_arr.iter().filter(|b| **b).count();

    let partially_correct = if fully_correct == 5 {
        0
    } else {
        check_word_partial(guess, answer, fully_correct_arr)
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

#[test]
fn test_score_guess() {
    assert_eq!(
        score_guess_impl(&preprocess_guess("guess"), &preprocess_answer("guess")),
        {
            CheckedGuess {
                guess: "guess",
                fully_correct: 5,
                partially_correct: 0,
            }
        }
    );
    assert_eq!(
        score_guess_impl(&preprocess_guess("creed"), &preprocess_answer("bleed")),
        {
            CheckedGuess {
                guess: "creed",
                fully_correct: 3,
                partially_correct: 0,
            }
        }
    );
    assert_eq!(
        score_guess_impl(&preprocess_guess("guess"), &preprocess_answer("trace")),
        {
            CheckedGuess {
                guess: "guess",
                fully_correct: 0,
                partially_correct: 1,
            }
        }
    );
    assert_eq!(
        score_guess_impl(&preprocess_guess("beech"), &preprocess_answer("crest")),
        {
            CheckedGuess {
                guess: "beech",
                fully_correct: 1,
                partially_correct: 1,
            }
        }
    );
    assert_eq!(
        score_guess_impl(&preprocess_guess("crest"), &preprocess_answer("trees")),
        {
            CheckedGuess {
                guess: "crest",
                fully_correct: 2,
                partially_correct: 2,
            }
        }
    );
    assert_eq!(
        score_guess_impl(&preprocess_guess("creed"), &preprocess_answer("pleat")),
        {
            CheckedGuess {
                guess: "creed",
                fully_correct: 1,
                partially_correct: 0,
            }
        }
    );
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

fn check_word_partial(guess: &Guess, answer: &Answer, exact_arr: ScoreArr) -> ScoreArr {
    if guess.has_dups {
        check_word_partial_dups(guess, answer, exact_arr)
    } else {
        check_word_partial_no_dups(guess, answer.text, exact_arr)
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
fn check_word_partial_dups(guess: &Guess, answer: &Answer, exact_arr: ScoreArr) -> ScoreArr {
    let mut guess_iter = guess.text.chars();

    // remove exact matches for guess from generic answer frequencies
    let mut answer_frequencies = answer.char_counts.clone();
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
