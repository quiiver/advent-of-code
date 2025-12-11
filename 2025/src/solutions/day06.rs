pub fn solution(input: &String) {
    println!("Part 1: {:?}", solve1(input));
    println!("Part 2: {:?}", solve2(input));
}

type Col = Vec<u64>;
type Columns = Vec<Col>;
type Operations = Vec<String>;
type RightAligned = bool;

fn parse(s: &str) -> u64 {
    s.parse::<u64>().expect("INVALID")
}

fn process_input(input: &str) -> (Columns, Operations) {
    input.lines().map(|line| line.split_whitespace()).fold(
        (Columns::new(), Operations::new()),
        |(mut cols, mut ops), mut row| {
            if cols.is_empty() {
                cols = row.by_ref().map(|x| vec![parse(x)]).collect();
            }
            row.enumerate().for_each(|(idx, s)| {
                if ["*", "+"].contains(&s) {
                    ops.push(s.to_string());
                } else {
                    cols.get_mut(idx).map(|col| {
                        col.push(parse(s));
                        col
                    });
                }
            });
            (cols, ops)
        },
    )
}

fn solve1(input: &str) -> u64 {
    let (cols, ops) = process_input(input);
    // println!("{cols:?}, {ops:?}");
    ops.iter()
        .enumerate()
        .map(|(idx, op)| cols.get(idx).map(|p| apply_op(p, op)).unwrap_or(0))
        .sum()
}

fn read_col(col: &Col, alignment: Option<&RightAligned>) -> Vec<u64> {
    // println!("cols: {col:?}");
    let rows: Vec<u64> = col
        .iter()
        .map(|n| {
            let mut num = *n;
            let mut row: Vec<u64> = Vec::new();
            // println!("num: {num}");
            while num > 0 {
                row.push(num.rem_euclid(10));
                if num < 10 {
                    num = 0;
                } else {
                    num /= 10;
                }
            }
            if *alignment.unwrap() {
                // println!("right aligned, reversing");
                row.reverse();
            }
            // println!("row: {row:?}");
            row
        })
        .enumerate()
        .fold(Vec::new(), |mut acc: Vec<u64>, (idx, row)| {
            // println!("acc: {acc:?}, idx: {idx}, row: {row:?}");
            for (j, n) in row.iter().enumerate() {
                match acc.get_mut(j) {
                    Some(i) => *i = *i * 10 + n,
                    None => acc.insert(j, *n),
                }
            }
            acc
        });
    // println!("rows: {rows:?}");
    rows
}

fn apply_op(nums: &[u64], op: &str) -> u64 {
    if op == "+" {
        nums.iter().sum()
    } else {
        nums.iter().product()
    }
}

fn parse_alignment(input: &str) -> Vec<RightAligned> {
    let widths: Vec<u64> = input
        .lines()
        .last()
        .map(|op_line| {
            op_line
                .chars()
                .rev()
                .fold((vec![], 0), |(mut widths, mut curs), char| {
                    // println!("char: {char:?}");
                    curs += 1;
                    if ['+', '*'].contains(&char) {
                        widths.push(curs - 1);
                        curs = 0
                    }
                    (widths, curs)
                })
                .0
        })
        .unwrap();

    let cols: Vec<bool> = widths
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, width)| {
            input
                .lines()
                .map(move |line| -> &str {
                    let start = (idx * (*width as usize)).min(line.len() - 1);
                    let stop = (start + *width as usize).min(line.len() - 1);
                    // println!(
                    //     "line: {line:?}, idx: {idx}, width: {width}, start: {start} stop: {stop}"
                    // );
                    &line[start..stop]
                })
                .collect()
        })
        .map(|col: Vec<&str>| -> bool { col.iter().all(|s| s.starts_with(' ')) })
        .collect();
    // println!("col widths: {widths:?}");
    // println!("full cols: {cols:?}");
    cols
}

fn solve2(input: &str) -> u64 {
    let (cols, ops) = process_input(input);
    let alignment = parse_alignment(input);
    ops.iter()
        .enumerate()
        .map(|(idx, op)| {
            cols.get(idx)
                .map(|p| apply_op(&read_col(p, alignment.get(idx)), op))
                .unwrap()
        })
        .sum()
}
