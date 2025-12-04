pub fn solution(input: &String) {
    println!("Part 1: {}", process_moves(process_input(input), &false));
    println!("Part 2: {}", process_moves(process_input(input), &true));
}

fn process_input(input: &str) -> Vec<Vec<&str>> {
    input.lines().map(|line| line.split("").collect()).collect()
}

fn update_map<'a>(map: &'a Vec<Vec<&str>>, positions: &'a [(usize, usize)]) -> Vec<Vec<&'a str>> {
    let new_map: Vec<Vec<&str>> = map
        .iter()
        .enumerate()
        .map(|(row_idx, row)| -> Vec<&str> {
            row.iter()
                .enumerate()
                .map(|(col_idx, original)| -> &str {
                    if positions.iter().any(|prev| prev == &(row_idx, col_idx)) {
                        "."
                    } else {
                        original
                    }
                })
                .collect()
        })
        .collect();
    new_map
}

fn print_map(map: &Vec<Vec<&str>>) {
    map.iter().for_each(|row| println!("{}", row.join("")));
}

fn process_moves(map: Vec<Vec<&str>>, remove: &bool) -> usize {
    println!("Iteration");
    print_map(&map);
    let positions = find_paper(&map);
    if *remove && !positions.is_empty() {
        // remove positions from map
        let new_map = update_map(&map, &positions);
        positions.len() + process_moves(new_map, remove)
    } else {
        positions.len()
    }
}

// fn check_boundary(cords: (i64, i64), pos: (usize, usize), bounds: ((u64, u64), (u64, u64))) {}

fn find_paper(paper_map: &Vec<Vec<&str>>) -> Vec<(usize, usize)> {
    paper_map
        .iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            let positions: Vec<(usize, usize)> = row
                .iter()
                .enumerate()
                .flat_map(|(col_index, position)| -> Option<(usize, usize)> {
                    if *position != "@" {
                        return None;
                    }
                    let (n, e, s, w): (i64, i64, i64, i64) = (-1, 1, 1, -1);
                    let dirs = [
                        (n, w),
                        (n, 0),
                        (n, e),
                        (0, w),
                        (0, e),
                        (s, w),
                        (s, 0),
                        (s, e),
                    ];
                    let can_forklift: i64 = dirs
                        .map(|d| {
                            let at_boundary = (d.0 == -1 && row_index == 0)
                                || (d.0 == 1 && row_index == paper_map.len() - 1)
                                || (d.1 == -1 && col_index == 0)
                                || (d.1 == 1 && col_index == row.len() - 1);
                            let (ridx, pidx) = (row_index as i64 + d.0, col_index as i64 + d.1);
                            if at_boundary {
                                return 0;
                            } else if paper_map[ridx as usize][pidx as usize] == "@" {
                                return 1;
                            }
                            0
                        })
                        .iter()
                        .sum();
                    if can_forklift < 4 {
                        Some((row_index, col_index))
                    } else {
                        None
                    }
                })
                .collect();
            positions
        })
        .collect()
}
