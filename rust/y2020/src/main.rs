use y2020::days::*;
use y2020::utils::get_input;

fn main() {
    let solutions = [(day1::part1, day1::part2), (day2::part1, day2::part2)];

    for (day, (part1, part2)) in solutions.iter().enumerate() {
        let day = day + 1;
        let input = get_input(day);
        println!(
            "Day {}:\n  Part 1: {}\n  Part 2: {}",
            day,
            part1(&input),
            part2(&input)
        );
    }
}
