use regex::Regex;
use rustc_hash::FxHashSet;

// my input only has max 10 lights
type Lights = [bool; 10];
const ZERO_LIGHT: [bool; 10] = [false; 10];

fn press_button(mut lights: Lights, button: Lights) -> Lights {
    for (i, light) in button.into_iter().enumerate() {
        lights[i] ^= light;
    }
    lights
}

fn search_solution(goal_lights: Lights, buttons: Vec<Lights>) -> u8 {
    let mut min = u8::MAX;
    let mut s = vec![];
    for (i, button) in buttons.iter().enumerate() {
        // my input had max 13 buttons per light set
        let mut presses = [0u8; 16];
        presses[i] = 1;
        s.push((press_button(ZERO_LIGHT, *button), 1, presses));
    }
    let mut seen = FxHashSet::default();
    while let Some((state, press_count, presses)) = s.pop() {
        if press_count >= min || press_count > 10 || seen.contains(&presses) {
            continue;
        }
        seen.insert(presses);
        if state == goal_lights {
            min = min.min(press_count);
            continue;
        }
        for (i, button) in buttons.iter().enumerate() {
            let mut new_presses = presses;
            new_presses[i] += 1;
            s.push((press_button(state, *button), press_count + 1, new_presses));
        }
    }
    min
}

pub fn part1(input: String) -> String {
    let mut lights = vec![];
    let mut buttons = vec![];

    let light_re = Regex::new(r"\[(.+)\]").unwrap();
    let button_re = Regex::new(r"\((.*?)\)").unwrap();
    let mut longest = 0;
    for line in input.lines() {
        let (_, [light_str]) = light_re.captures(line).unwrap().extract();
        let mut target_lights = [false; 10];
        for (i, light) in light_str.chars().map(|c| c == '#').enumerate() {
            target_lights[i] = light;
        }

        let new_buttons = button_re
            .captures_iter(line)
            .map(|c| c.extract())
            .map(|(_, [cap])| {
                let mut arr = [false; 10];
                for light in cap.split(",").map(|s| s.parse::<usize>().unwrap()) {
                    arr[light] = true;
                }
                arr
            })
            .collect::<Vec<Lights>>();

        longest = longest.max(new_buttons.len());
        lights.push(target_lights);
        buttons.push(new_buttons);
    }

    lights
        .into_iter()
        .zip(buttons)
        .map(|(goal, buttons)| search_solution(goal, buttons) as u64)
        .sum::<u64>()
        .to_string()
}

pub fn part2(input: String) -> String {
    "-1".to_string()
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

        let input = "".to_string();
        assert_eq!(part2(input), "31");
    }
}
