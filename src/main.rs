use once_cell::sync::Lazy;
use rua::{parse, utils::New, vm};
use std::{env::args, fs::File, io::BufReader};

static PROJECT_ROOT: Lazy<String> = Lazy::new(|| {
  project_root::get_project_root()
    .expect("no project root found")
    .to_str()
    .unwrap()
    .to_owned()
});

fn open_file(path: &str) -> File {
  let complete_filepath = PROJECT_ROOT.to_string() + path;
  File::open(complete_filepath).unwrap()
}

fn main() {
  let args = args().collect::<Vec<_>>();
  if args.len() != 2 {
    println!("Usage: {} script", args[0]);
    return;
  }

  let file = open_file(&args[1]);
  let proto = parse::ParseProto::load(BufReader::new(file));
  vm::ExeState::new().execute(&proto);
}

#[cfg(test)]
mod simple_test {
  use super::*;

  fn open_file(path: &str) -> File {
    File::open(PROJECT_ROOT.to_owned() + path).unwrap()
  }

  #[test]
  fn hello_world() {
    let file = open_file("/examples/hello_world.lua");
    vm::ExeState::new().execute(&parse::ParseProto::load(file));
  }

  #[test]
  fn print_single_argument() {
    let file = open_file("/examples/print_single_arg.lua");
    vm::ExeState::new().execute(&parse::ParseProto::load(file));
  }

  #[test]
  fn scientific_notation() {
    let file = open_file("/examples/scientific_notation.lua");
    vm::ExeState::new().execute(&parse::ParseProto::load(file));
  }
}
