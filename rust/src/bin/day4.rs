use std::{
  env, fs::File, io::{prelude::*, BufReader}, ops::Add, path::Path
};

use nom::{branch::alt, character::complete::one_of, error::VerboseError, multi::many0, IResult, Parser};

#[derive(Debug)]
struct Pos {
  x: usize,
  y: usize,
}

struct Size {
  w: isize,
  h: isize,
}

#[derive(Debug)]
struct Dir {
  dx: isize,
  dy: isize,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Letter {
  X,
  M,
  A,
  S,
}

const WORD_LENGTH: usize = 4;

const WORD: [Letter; WORD_LENGTH] = [
  Letter::X,
  Letter::M,
  Letter::A,
  Letter::S,
];

const DIRS: [Dir; 8] = [
  // Top
  Dir { dx: -1, dy: -1},
  Dir { dx: 0, dy: -1},
  Dir { dx: 1, dy: -1},

  // Left and Right
  Dir { dx: -1, dy: 0},
  Dir { dx: 1, dy: 0},

  // Bottom
  Dir { dx: -1, dy: 1},
  Dir { dx: 0, dy: 1},
  Dir { dx: 1, dy: 1},
];

fn main() {
  let args: Vec<String> = env::args().collect();
  let default_filename = "../inputs/day4.txt";
  let filename = args.get(1)
    .map_or(default_filename, |first_arg| first_arg.as_ref());

  let input = read_input(filename);

  dbg!(&input);

  // dbg!(within_bounds(&Size { w: 10, h: 10 }, &Pos { x: 3, y: 9 }, &Dir { dx: -1, dy: -1 }));

  println!("Part 1: {}", solve(&input));
  // println!("Part 2: {}", solve(input.clone(), true));
}



fn solve(input: &Vec<Vec<Letter>>) -> usize {
  let grid_size = Size { w: input[0].len() as isize, h: input.len() as isize };

  let mut word_count: usize = 0;

  for starty in 0..grid_size.h {
    for startx in 0..grid_size.w {
      let start = Pos { x: startx as usize, y: starty as usize };
      for dir in DIRS {
        let is_valid = is_valid_word(&input, &grid_size, &start, &dir);
        word_count += is_valid as usize;

        if is_valid {
          println!("{} {} {:2} {:2}", start.x, start.y, dir.dx, dir.dy);
        }
      }
    }
  }
  
  word_count
}

fn within_bounds(grid_size: &Size, start: &Pos, dir: &Dir) -> bool {
  let Size { w, h } = *grid_size;
  let Pos { x, y } = *start;
  let Dir { dx, dy } = *dir;
  let l  = WORD_LENGTH as isize;

  let lastx = (dx * (l - 1)).saturating_add_unsigned(x);
  let lasty = (dy * (l - 1)).saturating_add_unsigned(y);

  true
  // & (x >= 0)
  & ((x as isize) < w)
  // & (y >= 0)
  & ((y as isize) < h)
  & (lastx >= 0)
  & (lastx < w)
  & (lasty >= 0)
  & (lasty < h)
}

fn is_valid_word(grid: &Vec<Vec<Letter>>, grid_size: &Size, start: &Pos, dir: &Dir) -> bool {
  if !within_bounds(grid_size, start, dir) {
    return false;
  }

  let Pos {x: startx, y: starty} = start;
  let Dir {dx, dy} = dir;

  for i in 0..WORD_LENGTH {
    let letter = &WORD[i];

    let s_i = i as isize;

    // Wrapping because we already checked the indices are correct.
    let x = startx.wrapping_add_signed(s_i * dx);
    let y = starty.wrapping_add_signed(s_i * dy);

    if &grid[y][x] != letter {
      return false;
    }
  }

  true
}

fn read_input(filename: impl AsRef<Path>) -> Vec<Vec<Letter>> {
  let file = File::open(filename).expect("No such file");
  let buf = BufReader::new(file);
  buf.lines().map(|l| match l {
    Ok(s) => parse_line(&s).unwrap().1,
    Err(_e) => panic!("Could not parse line, error: ")
  }).collect()
}

fn parse_line<'a>(i: &'a str) -> IResult<&'a str, Vec<Letter>, VerboseError<&'a str>> {
  many0(parse_letter)(i)
}

fn parse_letter<'a>(i: &'a str) -> IResult<&'a str, Letter, VerboseError<&'a str>> {
  alt((
    nom::character::complete::char('X').map(|_| Letter::X),
    nom::character::complete::char('M').map(|_| Letter::M),
    nom::character::complete::char('A').map(|_| Letter::A),
    nom::character::complete::char('S').map(|_| Letter::S),
  ))(i)
}