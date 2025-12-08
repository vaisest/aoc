use std::time::Instant;

use itertools::Itertools;
use rustc_hash::FxHashSet;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

// euclidean distance except no sqrt because they're only used for comparison
fn dist(a: &Coord, b: &Coord) -> i64 {
    // this would be zero but it would break sorting
    assert!(a != b, "a and b should not be equal");
    (a.x - b.x).pow(2) + (a.y - b.y).pow(2) + (a.z - b.z).pow(2)
}

pub fn part1(input: String) -> String {
    let boxes = input
        .lines()
        .map(|line| {
            let (x, y, z) = line
                .split(",")
                .map(|it| {
                    it.parse::<_>()
                        .unwrap_or_else(|_| panic!("error parsing {it}"))
                })
                .next_tuple()
                .expect("input should consist of triples");
            Coord { x, y, z }
        })
        .collect::<Vec<_>>();

    // test input has a different amount of connections
    let connection_count = if boxes.len() == 20 { 10 } else { 1000 };

    let timer = Instant::now();
    let connections_to_make = (0..boxes.len())
        .tuple_combinations()
        .filter_map(|(i, j)| if i == j { None } else { Some((i, j)) })
        .sorted_by_cached_key(|(a, b)| dist(&boxes[*a], &boxes[*b]))
        .take(connection_count);
    println!("{} ms", timer.elapsed().as_millis());
    let mut connections = vec![];
    // we partly merge connections. This merges e.g. (i,j) and (j, k), but not (i,j), (k, l), and (l, i)
    let timer = Instant::now();
    for (i, j) in connections_to_make {
        if connections.is_empty() {
            connections.push(FxHashSet::from_iter([i, j]));
        }
        let mut was_inserted = false;
        for circuit in connections.iter_mut() {
            if circuit.contains(&i) {
                circuit.insert(j);
                was_inserted = true;
            }
            if circuit.contains(&j) {
                circuit.insert(i);
                was_inserted = true;
            }
        }
        if !was_inserted {
            connections.push(FxHashSet::from_iter([i, j]));
        }
    }
    println!("{} ms", timer.elapsed().as_millis());
    // iteratively merge the unmerged circuits from the last step
    let timer = Instant::now();

    'outer: loop {
        for i in 0..connections.len() {
            let mut to_combine = vec![i];
            for j in 0..connections.len() {
                if i == j {
                    continue;
                }
                if connections[i]
                    .intersection(&connections[j])
                    .next()
                    .is_some()
                {
                    to_combine.push(j);
                }
            }
            if to_combine.len() > 1 {
                let mut combined = FxHashSet::default();
                for idx in to_combine.into_iter().sorted().rev() {
                    combined.extend(connections.remove(idx));
                }
                connections.push(combined);
                // we modified the list we're iterating over, so we should restart
                continue 'outer;
            }
        }
        break;
    }
    println!("{} ms\n", timer.elapsed().as_millis());

    connections
        .into_iter()
        .map(|v| v.len())
        .sorted()
        .rev()
        .take(3)
        .reduce(|a, b| a * b)
        .unwrap()
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

        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
            .to_string();
        assert_eq!(part1(input), "40");
    }

    #[test]
    fn sample_p2() {
        use super::part2;

        let input = "".to_string();
        assert_eq!(part2(input), "31");
    }
}
