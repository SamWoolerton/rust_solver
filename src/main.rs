mod logic;
mod utility;
mod words;

fn main() {
    let t = utility::timed(|| {
        let mut count_letters: usize = 0;
        for w in words::VALID_WORDS {
            count_letters += w.len()
        }
        println!("{:?}", count_letters);

        let mut count_words: usize = 0;
        for _guess in words::VALID_WORDS {
            for _answer in words::VALID_WORDS {
                count_words += 1
            }
        }
        println!("{:?}", count_words);
    });

    println!("{:?}", t);
}
