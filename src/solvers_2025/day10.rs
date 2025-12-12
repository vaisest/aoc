use itertools::Itertools;
use regex::Regex;
use z3::{Optimize, SatResult, ast::Int};

// my input only has max 10 lights
const LIGHT_LEN: usize = 10;
type Lights = [bool; LIGHT_LEN];
const ZERO_LIGHT: [bool; LIGHT_LEN] = [false; LIGHT_LEN];

fn press_button(mut lights: Lights, button: Lights) -> Lights {
    for (i, light) in button.into_iter().enumerate() {
        lights[i] ^= light;
    }
    lights
}

fn search_solution(goal_lights: Lights, buttons: Vec<Lights>) -> u8 {
    // all of the solutions are actually <= 7 presses
    for i in 1..=10 {
        for combo in buttons.iter().combinations(i) {
            let mut lights = ZERO_LIGHT;
            for button in combo {
                lights = press_button(lights, *button);
            }
            if lights == goal_lights {
                return i.try_into().unwrap();
            }
        }
    }
    panic!("no solution found for lights {goal_lights:?} with buttons {buttons:?}");
}

pub fn part1(input: String) -> String {
    let light_re = Regex::new(r"\[(.+)\]").unwrap();
    let button_re = Regex::new(r"\((.*?)\)").unwrap();
    let problems = input.lines().map(|line| {
        let (_, [light_str]) = light_re.captures(line).unwrap().extract();
        // our goal state
        let mut goal_lights = ZERO_LIGHT;
        for (i, light) in light_str.chars().map(|c| c == '#').enumerate() {
            goal_lights[i] = light;
        }

        let buttons = button_re
            .captures_iter(line)
            .map(|c| c.extract())
            .map(|(_, [cap])| {
                // this will be what we xor the current lights with
                let mut arr = ZERO_LIGHT;
                for light in cap.split(",").map(|s| s.parse::<usize>().unwrap()) {
                    arr[light] = true;
                }
                arr
            })
            .collect::<Vec<Lights>>();

        (goal_lights, buttons)
    });

    problems
        // we get our solution from a DFS which tries to avoid unnecessary work
        // by keeping track of how many times each button has been pressed. This
        // is possible because it doesnt matter what order we press the buttons
        // in
        .map(|(goal, buttons)| search_solution(goal, buttons) as u64)
        .sum::<u64>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let joltage_re = Regex::new(r"\{(.*?)\}").unwrap();
    let button_re = Regex::new(r"\((.*?)\)").unwrap();
    let answers = input.lines().map(|line| {
        let mut buttons = button_re
            .captures_iter(line)
            .map(|c| c.extract())
            .map(|(_, [cap])| cap.split(",").map(|s| s.parse::<_>().unwrap()).collect())
            .collect::<Vec<Vec<_>>>();

        let (_, [joltage_str]) = joltage_re.captures(line).unwrap().extract();
        let mut joltages = joltage_str
            .split(",")
            .map(|w| w.parse::<i64>().unwrap())
            .collect_vec();

        // z3 seems to break if a button isnt allowed to be pressed. It should
        // work because the assertions are >= 0 and not > 0, but it doesnt so
        // idk
        while let Some((idx, _)) = joltages
            .iter()
            .enumerate()
            .find(|(_, joltage)| **joltage == 0)
        {
            for button in buttons.iter_mut() {
                button.retain(|dest_joltage| *dest_joltage != idx);
                for dest_joltage in button.iter_mut() {
                    if *dest_joltage >= idx {
                        *dest_joltage -= 1;
                    }
                }
            }
            joltages.remove(idx);
        }

        // our button press count integers
        let presses = buttons
            .iter()
            .enumerate()
            .map(|(i, _)| Int::new_const(format!("press{i}")))
            .collect_vec();

        let optimize = Optimize::new();

        for press in presses.iter() {
            optimize.assert(&press.ge(0))
        }

        for (i, &joltage) in joltages.iter().filter(|v| **v > 0).enumerate() {
            let sum = buttons
                .iter()
                .enumerate()
                .fold(Int::from_i64(0), |acc, (idx, button)| {
                    if button.contains(&(i)) {
                        acc + &presses[idx]
                    } else {
                        acc
                    }
                });
            optimize.assert(&sum.eq(joltage));
        }

        // magic
        optimize.minimize(&Int::add(&presses));

        match optimize.check(&[]) {
            SatResult::Sat => {
                let model = optimize.get_model().unwrap();
                presses
                    .iter()
                    .map(|press| model.eval(press, true).unwrap().as_i64().unwrap())
                    .sum::<i64>()
            }
            SatResult::Unknown => {
                panic!("optimisation failed for line {line} with unknown Sat")
            }
            SatResult::Unsat => {
                panic!("optimisation failed for line {line} with unsatisfiable")
            }
        }
    });

    answers.sum::<i64>().to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_p1() {
        use super::part1;

        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
            .to_string();
        assert_eq!(part1(input), "7");
    }

    #[test]
    fn sample_p2() {
        use super::part2;

        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
            .to_string();
        assert_eq!(part2(input), "33");

        let input = "[.###.#] (1) (2) (3) {0,0,5}".to_string();
        assert_eq!(part2(input), "5");
    }
}
