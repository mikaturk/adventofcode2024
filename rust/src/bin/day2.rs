use std::{
  env, fs::File, io::{prelude::*, BufReader}, path::Path
};

use nom::{character::complete::{self, char}, error::VerboseError, multi::separated_list1, IResult};

fn main() {
  let args: Vec<String> = env::args().collect();
  let default_filename = "../inputs/day2.txt";
  let filename = args.get(1)
    .map_or(default_filename, |first_arg| first_arg.as_ref());

  let input = read_input(filename);

  println!("Part 1: {}", solve(input.clone(), false));
  println!("Part 2: {}", solve(input.clone(), true));
}

fn solve(input: Vec<Vec<u32>>, allow_failure: bool) -> u32 {
  let x = input.into_iter().map(|x| is_safe(x, allow_failure));

  // dbg!(x.clone().zip(0..).collect::<Vec<_>>());
  x.clone().filter(|c| *c).count() as u32
}

fn is_safe(report: Vec<u32>, allow_failure: bool) -> bool {
  all_windowed_opt_failure(&report, |x,y| (1..=3).contains(&(x.wrapping_sub(y))), allow_failure)
  || all_windowed_opt_failure(&report, |x,y| (1..=3).contains(&(y.wrapping_sub(x))), allow_failure)
}

fn all_windowed_opt_failure(report: &[u32], f: impl Fn(u32, u32) -> bool, allow_failure: bool) -> bool {
  let mut have_allowed_failure = !allow_failure;

  let mut iter = 0..report.len()-1;

  while let Some(i) = iter.next() {
    let x = report[i];
    let y = report[i+1];
    
    if f(x, y) {
      continue
    }

    if have_allowed_failure {
      return false;
    }

    if i == report.len()-2 {
      continue
    }

    have_allowed_failure = true;

    if f(x, report[i+2]) {
      // Skip over y
      iter.next();
    }
  }

  true
}

fn read_input(filename: impl AsRef<Path>) -> Vec<Vec<u32>> {
  let file = File::open(filename).expect("No such file");
  let buf = BufReader::new(file);
  buf.lines().map(|l| match l {
    Ok(s) => parse_line(&s).unwrap().1,
    Err(_e) => panic!("Could not parse line, error: ")
  }).collect()
}

fn parse_line<'a>(input: &'a str) -> IResult<&'a str, Vec<u32>, VerboseError<&'a str>> {
  separated_list1(char(' '), complete::u32)(input)
}