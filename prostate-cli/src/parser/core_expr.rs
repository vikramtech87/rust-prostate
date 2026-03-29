#[derive(Debug, PartialEq)]
pub enum CoreExprError {
  EmptyInput,
  InvalidPattern,
  InvalidTumorPercent,
  MissingTumorPercent,
}

#[derive(Debug, PartialEq)]
pub struct CoreExpr {
  pub p3: u8,
  pub p4: u8,
  pub p5: u8,
  pub tumor_pct: u8,
}

impl CoreExpr {
  pub fn benign() -> Self {
    Self { 
      p3: 0, 
      p4: 0, 
      p5: 0, 
      tumor_pct: 0 
    }
  }

  pub fn from_single(pattern: u8, tumor_pct: u8) -> Result<Self, CoreExprError> {
    let (p3, p4, p5): (u8, u8, u8) = match pattern {
      3 => (100, 0, 0),
      4 => (0, 100, 0),
      5 => (0, 0, 100),
      _ => return Err(CoreExprError::InvalidPattern),
    };

    Ok(Self { p3, p4, p5, tumor_pct})
  }

  pub fn from_primary_secondary(primary: u8, secondary: u8, secondary_pct: u8, tumor_pct: u8) -> Result<Self, CoreExprError> {
    if secondary_pct > 50 {
      return Err(CoreExprError::InvalidPattern);
    }

    let primary_pct = 100 - secondary_pct;

    let (p3, p4, p5) = match (primary, secondary) {
      (3, 4) => (primary_pct, secondary_pct, 0),
      (3, 5) => (primary_pct, 0, secondary_pct),
      (4, 3) => (secondary_pct, primary_pct, 0),
      (4, 5) => (0, primary_pct, secondary_pct),
      (5, 3) => (secondary_pct, 0, primary_pct),
      (5, 4) => (0, secondary_pct, primary_pct),
      _ => return Err(CoreExprError::InvalidPattern),
    };
    
    Ok(CoreExpr { p3, p4, p5, tumor_pct })
  }
}

pub fn parse(input: &str) -> Result<CoreExpr, CoreExprError> {
  // Parse empty
  if input.is_empty() {
    return Err(CoreExprError::EmptyInput);
  }

  // Parse benign
  if input == "0" {
    return Ok(CoreExpr::benign());
  }

  // Parse malignant
  let (pattern_str, pct_str) = input
    .split_once("/")
    .ok_or(CoreExprError::MissingTumorPercent)?;
  
  let tumor_pct: u8 = pct_str
    .parse()
    .map_err(|_| CoreExprError::InvalidTumorPercent)?;

  // Parse explicit patterns. e.g. 10,20,70/60
  if pattern_str.contains(',') {
    let parts: Vec<&str> = pattern_str
      .split(',')
      .collect();
    
    if parts.len() != 3 {
      return Err(CoreExprError::InvalidPattern);
    }

    let p3: u8 = parts[0].parse().map_err(|_| CoreExprError::InvalidPattern)?;
    let p4: u8 = parts[1].parse().map_err(|_| CoreExprError::InvalidPattern)?;
    let p5: u8 = parts[2].parse().map_err(|_| CoreExprError::InvalidPattern)?;

    return Ok(CoreExpr { p3, p4, p5, tumor_pct });
  }

  // Parse cases with primary and secondary patterns. e.g. 3+4[30]/60
  if pattern_str.contains("+") {
    let (primary_str, rest_str) = pattern_str
      .split_once('+')
      .ok_or(CoreExprError::InvalidPattern)?;

    let primary: u8 = primary_str
      .parse()
      .map_err(|_| CoreExprError::InvalidPattern)?;

    let (secondary_str, secondary_pct_str) = rest_str
      .split_once('[')
      .ok_or(CoreExprError::InvalidPattern)?;

    let secondary: u8 = secondary_str
      .parse()
      .map_err(|_| CoreExprError::InvalidPattern)?;

    let secondary_pct: u8 = secondary_pct_str
      .trim_end_matches(']')
      .parse()
      .map_err(|_| CoreExprError::InvalidPattern)?;

    return CoreExpr::from_primary_secondary(primary, secondary, secondary_pct, tumor_pct);
  } 
  
  // Parse only one pattern. e.g. 3/60
  let pattern: u8 = pattern_str
  .parse()
  .map_err(|_| CoreExprError::InvalidPattern)?;

  CoreExpr::from_single(pattern, tumor_pct)
}

#[cfg(test)]
mod tests {
  use super::*;

  struct ValidCase<'a> {
    input: &'a str,
    expected: CoreExpr,
  }

  #[test]
  fn empty_input_should_error() {
    let result = parse("");

    assert_eq!(result.unwrap_err(), CoreExprError::EmptyInput);
  }

  #[test]
  fn benign_core() {
    let result = parse("0").unwrap();

    assert_eq!(result, CoreExpr::benign());
  }

  #[test]
  fn missing_percent_should_error() {
    let cases = [
      "3",
      "4+5",
    ];

    for case in cases {
      let result = parse(case).unwrap_err();
      assert_eq!(result, CoreExprError::MissingTumorPercent);
    }
  }

  #[test]
  fn invalid_percent_should_error() {
    let cases = [
      "3/a",
      "a/c",
    ];

    for case in cases {
      let result = parse(case).unwrap_err();
      assert_eq!(result, CoreExprError::InvalidTumorPercent);
    }
  }

  #[test]
  fn invalid_pattern_should_error() {
    let cases = [
      "a/20",
      "b/60",
      "1/20",
      "2/20",
      "6/40",
    ];

    for case in cases {
      let result = parse(case).unwrap_err();
      assert_eq!(result, CoreExprError::InvalidPattern)
    }
  }

  #[test]
  fn single_pattern() {
    let cases = [
      ValidCase {
        input: "3/20",
        expected: CoreExpr { p3: 100, p4: 0, p5: 0, tumor_pct: 20 },
      },
      ValidCase {
        input: "4/50",
        expected: CoreExpr { p3: 0, p4: 100, p5: 0, tumor_pct: 50 },
      },
      ValidCase {
        input: "5/80",
        expected: CoreExpr { p3: 0, p4: 0, p5: 100, tumor_pct: 80 },
      }
    ];

    for case in cases {
      let result = parse(case.input).unwrap();
      assert_eq!(result, case.expected, "failed for input {}", case.input);
    }
  }

  #[test]
  fn valid_two_pattern_cases() {
    let cases = [
      ValidCase {
        input: "3+4[20]/40",
        expected: CoreExpr { p3: 80, p4: 20, p5: 0, tumor_pct: 40 },
      },
      ValidCase {
        input: "4+3[30]/60",
        expected: CoreExpr { p3: 30, p4: 70, p5: 0, tumor_pct: 60 },
      },
      ValidCase {
        input: "4+5[10]/80",
        expected: CoreExpr { p3: 0, p4: 90, p5: 10, tumor_pct: 80 },
      },
      ValidCase {
        input: "3+5[40]/60",
        expected: CoreExpr { p3: 60, p4: 0, p5: 40, tumor_pct: 60 }
      },
      ValidCase {
        input: "5+3[30]/70",
        expected: CoreExpr { p3: 30, p4: 0, p5: 70, tumor_pct: 70 }
      },
      ValidCase {
        input: "5+4[50]/80",
        expected: CoreExpr { p3: 0, p4: 50, p5: 50, tumor_pct: 80 }
      }
    ];

    for case in cases {
      let result = parse(case.input).unwrap();
      assert_eq!(result, case.expected);
    }
  }

  #[test]
  fn invalid_two_pattern_cases() {
    let cases = [
      "4+5[60]/70",
      "4+5]30[/80"
    ];

    for case in cases {
      let result = parse(case).unwrap_err();
      assert_eq!(result, CoreExprError::InvalidPattern);
    }
  }

  #[test]
  fn tertiary_pattern_case()
  {
    let result = parse("10,40,50/60").unwrap();
    assert_eq!(
      result,
      CoreExpr { p3: 10, p4: 40, p5: 50, tumor_pct: 60 },
    )
  }
}