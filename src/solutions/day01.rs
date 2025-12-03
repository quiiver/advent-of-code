fn rem(a: i32, b: i32) -> i32 {
    return a.rem_euclid(b);
}

fn rotate(start: i32, dir: &str, count: i32, count_passes: bool) -> (i32, u32, u32) {
    let sign = if dir == "L" { -1 } else { 1 };
    let pos = start + (sign * rem(count, 100)); // get the absolute position
    let passes = if count_passes {
        (count / 100) + (if start != 0 && (pos < 0 || pos > 100) {
            1
        } else { 0 })
    } else { 0 };

    let end = rem(pos, 100);
    let password = if end == 0 { 1 } else { 0 };

    // println!("start: {}, sign: {}, count: {}, offset: {}, pos: {}, end: {}, passes: {},", start, sign, count, off, pos, end, passes);

    return (end, password, passes as u32);
}

fn get_password(input: &String, count_passes: bool) -> u32 {
    let tpl = input
        .lines()
        .fold((50, 0), |acc, s| {
            let (dir, count) = s.split_at(1);
            let cur = rotate(acc.0, dir, count.parse::<i32>().unwrap(), count_passes);
            return (cur.0, acc.1 + cur.1 + cur.2);
        });
    return tpl.1
}

pub fn solution(input: &String) {
    println!("Part 1: Password is {}", get_password(input, false));
    println!("Part 2: Password is {}", get_password(input, true));
}
