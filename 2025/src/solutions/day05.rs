use std::ops::RangeInclusive;

pub fn solution(input: &String) {
    println!("Part 1: {:?}", solve1(input));
    println!("Part 2: {:?}", solve2(input));
    // println!("Part 2: {}", process_moves(process_input(input), &true));
}

type R = RangeInclusive<u64>;
type Ranges = Vec<R>;
type Ingredients = Vec<u64>;

fn parse(s: &str) -> u64 {
    s.parse::<u64>().expect("INVALID")
}

fn parse_range(s: String) -> R {
    let ints: Vec<u64> = s.split('-').map(self::parse).collect();
    ints[0]..=ints[1]
}

fn process_input(input: &str) -> (Ranges, Ingredients) {
    input.lines().fold(
        (Ranges::new(), Ingredients::new()),
        |(mut ranges, mut ingredients), line| {
            if line.contains('-') {
                let range = self::parse_range(line.to_string());
                ranges.push(range);
                (ranges, ingredients)
            } else if !line.trim().is_empty() {
                ingredients.push(self::parse(line.trim()));
                (ranges, ingredients)
            } else {
                (ranges, ingredients)
            }
        },
    )
}

fn merge_ranges(ranges: &mut Ranges) -> Ranges {
    ranges.sort_by(|a, b| a.start().cmp(b.start()));
    ranges.iter().fold(Ranges::new(), |mut acc, range| {
        if acc.is_empty() {
            acc.push(range.clone())
        } else if acc
            .last()
            .map(|x| x.end() >= range.start())
            .unwrap_or(false)
        {
            if let Some((last, remain)) = acc.split_last() {
                let mut ret = remain.to_vec();
                let new_range = *last.start()..=*range.end().max(last.end());
                ret.push(new_range);
                acc = ret;
            };
        } else {
            acc.push(range.clone());
        }
        acc
    })
}

fn solve1(input: &str) -> u64 {
    let (mut ranges, mut ingredients) = process_input(input);
    let merged_ranges = merge_ranges(&mut ranges);
    ingredients.sort();
    ingredients.iter().fold(0, |acc, id| {
        if merged_ranges.iter().any(|range| range.contains(id)) {
            acc + 1
        } else {
            acc
        }
    })
}

fn solve2(input: &str) -> u64 {
    let (mut ranges, _) = process_input(input);
    merge_ranges(&mut ranges)
        .iter()
        .fold(0, |acc, range| acc + (range.end() + 1 - range.start()))
}
