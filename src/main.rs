mod logic;
mod utility;
mod words;

fn main() {
    for n in [10, 20, 30, 40, 50, 60, 70, 80, 90, 100] {
        test(n)
    }
}

fn test(n: usize) {
    let words: Vec<&str> = words::VALID_WORDS.iter().take(n).copied().collect();

    let (_, t) = utility::timed(|| logic::calculate_entropy(words));

    println!("Using the first {:?}, took {:?}", n, t);
}

/*

Using the first 10, took 8.8354ms
Using the first 20, took 62.938ms
Using the first 30, took 178.061ms
Using the first 40, took 402.1568ms
Using the first 50, took 845.5908ms
Using the first 60, took 1.5198183s
Using the first 70, took 2.3878333s
Using the first 80, took 3.3459977s
Using the first 90, took 4.7401197s
Using the first 100, took 7.0207514s

*/
