use std::env;

pub fn run() {
  let args: Vec<String> = env::args().collect();
  let command = args[1].clone();
  let name = "Brad";
  let status = "100%";

  // println!("Command: {}", command);

  if command == "hello" {
    println!("Hi {}, how are you?", name);
  } else if command == "status" {
    println!("Status is {}", status);
  } else {
    println!("That is not a valid command");
  }
}
