pub fn solution(input: &String) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn process_input(input: &String) -> Vec<(u64, u64)> {
    input
        .split(",")
        .map(|range| {
            range
                .trim()
                .split_once("-")
                .map(|tpl| (parse(tpl.0), parse(tpl.1)))
                .unwrap()
        })
        .collect()
}

fn parse(s: &str) -> u64 {
    s.parse::<u64>().expect("INVALID")
}

fn part1(input: &String) -> u64 {
    let ranges = process_input(input);
    ranges.into_iter().fold(0, |acc, range| {
        let invalid_sum: u64 = (range.0..=range.1)
            .map(|i| {
                let istr = i.to_string();
                let (a, b) = istr.split_at(istr.len() / 2);
                if a == b { i } else { 0 }
            })
            .sum();
        acc + invalid_sum
    })
}

fn part2(input: &String) -> u64 {
    let ranges = process_input(input);
    ranges.into_iter().fold(0, |acc, range| {
        let invalid_sum: u64 = (range.0..=range.1)
            .map(|i| -> u64 {
                let istr = i.to_string();
                let invalid = (0..istr.len()).any(|idx| {
                    if idx == istr.len() - 1 {
                        return false;
                    }
                    istr.split(istr.get(..idx + 1).unwrap())
                        .all(|s| s.is_empty())
                });
                if invalid { i } else { 0 }
            })
            .sum();
        acc + invalid_sum
    })
}
