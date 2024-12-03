use std::{
  env, fs::{self}
};

use nom::{branch::alt, bytes::complete::tag, character::complete::{anychar, one_of}, combinator::{map_res, recognize}, error::VerboseError, multi::{many0, many_m_n, many_till}, sequence::{preceded, separated_pair, terminated}, IResult, Parser};

#[derive(Clone, Debug)]
enum Operator {
  MulOp(u16, u16),
  DoOp,
  DontOp,
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let default_filename = "../inputs/day3.txt";
  let filename = args.get(1)
    .map_or(default_filename, |first_arg| first_arg.as_ref());

  let s = fs::read_to_string(filename).expect("Reading failed");

  let (_, mul_ops) = parse_file_part1(&s).expect("Parsing part 1 failed");
  let (_, ops) = parse_file_part2(&s).expect("Parsing part 2 failed");

  // dbg!(mul_ops.clone());
  // dbg!(ops.clone());

  println!("Part 1: {}", part1(mul_ops.clone()));
  println!("Part 2: {}", part2(ops.clone()));
}

fn part1(mul_ops: Vec<Operator>) -> usize {
  mul_ops.into_iter().map(
    |op| match op {
      Operator::MulOp(a, b) => a as usize * b as usize,
      _ => 0,
    }
  ).sum()
}

fn part2(ops: Vec<Operator>) -> usize {
  ops.into_iter().fold(
    (0, true),
    |(total, enabled), op| match op {
      Operator::MulOp(a, b) => (
        total + if enabled { a as usize * b as usize } else { 0 },
        enabled,
      ),
      Operator::DoOp => (total, true),
      Operator::DontOp => (total, false),
    }
  ).0
}

fn mul_input<'a>(i: &'a str) -> IResult<&'a str, u16, VerboseError<&'a str>> {
  map_res(
    recognize(many_m_n(1, 3, one_of("0123456789"))),
    |s| u16::from_str_radix(s, 10)
  )(i)
}

fn mul_op<'a>(i: &'a str) -> IResult<&'a str, Operator, VerboseError<&'a str>> {
  terminated(
    preceded(
      tag("mul("),
      separated_pair(mul_input, tag(","), mul_input)
    ),
    tag(")"),
  ).map(|(a, b)| Operator::MulOp(a, b)).parse(i)
}

fn part2_op<'a>(i: &'a str) -> IResult<&'a str, Operator, VerboseError<&'a str>> {
  alt((
    mul_op,
    tag("do()").map(|_| Operator::DoOp),
    tag("don't()").map(|_| Operator::DontOp),
  ))(i)
}

fn garbage_then_part2_op<'a>(i: &'a str) -> IResult<&'a str, Operator, VerboseError<&'a str>> {
  many_till(anychar, part2_op).map(|(_, x)| x).parse(i)
}

fn garbage_then_part1_op<'a>(i: &'a str) -> IResult<&'a str, Operator, VerboseError<&'a str>> {
  many_till(anychar, mul_op).map(|(_, x)| x).parse(i)
}

fn parse_file_part1<'a>(i: &'a str) -> IResult<&'a str, Vec<Operator>, VerboseError<&'a str>> {
  terminated(
    many0(garbage_then_part1_op),
    many0(anychar)
  )(i)
}

fn parse_file_part2<'a>(i: &'a str) -> IResult<&'a str, Vec<Operator>, VerboseError<&'a str>> {
  terminated(
    many0(garbage_then_part2_op),
    many0(anychar)
  )(i)
}