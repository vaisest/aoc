use itertools::Itertools;
use rustc_hash::FxHashMap;

fn parse_input(input: &str) -> FxHashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|line| {
            let (source, rest) = line.split_once(": ").unwrap();
            (source, rest.split_ascii_whitespace().collect_vec())
        })
        .collect::<_>()
}

// simple dfs with memoisation
fn _simple_path<'a>(
    source: &'a str,
    target: &'a str,
    connections: &FxHashMap<&'a str, Vec<&'a str>>,
    memo: &mut FxHashMap<&'a str, u64>,
) -> u64 {
    if source == target {
        return 1;
    }
    if let Some(count) = memo.get(source) {
        return *count;
    }
    let mut path_count = 0;
    if let Some(adj) = connections.get(source) {
        for &node in adj {
            path_count += _simple_path(node, target, connections, memo);
        }
    }
    memo.insert(source, path_count);
    path_count
}
fn simple_path<'a>(
    source: &'a str,
    target: &'a str,
    connections: &FxHashMap<&'a str, Vec<&'a str>>,
) -> u64 {
    let mut memo = FxHashMap::default();
    _simple_path(source, target, connections, &mut memo)
}

pub fn part1(input: String) -> String {
    let connections = parse_input(&input);

    simple_path("you", "out", &connections).to_string()
}

pub fn part2(input: String) -> String {
    let connections = parse_input(&input);

    [("svr", "fft"), ("fft", "dac"), ("dac", "out")]
        .map(|(from, to)| simple_path(from, to, &connections))
        .into_iter()
        .reduce(|a, b| a * b)
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_p1() {
        use super::part1;

        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"
            .to_string();
        assert_eq!(part1(input), "5");
    }

    #[test]
    fn sample_p2() {
        use super::part2;

        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"
            .to_string();
        assert_eq!(part2(input), "2");
    }
}
