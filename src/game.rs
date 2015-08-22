use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
enum CellState {
  Empty,
  X,
  O,
}

impl fmt::Display for CellState {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match *self {
      CellState::Empty => " ",
      CellState::X => "X",
      CellState::O => "O",
    };
    write!(f, "{}", s)
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PlayerStatus {
  Winner,
  Loser,
  Draw,
  Indeterminate,
}

#[derive(Debug, Clone)]
pub struct GameState {
  state: Vec<CellState>,
}

fn get_index(r: u32, c: u32) -> usize {
  let r = r as usize;
  let c = c as usize;
  r * 3 + c
}

impl GameState {
  pub fn new() -> GameState {
    let mut state = Vec::with_capacity(3 * 3);
    for _ in 0..9 {
      state.push(CellState::Empty);
    }
    GameState { state: state }
  }

  pub fn print(&self) {
    for r in 0..3 {
      for c in 0..3 {
        print!("|{}|", self.state.get(get_index(r, c)).unwrap());
      }
      println!("");
      if r != 2 {
        println!("=========");
      }
    }
  }

  pub fn make_move(&mut self, r: u32, c: u32) -> bool {
    if let Some(cell) = self.state.get_mut(get_index(r, c)) {
      if *cell == CellState::Empty {
        *cell = CellState::X;
        true
      } else {
        false
      }
    } else {
      false
    }
  }

  pub fn check_winner(&self) -> PlayerStatus {
    // columns
    for c in 0..3 {
      let mut winner = true;
      let mut loser = true;
      for r in 0..3 {
        for cell in self.state.get(get_index(r, c)) {
          match *cell {
            CellState::X => {
              loser = false;
              winner = winner && true;
            },
            CellState::O => {
              winner = false;
              loser = loser && true;
            },
            CellState::Empty => {
              winner = false;
              loser = false;
            },
          }
        }
      }
      if winner { return PlayerStatus::Winner }
      else if loser { return PlayerStatus::Loser }
    }

    // rows
    for r in 0..3 {
      let mut winner = true;
      let mut loser = true;
      for c in 0..3 {
        for cell in self.state.get(get_index(r, c)) {
          match *cell {
            CellState::X => {
              loser = false;
              winner = winner && true;
            },
            CellState::O => {
              winner = false;
              loser = loser && true;
            },
            CellState::Empty => {
              winner = false;
              loser = false;
            },
          }
        }
      }
      if winner { return PlayerStatus::Winner }
      else if loser { return PlayerStatus::Loser }
    }

    // diagnal 1
    let mut winner = true;
    let mut loser = true;
    for r in 0..3 {
      for cell in self.state.get(get_index(r, r)) {
        match *cell {
          CellState::X => {
            loser = false;
            winner = winner && true;
          },
          CellState::O => {
            winner = false;
            loser = loser && true;
          },
          CellState::Empty => {
            winner = false;
            loser = false;
          },
        }
      }
    }
    if winner { return PlayerStatus::Winner }
    else if loser { return PlayerStatus::Loser }

    // diagnal 2
    let mut winner = true;
    let mut loser = true;
    for r in 0..3 {
      for cell in self.state.get(get_index(r, 3 - r)) {
        match *cell {
          CellState::X => {
            loser = false;
            winner = winner && true;
          },
          CellState::O => {
            winner = false;
            loser = loser && true;
          },
          CellState::Empty => {
            winner = false;
            loser = false;
          },
        }
      }
    }
    if winner { return PlayerStatus::Winner }
    else if loser { return PlayerStatus::Loser }

    // full but neither winner nor loser
    let mut draw = true;
    for cell in &self.state {
      if *cell == CellState::Empty {
        draw = false;
      }
    }
    if draw { return PlayerStatus::Draw }

    PlayerStatus::Indeterminate
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use super::CellState::*;
  use super::PlayerStatus::*;

  #[test]
  fn indeterminate1() {
    let E = Empty;
    let mut state = GameState::new();
    state.state = vec!(
      E,E,E,
      E,E,E,
      E,E,E);
    assert_eq!(state.check_winner(), Indeterminate);
  }

  #[test]
  fn winner1() {
    let E = Empty;
    let mut state = GameState::new();
    state.state = vec!(
      X,X,X,
      E,E,E,
      E,E,E);
    assert_eq!(state.check_winner(), Winner);
  }

  #[test]
  fn loser1() {
    let E = Empty;
    let mut state = GameState::new();
    state.state = vec!(
      O,O,O,
      E,E,E,
      E,E,E);
    assert_eq!(state.check_winner(), Loser);
  }

  #[test]
  fn draw() {
    let E = Empty;
    let mut state = GameState::new();
    state.state = vec!(
      O,X,O,
      X,X,O,
      O,O,X);
    assert_eq!(state.check_winner(), Draw);
  }
}
