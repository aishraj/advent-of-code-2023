use itertools::Itertools;

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
        variable: String,
        operator: char,
        value: i32,
    }

    #[derive(Debug, Clone)]
    pub(crate) enum Terminal {
        Goto(String),
        Stop(End),
    }

    #[derive(Debug, Clone)]
    pub(crate) struct Xmas {
        x: usize,
        m: usize,
        a: usize,
        s: usize,
    }

    #[derive(Debug, Clone)]
    pub(crate) enum End {
        Accept,
        Reject,
    }

    #[derive(Debug, Clone)]
    pub(crate) struct Program {
        name: String,
        instructions: Vec<Instruction>,
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
    println!("{:?}", programs);
    println!("{:?}", xmases);
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
