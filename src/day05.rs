use lazy_static::lazy_static;
use regex::Regex;

type Crate = char;

const TOTAL_STACKS: usize = 9;
const NO_CRATE: Crate = 'X';

fn get_crate_at_index(row_string: &str, index: usize) -> Crate {
    // assume a crate def is exactly of type [x] (so three characters, separated by one space)
    let offset = index * 4;

    if index >= TOTAL_STACKS || offset > row_string.len() {
        return NO_CRATE;
    }

    let c = row_string.chars().nth(offset + 1).unwrap();

    if c == ' ' {
        NO_CRATE
    } else {
        c
    }
}

fn exec_command_crate_mover_9000(
    n: i32,
    from_index: usize,
    to_index: usize,
    stacks: &mut Vec<Vec<Crate>>,
) {
    for _ in 0..n {
        let c = stacks[from_index].pop().unwrap();
        stacks[to_index].push(c);
    }
}

fn exec_command_crate_mover_9001(
    n: i32,
    from_index: usize,
    to_index: usize,
    stacks: &mut Vec<Vec<Crate>>,
) {
    let mut crates: Vec<Crate> = (0..n)
        .into_iter()
        .map(|_| stacks[from_index].pop().unwrap())
        .collect();
    crates.reverse();
    crates.iter().for_each(|c| stacks[to_index].push(*c));
}

pub fn day05(input: String) -> String {
    let mut stacks: Vec<Vec<Crate>> = vec![vec![]; TOTAL_STACKS];
    let mut stacks_9001: Vec<Vec<Crate>> = vec![vec![]; TOTAL_STACKS];
    let lines: Vec<&str> = input.lines().collect();
    lazy_static! {
        static ref COMMAND_RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    }

    let mut reading_stacks = true;
    for line in lines {
        if line.trim().len() == 0 {
            continue;
        }

        if reading_stacks {
            let end_test = get_crate_at_index(line, 0);
            if end_test == '1' {
                reading_stacks = false;
                for i in 0..TOTAL_STACKS {
                    stacks[i].reverse();
                    stacks_9001[i] = stacks[i].clone();
                }
                continue;
            }

            for i in 0..TOTAL_STACKS {
                let c = get_crate_at_index(line, i);
                if c != NO_CRATE {
                    stacks[i].push(c);
                }
            }
        } else {
            for cap in COMMAND_RE.captures_iter(line) {
                let n = cap[1].parse::<i32>().unwrap();
                let from_index = cap[2].parse::<usize>().unwrap() - 1;
                let to_index = cap[3].parse::<usize>().unwrap() - 1;
                exec_command_crate_mover_9000(n, from_index, to_index, &mut stacks);
                exec_command_crate_mover_9001(n, from_index, to_index, &mut stacks_9001);
            }
        }
    }

    let res_9000: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();
    let res_9001: String = stacks_9001
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect();
    return format!("Result 9000: {}\nResult 9001: {}", res_9000, res_9001);
}
