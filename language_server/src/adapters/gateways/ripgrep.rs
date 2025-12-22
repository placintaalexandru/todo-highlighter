use std::{collections::HashMap, path::Path};

use grep::{
    matcher::Matcher,
    regex::RegexMatcher,
    searcher::{Searcher as GrepSearcher, sinks::UTF8},
};
use walkdir::WalkDir;

use crate::{
    entities::{Column, Error, FileState, Match, RowMetadata, State, TodoResult},
    use_cases::ports::{RegexManager, RegexSearcher, Searcher},
};

#[derive(Debug, Clone)]
pub struct RipGrepSearcher {
    matcher: RegexMatcher,
}

impl RipGrepSearcher {
    fn first_non_white_space_position(&self, text: &str) -> Column {
        for (i, c) in text.chars().enumerate() {
            if !c.is_whitespace() {
                return i.into();
            }
        }

        text.len().into()
    }
}

impl Searcher for RipGrepSearcher {
    fn search_in_path<P: AsRef<Path>>(&self, path: P) -> Option<FileState> {
        let mut file_matches = HashMap::new();
        let mut searcher = GrepSearcher::new();

        let _ = searcher.search_path(
            &self.matcher,
            path,
            UTF8(|line_num, line| {
                let row = (line_num as usize - 1).into();
                let first_non_empty_col = self.first_non_white_space_position(line);
                let metadata = RowMetadata::new(line.len(), first_non_empty_col);
                let mut matches = vec![];
                let _ = self.matcher.try_find_iter(line.as_bytes(), |m| {
                    let matched_patch = line[m.start()..m.end()].to_owned();
                    let match_start = m.start().into();
                    let m = Match::new(match_start, matched_patch);

                    matches.push(m);
                    Result::<bool, ()>::Ok(true)
                });

                file_matches.entry(row).insert_entry((metadata, matches));
                Ok(true)
            }),
        );

        FileState::try_new(file_matches)
    }

    fn search_in_text(&self, text: &str) -> Option<FileState> {
        let mut file_matches = HashMap::new();
        let mut searcher = GrepSearcher::new();

        let _ = searcher.search_slice(
            &self.matcher,
            text.as_bytes(),
            UTF8(|line_num, line| {
                let row = (line_num as usize - 1).into();
                let first_non_empty_col = self.first_non_white_space_position(line);
                let metadata = RowMetadata::new(line.len(), first_non_empty_col);
                let mut matches = Vec::new();
                let _ = self.matcher.try_find_iter(line.as_bytes(), |m| {
                    let matched_patch = line[m.start()..m.end()].to_owned();
                    let match_start = m.start().into();
                    let m = Match::new(match_start, matched_patch);

                    matches.push(m);
                    Result::<bool, ()>::Ok(true)
                });

                file_matches.entry(row).insert_entry((metadata, matches));
                Ok(true)
            }),
        );

        FileState::try_new(file_matches)
    }

    fn recurssive_search<P: AsRef<Path>>(&self, root: P) -> State {
        let mut state = State::default();

        for entry in WalkDir::new(root)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            if path.is_dir() {
                if self.should_skip(path) {
                    continue;
                }

                state.extend(self.recurssive_search(path));
            } else if path.is_file()
                && let Some(file_full_path) = path.to_str()
                && let Some(file_matches) = self.search_in_path(path)
            {
                let file_full_path = file_full_path.to_owned();
                state.extend(State::from_file_matches(file_full_path, file_matches));
            }
        }

        state
    }
}

impl Default for RipGrepSearcher {
    fn default() -> Self {
        Self::try_from_regex(r"TODO").unwrap()
    }
}

impl RegexManager for RipGrepSearcher {
    fn update_regex(&mut self, key_words: &[&str]) -> TodoResult<()> {
        if key_words.is_empty() {
            return Err(Error::InvalidRegex(
                "Cannot empty list of keyowrds".to_owned(),
            ));
        }

        let regex = key_words
            .iter()
            .map(|s| (*s).trim())
            .filter(|s| !(*s).is_empty())
            .collect::<Vec<_>>()
            .join("|");

        match Self::try_from_regex(&regex) {
            Ok(new_matcher) => {
                *self = new_matcher;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

impl RipGrepSearcher {
    fn try_from_regex(pattern: &str) -> TodoResult<Self> {
        let matcher = RegexMatcher::new(pattern).map_err(|e| Error::InvalidRegex(e.to_string()))?;
        Ok(Self { matcher })
    }
}

impl RegexSearcher for RipGrepSearcher {}

// grcov-excl-start
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_does_not_panic() {
        let _ = RipGrepSearcher::default();
    }

    #[test]
    fn updating_regex_trims_keywords() {
        let mut searcher = RipGrepSearcher::default();
        let keywords_with_space = ["  TODO ", "FIXME  "];
        searcher.update_regex(&keywords_with_space).unwrap();

        let m1 = searcher
            .matcher
            .find(keywords_with_space[0].as_bytes())
            .unwrap()
            .unwrap();
        let m2 = searcher
            .matcher
            .find(keywords_with_space[1].as_bytes())
            .unwrap()
            .unwrap();

        assert_eq!((m1.start(), m1.end()), (2, 6));
        assert_eq!((m2.start(), m2.end()), (0, 5));
    }
}
// grcov-excl-stop
