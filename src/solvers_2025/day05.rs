use regex::Regex;

fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut merged = Vec::new();
    ranges.sort_unstable();
    // there can be duplicates
    ranges.dedup();

    merged.push(ranges.pop().unwrap());

    while let Some(range) = ranges.pop() {
        let last = merged.last_mut().unwrap();
        // check if range overlaps. because the list is sorted, it is guaranteed
        // that range.0 <= last.0, however range.1 <= last.1 is not guaranteed
        debug_assert!(range.0 <= last.0);
        if last.0 <= range.1 {
            // it's possible for last to be fully inside range
            *last = (range.0, last.1.max(range.1));
        } else {
            merged.push(range);
        }
    }
    merged
}

fn parse(input: String) -> (std::vec::Vec<(u64, u64)>, std::vec::Vec<u64>) {
    let re = Regex::new(r"(?:(\d+)-(\d+))|(\d+)").unwrap();
    let mut ranges = Vec::new();
    let mut available_ids = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let caps = re.captures(line).expect("line should match regex");
        if let Some(ingredient) = caps.get(3) {
            available_ids.push(ingredient.as_str().parse::<u64>().unwrap());
        } else {
            let start = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let end: u64 = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
            ranges.push((start, end));
        }
    }
    (ranges, available_ids)
}

pub fn part1(input: String) -> String {
    let (ranges, ids) = parse(input);
    let merged = merge_ranges(ranges);

    // by now the range array is so small that optimising this didnt seem to do
    // anything
    ids.into_iter()
        .filter(|&id| merged.iter().any(|&range| range.0 <= id && id <= range.1))
        .count()
        .to_string()
}

pub fn part2(input: String) -> String {
    let (ranges, _) = parse(input);
    let merged = merge_ranges(ranges);
    merged
        .into_iter()
        // +1 because range is inclusive
        .map(|(start, end)| end - start + 1)
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_p1() {
        use super::part1;

        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
        .to_string();
        assert_eq!(part1(input), "3");
    }

    #[test]
    fn sample_p2() {
        use super::part2;

        let input = "3-5
10-14
16-20
12-18"
            .to_string();
        assert_eq!(part2(input), "14");
    }
}
