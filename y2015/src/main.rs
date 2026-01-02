use std::time::Instant;
use y2015::*;

fn main() {
    let solutions = [
        day1::solve,
        day2::solve,
        day3::solve,
        //day4::solve,
        day5::solve,
        day6::solve,
        day7::solve,
    ];

    let start = Instant::now();
    for (day, solve) in solutions.iter().enumerate() {
        let mut day = day + 1;
        // day4 is takin too long to solve so skip it for now
        if day >= 4 {
            day += 1;
        }
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
