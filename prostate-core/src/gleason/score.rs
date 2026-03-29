use super::percentages::PatternPercentages;
use super::pattern::Pattern;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GleasonScore {
  primary: Pattern,
  secondary: Pattern
}

impl GleasonScore {
  pub fn primary(&self) -> Pattern {
    self.primary
  }

  pub fn secondary(&self) -> Pattern {
    self.secondary
  }

  pub fn from_percentages(perc: &PatternPercentages) -> Self {
    let primary = Pattern::all()
      .into_iter()
      .max_by_key(|p| perc[*p])
      .unwrap();

    let secondary = Pattern::all()
      .into_iter()
      .filter(|&p| p != primary)
      .filter(|&p| perc[p] > 0)
      .filter(|&p| perc[p] >= 5 || p.value() > primary.value())
      .max_by_key(|p| p.value())
      .unwrap_or(primary);

    GleasonScore { 
      primary, 
      secondary
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gleason::percentages::PatternPercentages;
    use crate::gleason::pattern::Pattern;

    #[derive(Debug)]
    struct Case {
        p3: u8,
        p4: u8,
        p5: u8,
        primary: Pattern,
        secondary: Pattern,
    }

    #[test]
    fn biopsy_gleason_score_cases() {
        let cases = vec![
            Case { p3:100, p4:0,   p5:0,  primary:Pattern::Three, secondary:Pattern::Three },
            Case { p3:90,  p4:10,  p5:0,  primary:Pattern::Three, secondary:Pattern::Four },
            Case { p3:50,  p4:50,  p5:0,  primary:Pattern::Four,  secondary:Pattern::Three },
            Case { p3:60,  p4:30,  p5:10, primary:Pattern::Three, secondary:Pattern::Five },
            Case { p3:30,  p4:40,  p5:30, primary:Pattern::Four,  secondary:Pattern::Five },
            Case { p3:40,  p4:30,  p5:30, primary:Pattern::Three, secondary:Pattern::Five },
            Case { p3:20,  p4:40,  p5:40, primary:Pattern::Five,  secondary:Pattern::Four },
            Case { p3:0,   p4:100, p5:0,  primary:Pattern::Four,  secondary:Pattern::Four },
            Case { p3:0,   p4:0,   p5:100,primary:Pattern::Five,  secondary:Pattern::Five },
            Case { p3:0,   p4:60,  p5:40, primary:Pattern::Four,  secondary:Pattern::Five },
            Case { p3:0,   p4:40,  p5:60, primary:Pattern::Five,  secondary:Pattern::Four },
            Case { p3:33,  p4:33,  p5:34, primary:Pattern::Five,  secondary:Pattern::Four },
            Case { p3:0,   p4:99, p5:1,  primary:Pattern::Four,  secondary:Pattern::Five },
            Case { p3:99,   p4:0, p5:1,  primary:Pattern::Three,  secondary:Pattern::Five },
            Case { p3:99,   p4:0, p5:1,  primary:Pattern::Three,  secondary:Pattern::Five },
            Case { p3: 4, p4: 96, p5: 0, primary:Pattern::Four, secondary:Pattern::Four},
            Case { p3: 4, p4: 0, p5: 96, primary:Pattern::Five, secondary:Pattern::Five},
            Case { p3: 0, p4: 4, p5: 96, primary:Pattern::Five, secondary:Pattern::Five},
        ];

        for case in cases {

            let perc = PatternPercentages::new(case.p3, case.p4, case.p5).unwrap();

            let score = GleasonScore::from_percentages(&perc);

            assert_eq!(score.primary(), case.primary, "case: {:?}", case);
            assert_eq!(score.secondary(), case.secondary, "case: {:?}", case);
        }
    }
}