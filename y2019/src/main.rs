use std::time::Instant;
use y2019::*;

fn main() {
    let solutions = [day6::solve];

    let start = Instant::now();
    for (day, solve) in solutions.iter().enumerate() {
        let day = day + 1;
        let sol_start = Instant::now();
        println!("{}", solve());
        println!(
            "  Time:   {}ms",
            Instant::now().duration_since(sol_start).as_millis()
        );
    }
    println!(
        "Total time:   {}ms",
        Instant::now().duration_since(start).as_millis()
    );
}
