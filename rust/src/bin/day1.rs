use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
  env,
};


fn main() {
  let args: Vec<String> = env::args().collect();
  let default_filename = "../inputs/day1.txt";
  let filename = args.get(1)
    .map_or(default_filename, |first_arg| first_arg.as_ref());

  println!("Loading Day 1 from {}", filename);
}
