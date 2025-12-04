pub fn solution(input: &String) {
    println!("Part 1: {}", jolts(input, 2));
    println!("Part 2: {}", jolts(input, 12));
}

fn process_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| -> Vec<u64> { line.chars().map(|c| self::parse(c.to_string())).collect() })
        .collect()
}

fn parse(s: String) -> u64 {
    s.parse::<u64>().expect("INVALID")
}

fn slice<T>(x: &[T], start: usize, end: usize) -> &[T] {
    match x.get(start..end) {
        Some(xs) => xs,
        None => panic!("Invalid range {start}, {end}"),
    }
}

fn max_index(xs: &[u64]) -> (&u64, usize) {
    let max = xs.iter().max().unwrap();
    (max, xs.iter().position(|x| x == max).unwrap())
}

fn jolts(input: &str, size: usize) -> u64 {
    process_input(input)
        .iter()
        .map(|bank| {
            (0..size)
                .fold((0, 0), |acc, i| {
                    let (start, end) = (acc.0, bank.len() - (size - i) + 1);
                    let range = slice(bank, start, end);
                    let (max, index) = max_index(range);
                    (acc.0 + index + 1, acc.1 * 10 + max)
                })
                .1
        })
        .inspect(|x| {
            println!("jolts: {x:?}");
        })
        .sum()
}
