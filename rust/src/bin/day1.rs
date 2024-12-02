use std::{
  collections::HashMap, env, fs::File, io::{prelude::*, BufReader}, path::Path
};

use nom::{bytes::complete::tag, character::complete, error::VerboseError, sequence::separated_pair};

fn main() {
  let args: Vec<String> = env::args().collect();
  let default_filename = "../inputs/day1.txt";
  let filename = args.get(1)
    .map_or(default_filename, |first_arg| first_arg.as_ref());

  let lists = read_lists(filename);

  println!("Part 1: {}", part1(lists.clone()));
  println!("Part 2: {}", part2(lists.clone()));
}

fn part1(lists: (Vec<u32>, Vec<u32>)) -> u32 {
  let (mut left, mut right) = lists;

  left.sort();
  right.sort();

  left.into_iter().zip(right).map(|(l, r)| l.abs_diff(r)).sum::<u32>()
}

fn part2(lists: (Vec<u32>, Vec<u32>)) -> u32 {
  let (left, right) = lists;

  let mut counts = HashMap::new();   

  right.into_iter().for_each(|i| {
    let v= counts.remove(&i).unwrap_or(0);

    counts.insert(i, v + 1);
  });
  
  left.into_iter().map(|l| {
    l * counts.get(&l).unwrap_or(&0)
  }).sum()
}

fn read_lists(filename: impl AsRef<Path>) -> (Vec<u32>, Vec<u32>) {
  let file = File::open(filename).expect("No such file");
  let buf = BufReader::new(file);
  buf.lines().map(|l| match l {
    Ok(s) => parse_line(&s).unwrap(),
    Err(_e) => panic!("Could not parse line, error: ")
  }).unzip()
}

fn parse_line<'a>(input: &'a str) -> Result<(u32, u32), nom::Err<VerboseError<&'a str>>> {
  Ok(separated_pair(
    complete::u32,
    tag("   "),
    complete::u32)(input)?.1)
}