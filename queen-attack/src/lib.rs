#[derive(Debug)]
pub struct ChessPosition {
  y: u32, x: u32
}

#[derive(Debug)]
pub struct Queen {
  pos: ChessPosition
}

impl ChessPosition {
  pub fn new(rank: i32, file: i32) -> Option<Self> {
    if rank < 0 || rank > 7 || file < 0 || file > 7 { return None; }
    Some(ChessPosition {y: rank as u32, x: file as u32 })
  }
}

impl Queen {
  pub fn new(position: ChessPosition) -> Self {
    Queen { pos: position }
  }

  pub fn can_attack(&self, other: &Queen) -> bool {
    let x_diff = (self.pos.x as i32 - other.pos.x as i32).abs();
    let y_diff = (self.pos.y as i32 - other.pos.y as i32).abs();

    return x_diff == 0 || y_diff == 0 || x_diff == y_diff
  }
}
