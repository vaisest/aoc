use std::{iter::repeat_n, mem::swap};

use arrayvec::ArrayVec;

fn solve(input: String) -> (u64, u64) {
    let mut grid: Vec<ArrayVec<char, 141>> = input.lines().map(|it| it.chars().collect()).collect();
    let mut beams = Vec::from_iter(grid[0].iter().map(|&c| (c == 'S') as _));
    let mut new_beams = Vec::from_iter(repeat_n(0, beams.len()));
    let width = grid[0].len();

    assert!(
        grid.iter().all(|line| line.len() == width),
        "malformed input"
    );

    let mut split_counter = 0;
    for row in grid.iter_mut() {
        // we start from 1 timeline, and on each split there is a possibility of
        // arriving in the same place multiple times through different routes
        for (i, (&elem, &prev_beam)) in row.iter().zip(&beams).enumerate() {
            if elem == '^' && prev_beam > 0 {
                new_beams[i - 1] += prev_beam;
                new_beams[i + 1] += prev_beam;
                split_counter += 1;
            } else if prev_beam > 0 {
                new_beams[i] += prev_beam;
            }
        }

        swap(&mut beams, &mut new_beams);
        new_beams = Vec::from_iter(repeat_n(0, beams.len()));
    }

    (split_counter, beams.into_iter().sum::<_>())
}

pub fn part1(input: String) -> String {
    solve(input).0.to_string()
}

pub fn part2(input: String) -> String {
    solve(input).1.to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_p1() {
        use super::part1;

        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
            .to_string();
        assert_eq!(part1(input), "21");
    }

    #[test]
    fn sample_p2() {
        use super::part2;

        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
            .to_string();
        assert_eq!(part2(input), "40");
    }
}
