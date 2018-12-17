use serde_derive::*;

use crate::{tokenize, LcsSeq, LineId};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LcsObject {
    pub tokens: LcsSeq,
    pub lines_ids: Vec<LineId>,
}

impl LcsObject {
    pub fn from_str(s: &str, line_id: LineId) -> LcsObject {
        let tokens = tokenize(s).map(|s| s.to_string()).collect();
        LcsObject::new(tokens, line_id)
    }

    pub fn from_str_slice(tokens: &[&str], line_id: LineId) -> LcsObject {
        LcsObject {
            tokens: tokens.iter().map(|s| s.to_string()).collect(),
            lines_ids: vec![line_id],
        }
    }

    pub fn new(tokens: LcsSeq, line_id: LineId) -> LcsObject {
        LcsObject {
            tokens,
            lines_ids: vec![line_id],
        }
    }

    pub fn get_lcs(&self, other: &LcsSeq) -> usize {
        let mut count = 0;

        // Simple loop implementation from the paper
        let mut last_match = 0;
        for s_left in self.tokens.iter().filter(|s| *s != "*") {
            for (j, s_right) in other.iter().enumerate().skip(last_match) {
                if s_left == s_right {
                    last_match = j + 1;
                    count += 1;
                    break;
                }
            }
        }

        count
    }

    pub fn insert(&mut self, tokens: LcsSeq, line_id: LineId) {
        self.lines_ids.push(line_id);
        let mut tmp = String::with_capacity(256);
        let mut last_match = 0;
        let mut placeholder = false;
        for s_left in self.tokens.iter() {
            if s_left == "*" {
                if !placeholder {
                    tmp.push_str("* ");
                }
                placeholder = true;
                continue;
            }

            for (j, s_right) in tokens.iter().enumerate().skip(last_match) {
                if s_left == s_right {
                    placeholder = false;
                    tmp.push_str(&s_left);
                    tmp.push(' ');
                    last_match = j + 1;
                    break;
                } else if !placeholder {
                    tmp.push_str("* ");
                    placeholder = true;
                }
            }
        }

        self.tokens = tmp.trim().split(' ').map(|s| s.to_string()).collect();
    }

    pub fn length(&self) -> usize {
        self.tokens.len()
    }

    pub fn count(&self) -> usize {
        self.lines_ids.len()
    }
}

impl std::fmt::Display for LcsObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}\n\t\t{{{}}}",
            self.tokens
                .iter()
                .map(|s| &**s)
                .collect::<Vec<&str>>()
                .join(" "),
            self.lines_ids
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor() {
        let input = "Command Failed on: node-127";
        let _lcs_object = LcsObject::from_str(input, 0);
    }
}
