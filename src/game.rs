#[derive(Debug, PartialEq, Clone)]
enum CellState {
  Empty,
  X,
  O,
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
    for i in 0..9 {
      state.push(CellState::Empty);
    }
    GameState { state: state }
  }

  pub fn get_state_at(&self, r: u32, c: u32) -> Option<&CellState> {
    self.state.get(get_index(r, c))
  }

  pub fn print(&self) {
    fn cell_str(cell_state: Option<&CellState>) -> &str {
      match cell_state {
        Some(c) => {
          match *c {
            CellState::Empty => " ",
            CellState::X => "X",
            CellState::O => "O",
          }
        }
        None => " "
      }
    }

    for r in 0..3 {
      for c in 0..3 {
        print!("|{}|", cell_str(self.get_state_at(r, c)));
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
        if let Some(cell) = self.get_state_at(r, c) {
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

    PlayerStatus::Indeterminate
  }
}
