pub mod map;
pub mod object;

pub type LineId = usize;

pub type LcsSeq = Vec<String>;

const TOKENIZATION_DELIMITERS: &[char] = &[' ', ',', '.', '_', '/', ':', '\\'];

pub fn tokenize(input: &str) -> impl Iterator<Item = &str> {
    input
        .trim()
        .split(TOKENIZATION_DELIMITERS)
        .filter(|s| !s.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenization() {
        let input = "Command Failed on: node-127";
        let tokenized: Vec<_> = tokenize(input).collect();
        let expected = vec!["Command", "Failed", "on", "node-127"];
        assert_eq!(tokenized, expected);
    }
}
