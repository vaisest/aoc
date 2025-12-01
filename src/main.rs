use clap::Parser;
use std::{fs, hint::black_box, time::Instant};

mod solvers_2024;
mod solvers_2025;

fn read_input(year: &str, day: &str) -> String {
    let path = format!("input/{year}/day_{day}.txt");
    fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("could not read input file from {path} with error {e}"))
        // rust seems to have a crazy amount of trouble with windows line
        // endings and splitting strings
        .replace("\r\n", "\n")
}

type SolverType = fn(String) -> String;
fn get_function_and_data(year: usize, day: usize) -> ((SolverType, SolverType), String) {
    // i couldn't figure out a way to macro this :/
    let input = read_input(&year.to_string(), &format!("{day:0>2}"));
    let functions: (SolverType, SolverType) = match year {
        2024 => match day {
            1 => (solvers_2024::day01::part1, solvers_2024::day01::part2),
            2 => (solvers_2024::day02::part1, solvers_2024::day02::part2),
            3 => (solvers_2024::day03::part1, solvers_2024::day03::part2),
            4 => (solvers_2024::day04::part1, solvers_2024::day04::part2),
            5 => (solvers_2024::day05::part1, solvers_2024::day05::part2),
            6 => (solvers_2024::day06::part1, solvers_2024::day06::part2),
            7 => (solvers_2024::day07::part1, solvers_2024::day07::part2),
            8 => (solvers_2024::day08::part1, solvers_2024::day08::part2),
            9 => (solvers_2024::day09::part1, solvers_2024::day09::part2),
            10 => (solvers_2024::day10::part1, solvers_2024::day10::part2),
            11 => (solvers_2024::day11::part1, solvers_2024::day11::part2),
            12 => (solvers_2024::day12::part1, solvers_2024::day12::part2),
            13 => (solvers_2024::day13::part1, solvers_2024::day13::part2),
            14 => (solvers_2024::day14::part1, solvers_2024::day14::part2),
            15 => (solvers_2024::day15::part1, solvers_2024::day15::part2),
            16 => (solvers_2024::day16::part1, solvers_2024::day16::part2),
            17 => (solvers_2024::day17::part1, solvers_2024::day17::part2),
            18 => (solvers_2024::day18::part1, solvers_2024::day18::part2),
            19 => (solvers_2024::day19::part1, solvers_2024::day19::part2),
            20 => (solvers_2024::day20::part1, solvers_2024::day20::part2),
            21 => (solvers_2024::day21::part1, solvers_2024::day21::part2),
            22 => (solvers_2024::day22::part1, solvers_2024::day22::part2),
            23 => (solvers_2024::day23::part1, solvers_2024::day23::part2),
            24 => (solvers_2024::day24::part1, solvers_2024::day24::part2),
            25 => (solvers_2024::day25::part1, solvers_2024::day25::part2),
            _ => {
                todo!();
            }
        },
        2025 => {
            todo!();
        },
        _ => {
            todo!();
        }
    };
    (functions, input)
}
fn run_bench(day: usize, part: usize, f: SolverType, input: &str) {
    let timer = Instant::now();
    let mut run_count = 0;
    const MIN_TIME_MILLIS: u128 = 750;
    while timer.elapsed().as_millis() < MIN_TIME_MILLIS {
        black_box(f(black_box(input.to_string())));
        run_count += 1;
        if run_count > 3333 {
            break;
        }
    }
    let total_time = timer.elapsed().as_millis();
    let per_run_millis = total_time as f64 / run_count as f64;
    println!(
        "Day {day:2} part {part} benchmark: {run_count:6} runs in {total_time:3} ms at {per_run_millis:3.2} ms per run"
    );
}

#[derive(Parser, Debug)]
struct Args {
    // Which specific day to run
    #[arg(short, long, default_value=None)]
    day: Option<usize>,

    // Which specific year to run
    #[arg(short, long, default_value=None)]
    year: Option<usize>,

    // Run benchmarks instead of executing normally
    #[arg(short, long, default_value_t = false)]
    benchmark: bool,
}
fn main() {
    let args = Args::parse();

    let year = args.year.unwrap_or(2025);

    let from = args.day.unwrap_or(1);
    let to = args.day.unwrap_or(25);
    for day in from..=to {
        let ((p1, p2), input) = get_function_and_data(year, day);
        if args.benchmark {
            run_bench(day, 1, p1, &input);
            run_bench(day, 2, p2, &input);
            println!();
        } else {
            println!("Day {day:2} part 1: {}", p1(input.clone()));
            println!("Day {day:2} part 2: {}", p2(input.clone()));
        }
    }
}
