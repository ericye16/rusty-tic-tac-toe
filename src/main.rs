mod game;
use game::*;

use std::io;
use std::str::FromStr;

fn read_number() -> Result<u32, <u32 as FromStr>::Err> {
  let mut n = String::new();
  io::stdin().read_line(&mut n)
    .ok()
    .expect("failed to read line");

  n.trim().parse()
}

fn main() {
  let mut game = GameState::new();
  loop {
    println!("row:");
    let row = match read_number() {
      Ok(num) if num >= 1 && num <= 3 => num - 1,
      _ => continue,
    };

    println!("col:");
    let col = match read_number() {
      Ok(num) if num >= 1 && num <= 3 => num - 1,
      _ => continue,
    };

    if !game.make_move(row, col) {
      println!("Bad move!");
      continue;
    }

    println!("You chose: row {} column {}", row + 1, col + 1);
    game.print();
  }
}
