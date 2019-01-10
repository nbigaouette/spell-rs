pub mod map;
pub mod object;

pub type LineId = usize;

pub type LcsToken = String;
pub type LcsSeq = Vec<LcsToken>;
pub type LcsDelimiters = Vec<char>;

/** 
 * Tokenizer is using delimiters specified in LcsMap::new to create the tokens.
*/
pub fn tokenize<'a>(input: &'a str, delimiters: &'a [char]) -> impl Iterator<Item = &'a str> {
    input.trim().split(&delimiters[..]).filter(|s| !s.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenization() {
        let input = "Command Failed on: node-127,node-234";
        let tokenized: Vec<_> = tokenize(input, &[' ']).collect();
        let expected = vec!["Command", "Failed", "on:", "node-127,node-234"];
        assert_eq!(tokenized, expected);
    }

    #[test]
    fn tokenization_with_multiple_delimiters() {
        let input = "Command Failed on: node-127,node-234";
        let tokenized: Vec<_> = tokenize(input, &[' ', ',', ':']).collect();
        let expected = vec!["Command", "Failed", "on", "node-127", "node-234"];
        assert_eq!(tokenized, expected);
    }

    #[test]
    fn tokenization_with_no_delimiters() {
        let input = "Command Failed on: node-127,node-234";
        let tokenized: Vec<_> = tokenize(input, &[]).collect();
        assert_eq!(tokenized, [input]);
    }
}
