use regex::Regex;

fn parse(input: &str) -> Vec<(i64, i64, i64, i64, i64, i64)> {
    let re = Regex::new(r"X.([0-9]*).*Y.([0-9]*)").unwrap();
    let ss: Vec<(i64, i64, i64, i64, i64, i64)> = input
        .split("\n\n")
        .filter(|x| !x.is_empty())
        .map(|block| {
            let parsed_block: Vec<(i64, i64)> = block
                .split("\n")
                .filter(|x| !x.is_empty())
                .map(|line| {
                    let y: Vec<(i64, i64)> = re
                        .captures_iter(line)
                        .map(|cap| {
                            (
                                cap[1].parse::<i64>().unwrap(),
                                cap[2].parse::<i64>().unwrap(),
                            )
                        })
                        .collect();
                    y[0]
                })
                .collect();
            (
                parsed_block[0].0,
                parsed_block[0].1,
                parsed_block[1].0,
                parsed_block[1].1,
                parsed_block[2].0,
                parsed_block[2].1,
            )
        })
        .collect();
    return ss;
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> i64 {
    let ss = parse(input);
    solve_part1(&ss)
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> i64 {
    let ss = parse(input);
    solve_part2(&ss)
}

fn solve(a: i64, b: i64, c: i64, d: i64, e: i64, f: i64) -> i64 {
    let mut res = vec![];
    for i in 1..=100 {
        for j in 1..=100 {
            if a * i + c * j == e && b * i + d * j == f {
                // println!("{} {} {} {}", a * i + c * j, b * i + d * j, e, f);
                res.push(3 * i + j);
            }
        }
    }
    if res.is_empty() {
        return 0;
    }
    return res.iter().min().unwrap().clone();
}

fn solve_simultaneous(a1: i64, b1: i64, c1: i64, a2: i64, b2: i64, c2: i64) -> Option<(i64, i64)> {
    let det_a = a1 * b2 - a2 * b1;
    if det_a.abs() == 0 {
        return None;
    }

    let det_x = c1 * b2 - c2 * b1;
    let det_y = a1 * c2 - a2 * c1;

    if (det_x % det_a != 0) || (det_y % det_a != 0) {
        return None;
    }
    let x = det_x / det_a;
    let y = det_y / det_a;
    Some((x, y))
}

fn solve_part1(pi: &Vec<(i64, i64, i64, i64, i64, i64)>) -> i64 {
    pi.iter().map(|f| solve(f.0, f.1, f.2, f.3, f.4, f.5)).sum()
}

fn solve_part2(pi: &Vec<(i64, i64, i64, i64, i64, i64)>) -> i64 {
    pi.iter()
        .map(|r| {
            match solve_simultaneous(
                r.0,
                r.2,
                10000000000000 + r.4,
                r.1,
                r.3,
                10000000000000 + r.5,
            ) {
                Some(res) => 3 * res.0 + res.1,
                None => 0,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn sample1() {}
}
