use super::core_expr::{CoreExpr, parse_core_expr};
use super::features::{Feature, parse_feature_list};
use crate::parser::repeat::parse_repeat;

#[derive(Debug, PartialEq, Eq)]
pub struct CoreInfo {
    pub core: CoreExpr,
    pub features: Vec<Feature>,
    pub repeat: u8,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseCoreInfoError {
    EmptyInput,
    InvalidToken,
    ExcessTokens,
    DuplicateToken,
}

pub fn parse_core(s: &str) -> Result<CoreInfo, ParseCoreInfoError> {
    let tokens: Vec<&str> = s.split_whitespace().collect();

    if tokens.is_empty() {
        return Err(ParseCoreInfoError::EmptyInput);
    }

    if tokens.len() > 3 {
        return Err(ParseCoreInfoError::ExcessTokens);
    }

    // Process first token
    let core = parse_core_expr(tokens[0]).map_err(|_| ParseCoreInfoError::InvalidToken)?;
    let mut features: Vec<Feature> = Vec::new();
    let mut repeat: u8 = 1;
    let mut is_feat_set = false;
    let mut is_repeat_set = false;

    // Process 2nd and 3rd tokens
    for token in tokens.iter().skip(1).copied() {
        if let Ok(f) = parse_feature_list(token) {
            if is_feat_set {
                return Err(ParseCoreInfoError::DuplicateToken);
            }
            features = f;
            is_feat_set = true;
        } else {
            let r = parse_repeat(token).ok_or(ParseCoreInfoError::InvalidToken)?;
            if is_repeat_set {
                return Err(ParseCoreInfoError::DuplicateToken);
            }
            repeat = r;
            is_repeat_set = true;
        }
    }

    Ok(CoreInfo {
        core,
        features,
        repeat,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_parse_report() {
        struct ValidCase {
            input: &'static str,
            p3: u8,
            p4: u8,
            p5: u8,
            vol: u8,
            features: Vec<Feature>,
            repeat: u8,
        }

        let cases = [
            ValidCase {
                input: "3+4[20]/70",
                p3: 80,
                p4: 20,
                p5: 0,
                vol: 70,
                features: vec![],
                repeat: 1,
            },
            ValidCase {
                input: "0 x4",
                p3: 0,
                p4: 0,
                p5: 0,
                vol: 0,
                features: vec![],
                repeat: 4,
            },
            ValidCase {
                input: "5/90 pni,svi,l x3",
                p3: 0,
                p4: 0,
                p5: 100,
                vol: 90,
                features: vec![
                    Feature::PerineuralInvasion,
                    Feature::SeminalVesicleInvasion,
                    Feature::LymphovascularInvasion,
                ],
                repeat: 3,
            },
            ValidCase {
                input: "5+4[20]/50 x3",
                p3: 0,
                p4: 20,
                p5: 80,
                vol: 50,
                features: vec![],
                repeat: 3,
            },
            ValidCase {
                input: "80,16,4/80 p,c,l",
                p3: 80,
                p4: 16,
                p5: 4,
                vol: 80,
                features: vec![
                    Feature::PerineuralInvasion,
                    Feature::Cribriform,
                    Feature::LymphovascularInvasion,
                ],
                repeat: 1,
            },
            ValidCase {
                input: "4/20 pni",
                p3: 0,
                p4: 100,
                p5: 0,
                vol: 20,
                features: vec![Feature::PerineuralInvasion],
                repeat: 1,
            },
            ValidCase {
                input: "3+4[20]/80 x2 p,i",
                p3: 80,
                p4: 20,
                p5: 0,
                vol: 80,
                features: vec![Feature::PerineuralInvasion, Feature::IntraductalCarcinoma],
                repeat: 2,
            },
        ];

        for case in cases {
            let result = parse_core(case.input).unwrap();
            assert_eq!(result.core.p3, case.p3);
            assert_eq!(result.core.p4, case.p4);
            assert_eq!(result.core.p5, case.p5);
            assert_eq!(result.core.tumor_pct, case.vol);
            assert_eq!(result.features, case.features);
            assert_eq!(result.repeat, case.repeat);
        }
    }

    #[test]
    fn reject_empty_input() {
        assert_eq!(
            parse_core("").unwrap_err(),
            ParseCoreInfoError::EmptyInput
        );
    }

    #[test]
    fn reject_excess_tokens() {
        assert_eq!(
            parse_core("5/20 p,c x4 x3").unwrap_err(),
            ParseCoreInfoError::ExcessTokens
        );
    }

    #[test]
    fn reject_repeat_tokens() {
        assert_eq!(
            parse_core("4/30 p,c l,s").unwrap_err(),
            ParseCoreInfoError::DuplicateToken
        );
        assert_eq!(
            parse_core("3/90 x2 x3").unwrap_err(),
            ParseCoreInfoError::DuplicateToken
        );
    }
}
