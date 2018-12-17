pub mod map;
pub mod object;

pub type LineId = usize;

pub type LcsToken = String;
pub type LcsSeq = Vec<LcsToken>;

pub fn tokenize(input: &str) -> impl Iterator<Item = &str> {
    input.trim().split_whitespace().filter(|s| !s.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenization() {
        let input = "Command Failed on: node-127";
        let tokenized: Vec<_> = tokenize(input).collect();
        let expected = vec!["Command", "Failed", "on:", "node-127"];
        assert_eq!(tokenized, expected);
    }
}
