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
