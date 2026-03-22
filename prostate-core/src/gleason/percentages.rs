use std::ops::Index;

use super::errors::PercentageError;
use super::pattern::Pattern;

#[derive(Debug, Clone, Copy)]
pub struct PatternPercentages {
  p3: u8,
  p4: u8,
  p5: u8,
}

impl PatternPercentages {
  pub fn new(p3: u8, p4: u8, p5: u8) -> Result<Self, PercentageError> {
      if p3 + p4 + p5 != 100 {
        return Err(PercentageError::SumNotHundred);
      }

      Ok(Self { p3, p4, p5 })
  }
}

impl Index<Pattern> for PatternPercentages {
  type Output = u8;

  fn index(&self, pattern: Pattern) -> &Self::Output {
    match pattern {
      Pattern::Three => &self.p3,
      Pattern::Four => &self.p4,
      Pattern::Five => &self.p5,
    }
  }
}