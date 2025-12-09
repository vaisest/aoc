use itertools::Itertools;

fn parse_input(input: String) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|line| {
            let (l, r) = line
                .split_once(",")
                .unwrap_or_else(|| panic!("line {line} not formatted correctly"));
            (l.parse::<_>().unwrap(), r.parse::<_>().unwrap())
        })
        .collect::<_>()
}

pub fn part1(input: String) -> String {
    let coords = parse_input(input);

    coords
        .iter()
        .tuple_combinations()
        .map(|((a, b), (c, d))| (1 + (a - c).abs()) * (1 + (b - d).abs()))
        .max()
        .unwrap()
        .to_string()
}

fn point_inside(vertical_edges: &[(f64, f64, f64)], point: (f64, f64)) -> bool {
    // https://en.wikipedia.org/wiki/Point_in_polygon

    // if horizontal ray crosses the edges an odd amount of times, the point is
    // inside our shape
    let mut intersections = 0u64;
    for &(vx, vy_min, vy_max) in vertical_edges {
        if vx > point.0 && vy_min <= point.1 && point.1 < vy_max {
            intersections += 1;
        }
    }
    intersections % 2 == 1
}

fn intersects(
    vertical_edges: &[(f64, f64, f64)],
    horizon_edges: &[(f64, f64, f64)],
    r_min_x: f64,
    r_min_y: f64,
    r_max_x: f64,
    r_max_y: f64,
) -> bool {
    for &(vx, vy_min, vy_max) in vertical_edges {
        if r_min_x < vx && vx < r_max_x {
            let ol_min = vy_min.max(r_min_y);
            let ol_max = vy_max.min(r_max_y);
            if ol_min < ol_max {
                return true;
            }
        }
    }

    for &(hx_min, hx_max, hy) in horizon_edges {
        if r_min_y < hy && hy < r_max_y {
            let ol_min = hx_min.max(r_min_x);
            let ol_max = hx_max.min(r_max_x);
            if ol_min < ol_max {
                return true;
            }
        }
    }

    false
}

// https://aoc.just2good.co.uk/2025/9 i couldnt do it myself. even copying this
// took 2 hours of debugging
pub fn part2(input: String) -> String {
    let corners = input
        .lines()
        .map(|line| {
            let (l, r) = line
                .split_once(",")
                .unwrap_or_else(|| panic!("line {line} not formatted correctly"));
            (l.parse::<_>().unwrap(), r.parse::<_>().unwrap())
        })
        .collect::<Vec<(f64, f64)>>();

    // [(x, y1, y2)] y1<y2
    let mut vertical_edges = vec![];
    // [(x1, x2, y)] x1<x2
    let mut horizon_edges = vec![];

    for i in 0..corners.len() {
        let a = corners[i];
        let b = corners[(i + 1) % corners.len()];
        // vertical edge
        if a.0 == b.0 {
            vertical_edges.push((a.0, a.1.min(b.1), a.1.max(b.1)));
        } else {
            assert!(a.1 == b.1);
            horizon_edges.push((a.0.min(b.0), a.0.max(b.0), a.1));
        }
    }

    let mut max = 0.0;
    for (a, b) in corners.iter().tuple_combinations() {
        let width = (a.0 - b.0).abs() + 1.0;
        let height = (a.1 - b.1).abs() + 1.0;
        let area = width * height;

        if area <= max {
            continue;
        }

        // top left
        let r_min_x = a.0.min(b.0);
        let r_min_y = a.1.min(b.1);
        // bottom right
        let r_max_x = a.0.max(b.0);
        let r_max_y = a.1.max(b.1);

        // point slightly to the bottom right of the top left of the rectangle
        if point_inside(&vertical_edges, (r_min_x + 0.5, r_min_y + 0.5))
            && !intersects(
                &vertical_edges,
                &horizon_edges,
                r_min_x,
                r_min_y,
                r_max_x,
                r_max_y,
            )
        {
            max = area;
        }
    }
    max.to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_p1() {
        use super::part1;

        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
        .to_string();
        assert_eq!(part1(input), "50");
    }

    #[test]
    fn sample_p2() {
        use super::part2;

        let input = "1,1
3,1
3,3
1,3"
        .to_string();
        assert_eq!(part2(input), "9");

        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
        .to_string();
        assert_eq!(part2(input), "24");

        // https://www.reddit.com/r/adventofcode/comments/1pi5rqn/2025_day_9_part_2_check_your_solution_with_this/
        // .#X#............#X#.
        // .XXX............XXX.
        // .XXX............XXX.
        // .XXX............XXX.
        // .XXX............XXX.
        // .XXX............XXX.
        // .XX#XXXXXXXXXXXX#XX.
        // .XXXXX#XXXXXX#XXXXX.
        // .XXXXXX......XXXXXX.
        // .#XXXX#......#XXXX#.
        let input = "1,0
3,0
3,6
16,6
16,0
18,0
18,9
13,9
13,7
6,7
6,9
1,9"
        .to_string();
        assert_eq!(part2(input), "30");
    }
}
