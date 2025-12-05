use crate::util::all_adjacent_in_bounds;
use arrayvec::ArrayVec;

enum Tile {
    Paper,
    Empty,
}

fn can_access(mat: &[ArrayVec<Tile, 140>], x: usize, y: usize) -> bool {
    let mut counter = 0;
    for (ny, nx) in all_adjacent_in_bounds(y, x, mat.len()) {
        if matches!(mat[ny][nx], Tile::Paper) {
            counter += 1;
        }
    }

    // The forklifts can only access a roll of paper if there are fewer than
    // four rolls of paper in the eight adjacent positions
    counter < 4
}

pub fn part1(input: String) -> String {
    let grid: Vec<ArrayVec<Tile, 140>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '@' => Tile::Paper,
                    _ => unreachable!("input should only consist of . and @"),
                })
                .collect()
        })
        .collect();

    assert!(
        grid.iter().all(|row| row.len() == grid.len()),
        "input is not a square matrix"
    );
    let mut total = 0;
    for y in 0..grid.len() {
        for x in 0..grid.len() {
            if matches!(grid[y][x], Tile::Empty) {
                continue;
            }

            if can_access(&grid, x, y) {
                total += 1;
            }
        }
    }
    total.to_string()
}

pub fn part2(input: String) -> String {
    let mut grid: Vec<ArrayVec<Tile, 140>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '@' => Tile::Paper,
                    _ => unreachable!("input should only consist of . and @"),
                })
                .collect()
        })
        .collect();

    let mut total = 0;
    loop {
        let mut loop_total = 0;
        for y in 0..grid.len() {
            for x in 0..grid.len() {
                if matches!(grid[y][x], Tile::Empty) {
                    continue;
                }

                if can_access(&grid, x, y) {
                    loop_total += 1;
                    grid[y][x] = Tile::Empty;
                }
            }
        }
        if loop_total == 0 {
            break;
        } else {
            total += loop_total;
        }
    }

    total.to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_p1() {
        use super::part1;

        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
            .to_string();
        assert_eq!(part1(input), "13");
    }

    #[test]
    fn sample_p2() {
        use super::part2;

        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
            .to_string();
        assert_eq!(part2(input), "43");
    }
}
