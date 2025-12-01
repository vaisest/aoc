pub fn part1(input: String) -> String {
    let mut dial = 50;
    let mut password = 0;
    for rotation in input.lines() {
        let (direction, steps) = rotation.split_at(1);
        let steps = steps
            .parse::<i16>()
            .expect("one of the input lines could not be parsed");

        match direction {
            "L" => {
                dial -= steps;
            }
            "R" => {
                dial += steps;
            }
            _ => unreachable!(),
        }

        dial %= 100;
        if dial == 0 {
            password += 1;
        }
    }

    password.to_string()
}

pub fn part2(input: String) -> String {
    let mut dial = 50;
    let mut password = 0;
    for rotation in input.lines() {
        let (direction, steps) = rotation.split_at(1);
        let mut steps = steps
            .parse::<i16>()
            .expect("one of the input lines could not be parsed");

        let hundreds = steps / 100;
        password += hundreds;
        steps %= 100;

        let dial_start = dial;

        match direction {
            "L" => {
                dial -= steps;
            }
            "R" => {
                dial += steps;
            }
            _ => unreachable!(),
        }

        match dial {
            ..0 => {
                dial += 100;
                // if we e.g. go 5 clicks left from 0 we didn't actually cross
                // zero because we started from it, and shouldn't get a password
                // count
                if dial_start != 0 {
                    password += 1;
                }
            }
            0 => {
                password += 1;
            }
            100.. => {
                dial -= 100;
                if dial_start != 0 {
                    password += 1;
                }
            }
            _ => {}
        }
    }

    password.to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_p1() {
        use super::part1;

        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
        .to_string();
        assert_eq!(part1(input), "3");

        let input = "R3
R10
L11
R9
L6
L3
R41
L13
L46
R23
R36"
        .to_string();
        assert_eq!(part1(input), "0");

        let input = "R50
R100
R200"
            .to_string();
        assert_eq!(part1(input), "3");
    }

    #[test]
    fn sample_p2() {
        use super::part2;

        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
        .to_string();
        assert_eq!(part2(input), "6");

        let input = "L68
L30
R48
L5
R60
L55
L1
R1000
L99
R14
L82"
        .to_string();
        assert_eq!(part2(input), "16");
    }
}
