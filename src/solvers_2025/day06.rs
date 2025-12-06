pub fn part1(input: String) -> String {
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let width = input[0].len();
    // actual input is 1 more row than test
    let height = input.len();
    assert!(
        input.iter().all(|line| line.len() == width),
        "malformed input"
    );

    let mut total = 0;
    // solved similar to part 2
    for (col_idx, operator) in input[height - 1]
        .iter()
        .enumerate()
        .filter(|v| !v.1.is_ascii_whitespace())
    {
        let mut col_width = 0;
        for idx_elem in &input[height - 1][(col_idx + 1)..] {
            if idx_elem.is_whitespace() {
                col_width += 1;
            } else {
                break;
            }
        }
        // edge case: right edge
        if col_width + col_idx == width - 1 {
            col_width += 1;
        }

        let mut numbers = Vec::new();
        // this part differs from p2. we just take numbers like a human would
        // instead of the right to left column stuff
        for row in input.iter().take(height - 1) {
            let number = row[col_idx..col_idx + col_width]
                .iter()
                .filter_map(|c| {
                    if c.is_ascii_whitespace() {
                        None
                    } else {
                        Some(c.to_digit(10).unwrap() as u64)
                    }
                })
                .reduce(|a, b| a * 10 + b)
                .unwrap();
            numbers.push(number);
        }
        match operator {
            '*' => total += numbers.into_iter().reduce(|a, b| a * b).unwrap(),
            '+' => total += numbers.into_iter().reduce(|a, b| a + b).unwrap(),
            _ => unreachable!(),
        }
    }
    total.to_string()
}

pub fn part2(input: String) -> String {
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let width = input[0].len();
    // actual input is 1 more row than test
    let height = input.len();
    assert!(
        input.iter().all(|line| line.len() == width),
        "malformed input"
    );

    let mut total = 0;
    // we use the operator row as our index and find the column width (which is
    // variable) by counting until the next column
    for (col_idx, operator) in input[height - 1]
        .iter()
        .enumerate()
        .filter(|v| !v.1.is_ascii_whitespace())
    {
        let mut col_width = 0;
        for idx_elem in &input[height - 1][(col_idx + 1)..] {
            if idx_elem.is_whitespace() {
                col_width += 1;
            } else {
                break;
            }
        }
        // edge case: right edge
        if col_width + col_idx == width - 1 {
            col_width += 1;
        }

        let mut combined = Vec::new();
        // numbers are read from right to left
        for x in (0..col_width).rev() {
            let mut numbers = [0; 4];
            for y in 0..(height - 1) {
                if let Some(n) = input[y][col_idx + x].to_digit(10) {
                    numbers[y] = n as u64;
                }
            }
            // the input doesn't seem to have any zeroes anywhere
            combined.push(
                // concatenate digits
                numbers
                    .into_iter()
                    .filter(|&v| v != 0)
                    .reduce(|a, b| a * 10 + b)
                    .unwrap(),
            );
        }
        match operator {
            '*' => total += combined.into_iter().reduce(|a, b| a * b).unwrap(),
            '+' => total += combined.into_iter().reduce(|a, b| a + b).unwrap(),
            _ => unreachable!(),
        }
    }

    total.to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_p1() {
        use super::part1;

        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
            .to_string();
        assert_eq!(part1(input), "4277556");
    }

    #[test]
    fn sample_p2() {
        use super::part2;

        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
            .to_string();
        assert_eq!(part2(input), "3263827");
    }
}
