use std::time::Instant;
use y2021::*;

fn main() {
    let solutions = [
        day1::solve,
        day2::solve,
        day3::solve,
        day4::solve,
        day5::solve,
        day6::solve,
        day7::solve,
        day8::solve,
        day9::solve,
        day10::solve,
        day11::solve,
    ];

    let start = Instant::now();
    for (day, solve) in solutions.iter().enumerate() {
        let day = day + 1;
        let sol_start = Instant::now();
        println!("Day {}:\n{}", day, solve());
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
