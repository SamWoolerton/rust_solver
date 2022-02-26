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

--release
Using the first 10, took 3.2559ms
Using the first 20, took 5.5726ms
Using the first 30, took 10.4458ms
Using the first 40, took 15.5882ms
Using the first 50, took 126.9262ms
Using the first 60, took 39.6478ms
Using the first 70, took 60.9412ms
Using the first 80, took 83.1429ms
Using the first 90, took 117.1565ms
Using the first 100, took 244.5292ms

*/
