#[derive(Debug, PartialEq)]
pub enum CoreExprError {
  EmptyInput,
  InvalidPattern,
  InvalidTumorPercent,
  MissingTumorPercent,
  PatternOutOfRange,
}

#[derive(Debug, PartialEq)]
pub struct CoreExpr {
  pub p3: u8,
  pub p4: u8,
  pub p5: u8,
  pub tumor_pct: u8,
}

pub fn parse(input: &str) -> Result<CoreExpr, CoreExprError> {
  todo!();
}