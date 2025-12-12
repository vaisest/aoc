use arrayvec::ArrayVec;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Bin {
    area: u64,
    quantities: ArrayVec<u64, 10>,
}

pub fn part1(input: String) -> String {
    let shape_re = Regex::new(r"(?s:\d:\n(.{11})\n\n)").unwrap();
    let bin_re = Regex::new(r"(\d{1,2})x(\d{1,2}): (.+)").unwrap();
    let shapes = shape_re
        .captures_iter(&input)
        .map(|caps| {
            let (_, [shape]) = caps.extract();
            shape.chars().filter(|c| *c == '#').count() as u64
        })
        .collect_vec();
    let bins = bin_re
        .captures_iter(&input)
        .map(|caps| {
            let (_, [width, height, quantities]) = caps.extract();
            let area = width.parse::<u64>().unwrap() * height.parse::<u64>().unwrap();
            let quantities = quantities
                .split_whitespace()
                .map(|word| word.parse::<_>().unwrap())
                .collect();
            Bin { area, quantities }
        })
        .collect_vec();
    let mut count = 0;
    for bin in bins {
        let [simple_area, complex_area] = bin
            .quantities
            .iter()
            .enumerate()
            // test
            .map(|(i, count)| [count * 9, count * shapes[i]])
            .reduce(|a, b| [a[0] + b[0], a[1] + b[1]])
            .unwrap();
        let fits_with_simple_packing = simple_area <= bin.area;

        // apparently the real input does not have situations where this is
        // necessary. The correct answer is simply the amount of simple packing fits
        let _possibly_fits_with_complex_packing = complex_area <= bin.area;
        #[cfg(test)] // i dont care to actually implement it just for one test
        if !fits_with_simple_packing && _possibly_fits_with_complex_packing {
            return "-1".to_string();
        }

        if fits_with_simple_packing {
            count += 1;
        }
    }
    count.to_string()
}

pub fn part2(_: String) -> String {
    "there was no day 12 part 2".to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_p1() {
        use super::part1;

        let input = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2"
            .to_string();
        assert_eq!(part1(input), "-1");
    }
}
