use std::collections::HashMap;

#[derive(Default, Clone)]
/// A deduplicated cache of all the regexes found in a rule.
/// This is used to create the per-rule `LazyLock` which
/// caches the compilation of the regex string
///
/// Not the most efficient implementation but doesn't require any dependencies
pub struct RuleRegexes {
    i: usize,
    map: HashMap<String, usize>,
}

impl RuleRegexes {
    pub fn insert(&mut self, value: &str) {
        if !self.map.contains_key(value) {
            self.map.insert(value.to_owned(), self.i);
            self.i += 1;
        }
    }

    pub fn get_index(&self, value: &str) -> Option<usize> {
        self.map.get(value).copied()
    }

    pub fn cloned_iter(&self) -> impl Iterator<Item = (String, usize)> + '_ {
        self.map.iter().map(|(k, v)| (k.clone(), *v))
    }
}
