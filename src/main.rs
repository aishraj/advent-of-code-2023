use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, digit1},
    multi::{many0, separated_list1},
    sequence::tuple,
    IResult, Parser,
};

#[derive(Debug, Clone)]
enum Instruction {
    Check((Comparison, Terminal)),
    Terminal(Terminal),
}

#[derive(Debug, Clone)]
struct Comparison {
    variable: String,
    operator: char,
    value: i32,
}

#[derive(Debug, Clone)]
enum Terminal {
    Goto(String),
    Stop(End),
}

#[derive(Debug, Clone)]
enum End {
    Accept,
    Reject,
}

#[derive(Debug, Clone)]
struct Program {
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
    return Ok((
        input,
        Instruction::Terminal(Terminal::Goto(input.to_string())),
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

fn parse_program(input: &str) -> IResult<&str, Program> {
    let (input, (name, _, instructions, _)) = tuple((
        parse_name,
        char('{'),
        separated_list1(tag(","), parse_instruction_or_terminal),
        char('}'),
    ))(input)?;

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

fn parse_instruction(input: &str) -> IResult<&str, crate::Instruction> {
    let (input, (variable, operator, value, _, next_state)) = tuple((
        take_while1(|c: char| c.is_alphabetic()),
        char('<').or(char('>')),
        digit1,
        char(':'),
        take_while1(|c: char| c.is_alphabetic()),
    ))(input)?;

    let value = value.parse::<i32>().unwrap();

    let (_remaining_input, terminal) = parse_terminal(next_state)?;
    //println!("The remaining_input input is: {:?}", remaining_input);
    if let Instruction::Terminal(term) = terminal {
        return Ok((
            input,
            Instruction::Check((
                Comparison {
                    variable: variable.to_string(),
                    operator,
                    value,
                },
                term,
            )),
        ));
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Char,
        )))
    }
}

fn main() {
    // another: px{a<2006:qkq,m>2090:A,rfg}
    let inp = "qqz{s>2770:qs,m<1801:hdj,R}";
    //let partial_input = "a<2006:qkq,m>2090:A,rfg";
    println!("{:?}", parse_program(inp));
}
