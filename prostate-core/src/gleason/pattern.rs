#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pattern {
  Three,
  Four,
  Five
}

impl Pattern {
  pub fn value(self) -> u8 {
    match self {
      Pattern::Three => 3,
      Pattern::Four => 4,
      Pattern::Five => 5
    }
  }

  pub fn all() -> [Pattern; 3] {
    [
      Pattern::Three,
      Pattern::Four,
      Pattern::Five
    ]
  }

  pub fn descending() -> [Pattern; 3] {
    [
      Pattern::Five,
      Pattern::Four,
      Pattern::Three,
    ]
  }
}