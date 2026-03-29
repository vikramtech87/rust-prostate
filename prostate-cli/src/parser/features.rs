use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static FEATURE_MAP: Lazy<HashMap<&'static str, Feature>> = Lazy::new(|| {
    let mut m = HashMap::new();

    m.insert("pni", Feature::PerineuralInvasion);
    m.insert("p", Feature::PerineuralInvasion);

    m.insert("crib", Feature::Cribriform);
    m.insert("c", Feature::Cribriform);

    m.insert("lvi", Feature::LymphovascularInvasion);
    m.insert("l", Feature::LymphovascularInvasion);

    m.insert("svi", Feature::SeminalVesicleInvasion);
    m.insert("s", Feature::SeminalVesicleInvasion);

    m.insert("idc", Feature::IntraductalCarcinoma);
    m.insert("i", Feature::IntraductalCarcinoma);

    m.insert("ppfe", Feature::PeriprostaticFatExtension);
    m.insert("f", Feature::PeriprostaticFatExtension);

    m
});

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Feature {
    PerineuralInvasion,
    Cribriform,
    LymphovascularInvasion,
    SeminalVesicleInvasion,
    IntraductalCarcinoma,
    PeriprostaticFatExtension,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseFeatureError {
    InvalidFeature,
}

pub fn parse_feature(s: &str) -> Option<Feature> {
    FEATURE_MAP.get(s).copied()
}

pub fn parse_feature_list(s: &str) -> Result<Vec<Feature>, ParseFeatureError> {
    s.split(',')
        .map(|f| parse_feature(f).ok_or(ParseFeatureError::InvalidFeature))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_features() {
        struct Case {
            input: &'static str,
            feature: Feature,
        }

        let cases = [
            Case {
                input: "pni",
                feature: Feature::PerineuralInvasion,
            },
            Case {
                input: "p",
                feature: Feature::PerineuralInvasion,
            },
            Case {
                input: "s",
                feature: Feature::SeminalVesicleInvasion,
            },
            Case {
                input: "svi",
                feature: Feature::SeminalVesicleInvasion,
            },
            Case {
                input: "c",
                feature: Feature::Cribriform,
            },
            Case {
                input: "crib",
                feature: Feature::Cribriform,
            },
            Case {
                input: "f",
                feature: Feature::PeriprostaticFatExtension,
            },
            Case {
                input: "ppfe",
                feature: Feature::PeriprostaticFatExtension,
            },
            Case {
                input: "l",
                feature: Feature::LymphovascularInvasion,
            },
            Case {
                input: "lvi",
                feature: Feature::LymphovascularInvasion,
            },
            Case {
                input: "i",
                feature: Feature::IntraductalCarcinoma,
            },
            Case {
                input: "idc",
                feature: Feature::IntraductalCarcinoma,
            },
        ];

        for case in cases {
            assert_eq!(parse_feature(case.input), Some(case.feature));
        }
    }

    #[test]
    fn reject_invalid_feature_list() {
        let result = parse_feature_list("xyz,pni").unwrap_err();
        assert_eq!(result, ParseFeatureError::InvalidFeature);
    }

    #[test]
    fn parse_valid_feature_list() {
        struct Case {
            input: &'static str,
            features: Vec<Feature>,
        }

        let cases = [
            Case {
                input: "p,crib,idc",
                features: vec![
                    Feature::PerineuralInvasion,
                    Feature::Cribriform,
                    Feature::IntraductalCarcinoma,
                ],
            },
            Case {
                input: "s",
                features: vec![
                    Feature::SeminalVesicleInvasion
                ],
            },
            Case {
                input: "l,p,f",
                features: vec![
                    Feature::LymphovascularInvasion,
                    Feature::PerineuralInvasion,
                    Feature::PeriprostaticFatExtension,
                ]
            }
        ];

        for case in cases {
            let result = parse_feature_list(case.input).unwrap();
            assert_eq!(result, case.features)
        }
    }
}
