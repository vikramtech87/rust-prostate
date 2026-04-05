use super::core_info::{parse_core, CoreInfo, ParseCoreInfoError};

pub struct SiteInfo {
    pub name: String,
    pub cores: Vec<CoreInfo>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseSiteInfoError {
    InvalidInput,
    InvalidCore(ParseCoreInfoError),
}

pub fn parse_site(s: &str) -> Result<SiteInfo, ParseSiteInfoError> {
    let (name, rest) = extract(s)
        .ok_or(ParseSiteInfoError::InvalidInput)?;

    let cores = extract_cores(&rest)?;
    if cores.is_empty() {
        return Err(ParseSiteInfoError::InvalidCore(ParseCoreInfoError::EmptyInput));
    }
    Ok(SiteInfo { name, cores })
}

fn extract(s: &str) -> Option<(String, String)> {
    if !s.starts_with('"') {
        return None;
    }

    let end = s[1..].find('"')? + 1;

    let name = s[1..end].trim().to_owned();
    let rest = s[end+1..].trim().to_owned();

    Some((name, rest))
}

fn extract_cores(c: &str) -> Result<Vec<CoreInfo>, ParseSiteInfoError> {
    c
        .split('|')
        .map(|c| c.trim())
        .filter(|c| !c.is_empty())
        .map(|c| parse_core(c).map_err(|e| ParseSiteInfoError::InvalidCore(e)))
        .collect::<Result<Vec<_>, _>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_name_and_rest() {
        let input = r#""Left upper lateral" 3+4[20]/70 p,c | 0 x3"#;

        let (name, rest) = extract(input).unwrap();
        assert_eq!(name, "Left upper lateral");
        assert_eq!(rest, "3+4[20]/70 p,c | 0 x3");
    }

    #[test]
    fn parse_site_invalid_cases() {
        struct Case {
            input: &'static str,
            is_invalid_core: bool,
        }

        let cases = [
            // ❌ missing quotes
            Case {
                input: r#"Left upper lateral 3+4[20]/70"#,
                is_invalid_core: false,
            },
            // ❌ missing closing quote
            Case {
                input: r#""Left upper medial 3+4[20]/70"#,
                is_invalid_core: false,
            },
            // ❌ empty input
            Case {
                input: r#""#,
                is_invalid_core: false,
            },
            // ❌ no cores provided
            Case {
                input: r#""Left mid lateral""#,
                is_invalid_core: true,
            },
            // ❌ only separator
            Case {
                input: r#""Left mid medial" |"#,
                is_invalid_core: true,
            },
            // ❌ multiple separators, no cores
            Case {
                input: r#""Left lower lateral" ||"#,
                is_invalid_core: true ,
            },
            // ❌ garbage after name
            Case {
                input: r#""Left lower medial" ???"#,
                is_invalid_core: true,
            },
            // ❌ invalid core (delegated failure)
            Case {
                input: r#""Right upper lateral" abc"#,
                is_invalid_core: true,
            },
            // ❌ partially valid + invalid core
            Case {
                input: r#""Right upper medial" 3+4[20]/70 | abc"#,
                is_invalid_core: true,
            },
        ];

        for case in cases {
            let result = parse_site(case.input);
            assert!(result.is_err(), "input should fail: {}", case.input);
            
            if case.is_invalid_core {
                assert!(matches!(
                    result,
                    Err(ParseSiteInfoError::InvalidCore(_)),
                ));
            } else {
                assert!(matches!(
                    result,
                    Err(ParseSiteInfoError::InvalidInput),
                ));
            }
        }
    }

    #[test]
    fn parse_site_valid_cases() {
        struct Case {
            input: &'static str,
            site_name: &'static str,
            core_count: usize,
        }

        let cases = [
            Case {
                input: r#""Left upper lateral" 3+4[20]/70"#,
                site_name: "Left upper lateral",
                core_count: 1,
            },
            Case {
                input: r#""Right apex" 3/70"#,
                site_name: "Right apex",
                core_count: 1,
            },
            Case {
                input: r#""Left base" 0"#,
                site_name: "Left base",
                core_count: 1,
            },
            Case {
                input: r#""Left mid" 3+4[20]/70 p,c"#,
                site_name: "Left mid",
                core_count: 1,
            },
            Case {
                input: r#""Left apex" 0 x3"#,
                site_name: "Left apex",
                core_count: 1,
            },
            Case {
                input: r#""Left upper lateral" 3+4[20]/70 p,c | 0 x3"#,
                site_name: "Left upper lateral",
                core_count: 2,
            },
            Case {
                input: r#""Right apex" 3/70 | 4+5[10]/80 p"#,
                site_name: "Right apex",
                core_count: 2,
            },
            Case {
                input: r#""Left base" 3+4[20]/70 p,c x2 | 0 x1 | 4+5[10]/80"#,
                site_name: "Left base",
                core_count: 3,
            },
            Case {
                input: r#""Left mid" 60,30,10/70"#,
                site_name: "Left mid",
                core_count: 1,
            },
            Case {
                input: r#""Left apex" 60,30,10/70 x2 | 0 x1"#,
                site_name: "Left apex",
                core_count: 2,
            },
        ];

        for case in cases {
            let site = parse_site(case.input)
                .expect("should parse");

            assert_eq!(site.name, case.site_name);
            assert_eq!(site.cores.len(), case.core_count);
        }
    }
}