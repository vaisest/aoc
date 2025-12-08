use std::{cmp::Reverse, collections::BinaryHeap};

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

fn parse_input(input: String) -> Vec<Coord> {
    input
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
        .collect::<Vec<_>>()
}

fn connections_iter(boxes: &[Coord]) -> impl Iterator<Item = (usize, usize)> {
    // getting the distances from a min heap is much faster compared to sorting
    // an iterator
    let mut heap = BinaryHeap::from_iter(
        (0..boxes.len())
            .tuple_combinations()
            .map(|(i, j)| Reverse((dist(&boxes[i], &boxes[j]), i, j))),
    );
    // nightly seems to have into_sorted_iter, but other than that the only
    // option is into_sorted_vec, which would make sense but it's close to 6x as
    // slow. This works like into_sorted_iter
    (0..heap.len()).map(move |_| {
        let v = heap.pop().unwrap();
        (v.0.1, v.0.2)
    })
}

pub fn part1(input: String) -> String {
    let boxes = parse_input(input);
    // test input has a different amount of connections
    let connection_count = if boxes.len() == 20 { 10 } else { 1000 };

    let connections_to_make = connections_iter(&boxes);

    let mut connections = vec![];
    // we partly merge connections. This merges e.g. (i,j) and (j, k), but not (i,j), (k, l), and (l, i)
    for (i, j) in connections_to_make.take(connection_count) {
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

    // iteratively merge the unmerged circuits from the last step
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

    // multiply the sizes of the 3 largest circuits
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

struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>,
    set_count: usize,
}

// https://www.geeksforgeeks.org/dsa/introduction-to-disjoint-set-data-structure-or-union-find-algorithm/
impl UnionFind {
    fn find(&mut self, v: usize) -> usize {
        let root = self.parents[v];
        // compress
        if self.parents[root] != root {
            self.parents[v] = self.find(root);
            return self.parents[v];
        }
        root
    }
    fn unite(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);

        // already united
        if x_root == y_root {
            return;
        }

        self.set_count -= 1;
        if self.ranks[x_root] < self.ranks[y_root] {
            self.parents[x_root] = y_root;
        } else if self.ranks[x_root] > self.ranks[y_root] {
            self.parents[y_root] = x_root;
        } else {
            self.parents[y_root] = x_root;
            self.ranks[x_root] += 1;
        }
    }

    fn are_all_merged(&mut self) -> bool {
        self.set_count == 1
    }

    fn new(count: usize) -> Self {
        Self {
            parents: Vec::from_iter(0..count),
            ranks: vec![0; count],
            set_count: count,
        }
    }
}

pub fn part2(input: String) -> String {
    let boxes = parse_input(input);
    let mut connections_to_make = connections_iter(&boxes);

    // // start with everything being its own parent
    let mut uf = UnionFind::new(boxes.len());

    let mut i = 0;
    let mut j = 0;
    while !uf.are_all_merged() {
        (i, j) = connections_to_make.next().unwrap();
        uf.unite(i, j);
    }
    (boxes[i].x * boxes[j].x).to_string()
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
        assert_eq!(part2(input), "25272");
    }
}
