use std::collections::HashMap;


#[derive(Debug)]
enum PathResult {
    Pattern(String, String),
    Match,
}

impl PathResult {
    fn pattern(&self) -> Option<(String, String)> {
        match self {
            PathResult::Pattern(s1, s2) => Some((s1.to_string(), s2.to_string())),
            PathResult::Match => None,
        }
    }
}

pub fn parse(path: &str, pattern: &str) -> Option<HashMap<String, String>> {
    let paths = pattern.trim().split('/').zip(path.trim().split('/'))
        .map(|(pattern, path)| {
            pattern.starts_with(":")
                .then(|| PathResult::Pattern(pattern.trim_start_matches(":").to_string(), path.to_string()))
                .or_else(|| pattern.eq(path).then(|| PathResult::Match))
        });

    paths.clone().all(|path| path.is_some())
        .then(|| paths.filter_map(|path| path.and_then(|result| result.pattern())).collect::<HashMap<String, String>>())
}


