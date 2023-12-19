use itertools::Itertools;
use parser::{Instruction, Program, Terminal, Xmas};

use crate::parser::End;

pub mod parser {
    use itertools::Itertools;
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_while1},
        character::complete::{char, digit1},
        combinator::all_consuming,
        multi::{many0, separated_list1},
        sequence::tuple,
        IResult, Parser,
    };
    use regex::Replacer;

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
    fn parse_kv(input: &str) -> IResult<&str, (&str, usize)> {
        let (input, (key, _, value)) =
            tuple((take_while1(|c: char| c.is_alphabetic()), char('='), digit1))(input)?;
        let value = value.parse::<usize>().unwrap();
        Ok((input, (key, value)))
    }
}

fn evaluate_program(program: &Vec<Program>, memory: Xmas) -> usize {
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
                println!("Accepting program");
                let sum = memory.x + memory.m + memory.a + memory.s;
                return sum as usize;
            }
            Terminal::Stop(End::Reject) => {
                return 0;
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

// {x=787,m=2655,a=1222,s=2876}: in -> qqz -> qs -> lnx -> A
// {x=1679,m=44,a=2067,s=496}: in -> px -> rfg -> gd -> R
// {x=2036,m=264,a=79,s=2244}: in -> qqz -> hdj -> pv -> A
// {x=2461,m=1339,a=466,s=291}: in -> px -> qkq -> crn -> R
// {x=2127,m=1623,a=2188,s=1013}: in -> px -> rfg -> A

fn main() {
    let input = std::fs::read_to_string("input/19_easy.txt").unwrap();
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
        .map(|xmas| evaluate_program(&programs, xmas.clone()))
        .sum::<usize>();
    println!("The total result is {}", total_result);

    // let first_result = evaluate_program(&programs, xmases[0].clone());
    // println!("The first result is {}", first_result);

    // let lines = "px{a<2006:qkq,m>2090:A,rfg}
    // pv{a>1716:R,A}
    // lnx{m>1548:A,A}
    // rfg{s<537:gd,x>2440:R,A}
    // qs{s>3448:A,lnx}
    // qkq{x<1416:A,crn}
    // crn{x>2662:A,R}
    // in{s<1351:px,qqz}
    // qqz{s>2770:qs,m<1801:hdj,R}
    // gd{a>3333:R,R}
    // hdj{m>838:A,pv}";
    // let l = lines.lines().map_while(|line| {
    //     let line = line.trim();
    //     nom::combinator::all_consuming(parser::parse_program)(line)
    //         .ok()
    //         .map(|(_, program)| program)
    // });
    // let mut programs = l.collect_vec();
    // let xmas_line = "{x=787,m=2655,a=1222,s=2876}";
    // println!("{:?}", programs);
    // let xmas = parser::parse_xmas(xmas_line).unwrap().1;
    // println!("{:?}", xmas);
}
