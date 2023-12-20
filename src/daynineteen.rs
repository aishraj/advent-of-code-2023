use itertools::Itertools;
use parser::{Instruction, Program, Terminal, Xmas};

use self::parser::End;

pub mod parser {

    use nom::{
        branch::alt,
        bytes::complete::{tag, take_while1},
        character::complete::{char, digit1},
        multi::separated_list1,
        sequence::tuple,
        IResult, Parser,
    };

    #[derive(Debug, Clone)]
    pub(crate) enum Instruction {
        Check((Comparison, Terminal)),
        Terminal(Terminal),
    }

    #[derive(Debug, Clone)]
    pub(crate) struct Comparison {
        pub(crate) variable: String,
        pub(crate) operator: char,
        pub(crate) value: i32,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub(crate) enum Terminal {
        Goto(String),
        Stop(End),
    }

    #[derive(Debug, Clone)]
    pub struct Xmas {
        pub x: usize,
        pub m: usize,
        pub a: usize,
        pub s: usize,
    }

    impl Xmas {
        pub fn get(&self, value: &str) -> usize {
            match value {
                "x" => self.x,
                "m" => self.m,
                "a" => self.a,
                "s" => self.s,
                _ => unreachable!(),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub(crate) enum End {
        Accept,
        Reject,
    }

    #[derive(Debug, Clone)]
    pub(crate) struct Program {
        pub(crate) name: String,
        pub(crate) instructions: Vec<Instruction>,
    }

    // a body is a comma separated list of instructions
    fn parse_body(input: &str) -> IResult<&str, Vec<Instruction>> {
        separated_list1(tag(","), parse_instruction_or_terminal)(input)
    }

    fn parse_terminal(input: &str) -> IResult<&str, Instruction> {
        alt((parse_stop, parse_goto))(input)
    }

    fn parse_goto(input: &str) -> IResult<&str, Instruction> {
        let (input, next_state) = take_while1(|c: char| c.is_alphabetic())(input)?;
        return Ok((
            input,
            Instruction::Terminal(Terminal::Goto(next_state.to_string())),
        ));
    }

    fn parse_stop(input: &str) -> IResult<&str, Instruction> {
        let (input, end) = alt((tag("A"), tag("R")))(input)?;
        let end = match end {
            "A" => End::Accept,
            "R" => End::Reject,
            _ => unreachable!(),
        };
        return Ok((input, Instruction::Terminal(Terminal::Stop(end))));
    }

    pub(crate) fn parse_program(input: &str) -> IResult<&str, Program> {
        let (input, (name, _, instructions, _)) =
            tuple((parse_name, char('{'), parse_body, char('}')))(input)?;

        Ok((
            input,
            Program {
                name: name.to_string(),
                instructions,
            },
        ))
    }

    fn parse_name(input: &str) -> IResult<&str, &str> {
        take_while1(|c: char| c.is_alphabetic())(input)
    }

    fn parse_instruction_or_terminal(input: &str) -> IResult<&str, Instruction> {
        alt((parse_instruction, parse_terminal))(input)
    }

    fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
        let (input, (variable, operator, value, _, next_state)) = tuple((
            take_while1(|c: char| c.is_alphabetic()),
            char('<').or(char('>')),
            digit1,
            char(':'),
            take_while1(|c: char| c.is_alphabetic()),
        ))(input)?;

        let value = value.parse::<i32>().unwrap();

        let next_state = match next_state {
            "A" => Terminal::Stop(End::Accept),
            "R" => Terminal::Stop(End::Reject),
            _ => Terminal::Goto(next_state.to_string()),
        };

        let comparison = Comparison {
            variable: variable.to_string(),
            operator,
            value,
        };
        return Ok((input, Instruction::Check((comparison, next_state))));
    }

    /// Pasers the input of the form
    /// {x=787,m=2655,a=1222,s=2876}
    pub(crate) fn parse_xmas(input: &str) -> IResult<&str, Xmas> {
        let (input, (_, x, _, m, _, a, _, s, _)) = tuple((
            char('{'),
            parse_kv,
            char(','),
            parse_kv,
            char(','),
            parse_kv,
            char(','),
            parse_kv,
            char('}'),
        ))(input)?;

        Ok((
            input,
            Xmas {
                x: x.1,
                m: m.1,
                a: a.1,
                s: s.1,
            },
        ))
    }

    // parses the input of the form
    // key=1223
    pub fn parse_kv(input: &str) -> IResult<&str, (&str, usize)> {
        let (input, (key, _, value)) =
            tuple((take_while1(|c: char| c.is_alphabetic()), char('='), digit1))(input)?;
        let value = value.parse::<usize>().unwrap();
        Ok((input, (key, value)))
    }
}

use std::collections::HashMap;

// Based on https://github.com/mebeim/aoc/blob/master/2023/README.md#day-19---aplenty

fn recursive_count(
    workflows: &HashMap<String, Vec<Instruction>>,
    ranges: &mut HashMap<String, (i64, i64)>,
    cur: &Terminal,
) -> i64 {
    match cur {
        Terminal::Stop(End::Accept) => ranges.values().map(|&(lo, hi)| hi - lo + 1).product(),
        Terminal::Stop(End::Reject) => 0,
        Terminal::Goto(label) => {
            let instructions = workflows.get(label).unwrap();
            let mut total = 0;

            for instruction in instructions {
                match instruction {
                    Instruction::Check((comparison, next_state)) => {
                        println!("var is {}", comparison.variable);
                        let (lo, hi) = ranges.get(&comparison.variable).unwrap().clone();
                        if comparison.operator == '<' {
                            if lo < comparison.value.into() {
                                let mut next_ranges = ranges.clone();
                                next_ranges.insert(
                                    comparison.variable.clone(),
                                    (lo, (comparison.value - 1).into()),
                                );
                                total += recursive_count(workflows, &mut next_ranges, next_state);
                            }
                            ranges.insert(
                                comparison.variable.clone(),
                                (std::cmp::max(lo, comparison.value.into()), hi),
                            );
                        } else {
                            if hi > comparison.value.into() {
                                let mut next_ranges = ranges.clone();
                                next_ranges.insert(
                                    comparison.variable.clone(),
                                    ((comparison.value + 1).into(), hi),
                                );
                                total += recursive_count(workflows, &mut next_ranges, next_state);
                            }
                            ranges.insert(
                                comparison.variable.clone(),
                                (lo, std::cmp::min(hi, comparison.value.into())),
                            );
                        }
                    }
                    Instruction::Terminal(Terminal::Goto(pos)) => {
                        total += recursive_count(
                            workflows,
                            &mut ranges.clone(),
                            &Terminal::Goto(pos.clone()),
                        );
                    }
                    Instruction::Terminal(term) => {
                        total += recursive_count(workflows, &mut ranges.clone(), term);
                    }
                }
            }

            total
        }
    }
}

pub fn solve_part_two(input: &str) -> i64 {
    let parts = input.split("\n\n").collect_vec();
    let programs = parts[0]
        .lines()
        .map_while(|line| {
            let line = line.trim();
            nom::combinator::all_consuming(parser::parse_program)(line)
                .ok()
                .map(|(_, program)| program)
        })
        .collect_vec();
    let mut ranges = HashMap::new();
    ranges.insert("x".to_string(), (1, 4000));
    ranges.insert("m".to_string(), (1, 4000));
    ranges.insert("a".to_string(), (1, 4000));
    ranges.insert("s".to_string(), (1, 4000));
    let workflows = programs
        .iter()
        .map(|program| (program.name.clone(), program.instructions.clone()))
        .collect::<HashMap<String, Vec<Instruction>>>();
    recursive_count(&workflows, &mut ranges, &Terminal::Goto("in".to_string()))
}

pub(crate) fn evaluate_program(program: &Vec<Program>, memory: &Xmas) -> bool {
    //convert the program to a hashmap
    let program = program
        .iter()
        .map(|program| (program.name.clone(), program.clone()))
        .collect::<std::collections::HashMap<String, Program>>();
    let mut state = Terminal::Goto("in".to_string());

    loop {
        //println!("State is {:?}", state);
        match state {
            Terminal::Stop(End::Accept) => {
                //println!("Accepting program");
                return true;
            }
            Terminal::Stop(End::Reject) => {
                return false;
            }
            Terminal::Goto(ref label) => {
                println!("Evaluating program {}", label);
                let program = program.get(label).unwrap();
                for instruction in program.instructions.iter() {
                    //println!("Instruction is {:?}", instruction);
                    match instruction {
                        Instruction::Check((comparison, next_state)) => {
                            let variable = comparison.variable.clone();
                            let value = memory.get(&variable) as i64;
                            let operator = comparison.operator;
                            let comparison_value = comparison.value;
                            let next_state = next_state.clone();
                            if operator == '<' {
                                if value < comparison_value.into() {
                                    state = next_state;
                                    break;
                                } else {
                                    continue;
                                }
                            } else if operator == '>' {
                                if value > comparison_value.into() {
                                    state = next_state;
                                    break;
                                } else {
                                    continue;
                                }
                            }
                        }
                        Instruction::Terminal(terminal) => {
                            state = terminal.clone();
                        }
                    };
                }
            }
        }
    }
}

fn solve_part_one(input: &str) -> usize {
    let parts = input.split("\n\n").collect_vec();
    let programs = parts[0]
        .lines()
        .map_while(|line| {
            let line = line.trim();
            nom::combinator::all_consuming(parser::parse_program)(line)
                .ok()
                .map(|(_, program)| program)
        })
        .collect_vec();
    let xmases = parts[1]
        .lines()
        .map_while(|line| {
            let line = line.trim();
            nom::combinator::all_consuming(parser::parse_xmas)(line)
                .ok()
                .map(|(_, xmas)| xmas)
        })
        .collect_vec();
    // println!("{:?}", programs);
    // println!("{:?}", xmases);

    let total_result = xmases
        .iter()
        .filter(|xmas| evaluate_program(&programs, xmas))
        .map(|xmas| xmas.x + xmas.m + xmas.a + xmas.s)
        .sum::<usize>();
    println!("The total result is {}", total_result);
    total_result
}

#[cfg(test)]
mod tests {
    #[test]
    fn solves_19_1_easy() {
        let input = std::fs::read_to_string("input/19_easy.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 19114);
    }

    #[test]
    fn solves_19_1_hard() {
        let input = std::fs::read_to_string("input/19_real.txt").unwrap();
        assert_eq!(super::solve_part_one(&input), 492702);
    }

    #[test]
    fn solves_19_2_easy() {
        let input = std::fs::read_to_string("input/19_easy.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 167409079868000);
    }

    #[test]
    fn solves_19_2_hard() {
        let input = std::fs::read_to_string("input/19_real.txt").unwrap();
        assert_eq!(super::solve_part_two(&input), 138616621185978);
    }
}
