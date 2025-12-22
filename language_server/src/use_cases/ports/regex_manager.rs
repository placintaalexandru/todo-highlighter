use crate::entities::TodoResult;

pub trait RegexManager {
    fn update_regex(&mut self, key_words: &[&str]) -> TodoResult<()>;
}
