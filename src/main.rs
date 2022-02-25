mod logic;
mod utility;

fn main() {
    logic::example();

    let t = utility::timed(|| {
        let mut sum: u64 = 0;
        for x in 1..100000 {
            sum += x
        }
        println!("{:?}", sum);
    });

    println!("{:?}", t);
}
