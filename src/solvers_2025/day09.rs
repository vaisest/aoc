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

pub fn part2(input: String) -> String {
    let coords = parse_input(input);

    let v = coords.iter().tuple_windows().map(|((a, b), _, (c, d))|
);

    "-1".to_string()
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
    }
}
