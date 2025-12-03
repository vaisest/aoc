use arrayvec::ArrayVec;

fn parse(input: &str) -> impl Iterator<Item = ArrayVec<u64, 100>> {
    input.lines().map(|line| {
        line.chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect()
    })
}

pub fn part1(input: String) -> String {
    parse(&input)
        .map(|bank| {
            // max of bank excluding last
            let (high_idx, high) = bank[0..bank.len() - 1]
                .iter()
                .enumerate()
                .max_by_key(|v| v.1)
                .unwrap();
            let second_highest = bank[high_idx + 1..].iter().max().unwrap();
            high * 10 + second_highest
        })
        .sum::<u64>()
        .to_string()
}

pub fn part2(input: String) -> String {
    parse(&input)
        // idea: max from sliding window of 12 where the start index is the max of the
        // previous step
        .map(|bank| {
            let mut total_joltage = 0;
            let mut last_high_idx = 0;
            for left in (0..12).rev() {
                // max of bank after the last chosen battery, but also excluding the
                // tail of the bank, because we need to get 12 batteries in total
                let (high_idx, high) = bank[last_high_idx..bank.len() - left]
                    .iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| {
                        // differs from a.cmp(b) by not having equality. this means
                        // the first max is returned and not the last max
                        if a > b {
                            std::cmp::Ordering::Greater
                        } else {
                            std::cmp::Ordering::Less
                        }
                    })
                    .unwrap();
                last_high_idx += high_idx + 1;
                total_joltage += *high * 10u64.pow(left as u32)
            }
            total_joltage
        })
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_p1() {
        use super::part1;

        let input = "987654321111111
811111111111119
234234234234278
818181911112111"
            .to_string();
        assert_eq!(part1(input), "357");
    }

    #[test]
    fn sample_p2() {
        use super::part2;

        let input = "987654321111111
811111111111119
234234234234278
818181911112111"
            .to_string();
        assert_eq!(part2(input), "3121910778619");
    }
}
