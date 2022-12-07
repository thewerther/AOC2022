fn main() {
    let data = include_str!("../input.txt");

    let mut calories: Vec<u32> = data.split("\n\n")
        .map(|calories_per_elf| {
            calories_per_elf.lines()
                .filter_map(|calories| calories.parse::<u32>().ok())
                .sum::<u32>()
        })
        .collect();

    calories.sort();

    println!("solution part 1: {}", calories.last().unwrap());

    println!("solution part 2: {}", calories.iter().rev().take(3).sum::<u32>());
}

