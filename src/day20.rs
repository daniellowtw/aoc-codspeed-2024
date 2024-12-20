use std::collections::VecDeque;

fn solve_all(pi: &PuzzleInput, start: (usize, usize)) -> Vec<Vec<i32>> {
    // Returns the cost map from start position to all other positions.
    let mut queue: VecDeque<(usize, usize, i32)> = VecDeque::new();
    let mut seen: Vec<Vec<i32>> = vec![vec![999999; pi.grid[0].len()]; pi.grid.len()];

    queue.push_back((start.0, start.1, 0));

    while let Some((r, w, score)) = queue.pop_front() {
        if pi.grid[r][w] == '#' {
            continue;
        }
        if seen[r][w] <= score {
            continue;
        }
        seen[r][w] = score;
        queue.push_back((r + 1, w, score + 1));
        queue.push_back((r - 1, w, score + 1));
        queue.push_back((r, w + 1, score + 1));
        queue.push_back((r, w - 1, score + 1));
    }
    seen
}

#[aoc(day20, part1)]
fn part1(pi: &str) -> i32 {
    let pi = parse(pi);
    solve_general(&pi, 2, 100) as i32
}

fn solve_general(pi: &PuzzleInput, cheat: i32, limit: i32) -> usize {
    // I took quite long for this. I kept missing the edge conditions such as not ending the cheat when it's still on the road.
    // My BFS and DFS solution was a mess. They would work, but was way too inefficient and took too long.

    // Only after the realization that treating a cheat just as a teleportation from A -> B.
    // Where the only requirements are A and B are both on the road did I rewrite this to use dijkstra.
    // So the idea here is calculate the distance to all points from the start and the end.
    // Then for each point achievable from the start that is less than the limit, try to teleport, and then look up the distance to the end.
    let from_start = solve_all(pi, pi.start);
    let from_end = solve_all(pi, pi.end);
    let orig = from_start[pi.end.0][pi.end.1];

    let mut count = 0;
    for r in 0..from_start.len() {
        for w in 0..from_start[r].len() {
            let cost = from_start[r][w];
            if cost <= orig - limit {
                for i in -cheat..=cheat {
                    let left: i32 = cheat - i.abs();
                    for j in -left..=left {
                        let dist = i.abs() + j.abs();
                        let nr = r as i32 + i;
                        let nw = w as i32 + j;
                        if nr < 0
                            || nw < 0
                            || nr >= pi.grid.len() as i32
                            || nw >= pi.grid[0].len() as i32
                        {
                            continue;
                        }
                        let total_cost = cost + from_end[nr as usize][nw as usize] + dist;
                        let savings = orig - total_cost;
                        if savings < limit {
                            continue;
                        }
                        count += 1
                    }
                }
            }
        }
    }

    count
}

#[aoc(day20, part2)]
fn part2(pi: &str) -> usize {
    let pi = parse(pi);
    solve_general(&pi, 20, 100)
}

#[derive(Debug, Clone)]
struct PuzzleInput {
    grid: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
}

fn parse(input: &str) -> PuzzleInput {
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|line| line.trim().chars().collect())
        .collect();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == 'S' {
                start = (i, j);
            }
            if *cell == 'E' {
                end = (i, j);
            }
        }
    }
    PuzzleInput { grid, start, end }
}
