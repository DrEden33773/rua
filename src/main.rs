use std::{env::args, fs::File};

use rua::{parse, utils::New, vm};

fn main() {
  let args = args().collect::<Vec<_>>();
  if args.len() != 2 {
    println!("Usage: {} script", args[0]);
    return;
  }

  let file = File::open(&args[1]).unwrap();
  let proto = parse::ParseProto::load(file);
  vm::ExeState::new().execute(&proto);
}

#[cfg(test)]
mod simple_test {
  use super::*;

  fn open_file(path: &str) -> File {
    File::open(
      project_root::get_project_root()
        .expect("no project root found")
        .to_str()
        .unwrap()
        .to_owned()
        + path,
    )
    .unwrap()
  }

  #[test]
  fn hello_world() {
    // project_root/examples/hello_world.rua
    // get project root via `project_root`
    let file = open_file("/examples/hello_world.lua");
    let proto = parse::ParseProto::load(file);
    vm::ExeState::new().execute(&proto);
  }
}
