use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("inputs/day5.txt").unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let mut stacks = parse_initial_stacks(&mut lines);
    lines.next();

    for line in lines {
        if let Ok(line) = line {
            let (amount, from, to) = parse_move(&line);

            for _ in 0..amount {
                let c = { stacks[from].pop().unwrap() };
                stacks[to].push(c);
            }
        }
    }

    let top: String = stacks.iter().map(|x| *x.last().unwrap()).collect();
    println!("Solution to part 1: {}", top);
}

fn part2() {
    let file = File::open("inputs/day5.txt").unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let mut stacks = parse_initial_stacks(&mut lines);
    lines.next();

    for line in lines {
        if let Ok(line) = line {
            let (amount, from, to) = parse_move(&line);
            let mut popped_crates = {
                let from_stack = &mut stacks[from];
                from_stack.split_off(from_stack.len() - amount)
            };

            let to_stack = &mut stacks[to];
            to_stack.append(&mut popped_crates);
        }
    }

    let top: String = stacks.iter().map(|x| *x.last().unwrap()).collect();
    println!("Solution to part 2: {}", top);
}

fn parse_move(m: &str) -> (usize, usize, usize) {
    let instruction: Vec<&str> = m.split(' ').collect();
    (
        instruction[1].parse().unwrap(),
        instruction[3].parse::<usize>().unwrap() - 1,
        instruction[5].parse::<usize>().unwrap() - 1,
    )
}

fn parse_initial_stacks<B>(lines: &mut Lines<B>) -> Vec<Vec<char>>
where
    B: BufRead,
{
    let stack_lines: Vec<String> = lines
        .by_ref()
        .take_while(|x| x.as_ref().unwrap().contains('['))
        .map(|r| r.unwrap())
        .collect();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    let first = &stack_lines[0];
    for _ in 0..partition(first, 4).len() {
        stacks.push(vec![]);
    }

    for line in stack_lines {
        let segments = partition(&line, 4);
        for (i, c) in segments.iter().enumerate() {
            if !c.trim().is_empty() {
                stacks[i].insert(0, c.chars().nth(1).unwrap());
            }
        }
    }
    stacks
}

fn partition(s: &str, n: usize) -> Vec<String> {
    s.chars()
        .collect::<Vec<char>>()
        .chunks(n)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
}
