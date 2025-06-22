#[derive(Default, Clone)]
/// A deduplicated cache of all the regexes found in a rule.
/// This is used to create the per-rule `LazyLock` which
/// caches the compilation of the regex string
///
/// Not the most efficient implementation but doesn't require any dependencies
pub struct RuleRegexes(Vec<String>);

impl RuleRegexes {
    pub fn insert(&mut self, value: &str) {
        if !self.0.iter().any(|v| v == value) {
            self.0.push(value.to_owned());
        }
    }

    pub fn get_index(&self, value: &str) -> Option<usize> {
        self.0.iter().position(|v| v == value)
    }

    pub fn cloned_iter(&self) -> impl Iterator<Item = (usize, String)> + '_ {
        self.0.iter().cloned().enumerate()
    }
}
