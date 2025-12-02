use regex::Regex;

fn parse_input(input: String) -> Vec<(u64, u64)> {
    let re = Regex::new(r"(\d+)-(\d+)").unwrap();
    re.captures_iter(&input)
        .map(|m| {
            let (_, [start, end]) = m.extract();
            (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        })
        .collect()
}

fn solve(ranges: &[(u64, u64)], max_repeats: u32) -> u64 {
    let mut ids = Vec::new();

    // idea: instead of looping through the range, we loop through its digit
    // count and small numbers (repeaters) which we concatenate using powers of
    // 10 to test if the repeater might be in the range
    for &(start, end) in ranges {
        for digit_count in (start.ilog10() + 1)..=(end.ilog10() + 1) {
            for repeats in 2..=digit_count.min(max_repeats) {
                if !digit_count.is_multiple_of(repeats) {
                    continue;
                }
                let repeater_size = digit_count / repeats;

                // e.g. for repeater size 2 we loop from 10 to 99
                for x in 10u64.pow(repeater_size - 1)..=(10u64.pow(repeater_size) - 1) {
                    let mut number = 0;
                    // e.g.
                    // 11885 + 11885*10^5 + 11885*10^10
                    // = 118851188511885
                    for repeat in 0..(repeats) {
                        number += x * 10u64.pow(repeater_size * repeat)
                    }
                    if number >= start && number <= end {
                        ids.push(number);
                    }
                }
            }
        }
    }
    ids.sort_unstable();
    ids.dedup();
    ids.iter().sum::<u64>()
}

pub fn part1(input: String) -> String {
    let ranges = parse_input(input);
    solve(&ranges, 2).to_string()
}

pub fn part2(input: String) -> String {
    let ranges = parse_input(input);
    solve(&ranges, u32::MAX).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_p1() {
        use super::part1;

        let input = "55-55".to_string();
        assert_eq!(part1(input), "55");

        let input = "6464-6464".to_string();
        assert_eq!(part1(input), "6464");

        let input = "123123-123123".to_string();
        assert_eq!(part1(input), "123123");

        let input = "1010-1010".to_string();
        assert_eq!(part1(input), "1010");

        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124"
            .to_string();
        assert_eq!(part1(input), "1227775554");
    }

    #[test]
    fn sample_p2() {
        use super::part2;

        let input = "55-55".to_string();
        assert_eq!(part2(input), "55");

        let input = "100-111".to_string();
        assert_eq!(part2(input), "111");

        let input = "99-111".to_string();
        assert_eq!(part2(input), "210");

        let input = "1-1".to_string();
        assert_eq!(part2(input), "0");

        let input = "5-15".to_string();
        assert_eq!(part2(input), "11");

        let input = "6464-6464".to_string();
        assert_eq!(part2(input), "6464");

        let input = "646646-646646".to_string();
        assert_eq!(part2(input), "646646");

        let input = "123123-123123".to_string();
        assert_eq!(part2(input), "123123");

        let input = "1212121212-1212121212".to_string();
        assert_eq!(part2(input), "1212121212");

        let input = "11111111-11111111".to_string();
        assert_eq!(part2(input), "11111111");

        let input = "118851188511885-118851188511885".to_string();
        assert_eq!(part2(input), "118851188511885");

        let input = "1-17".to_string();
        assert_eq!(part2(input), "11");

        let input = "12-17".to_string();
        assert_eq!(part2(input), "0");

        let input = "1-4294967296".to_string();
        assert_eq!(part2(input), "88304989965662");

        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,
        824824821-824824827,2121212118-2121212124"
            .to_string();
        assert_eq!(part2(input), "4174379265");

        let input = "656-1074".to_string();
        assert_eq!(part2(input), "4340");
    }
}
