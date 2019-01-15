use serde_derive::*;

use crate::{object::LcsObject, tokenize, LcsDelimiters, LcsSeq, LineId};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LcsMap {
    pub seq: Vec<LcsObject>,
    pub line_id: LineId,
    pub delimiters: LcsDelimiters,
}

macro_rules! fold_get_match {
    ($iter:expr, $tokenized:expr) => {
        $iter.fold((None, 0), |(best_match, best_match_length), obj| {
            // Pruning as described in paper
            if obj.length() < $tokenized.len() / 2 || obj.length() > $tokenized.len() * 2 {
                (best_match, best_match_length)
            } else {
                let l = obj.get_lcs($tokenized);
                if l >= $tokenized.len() / 2 && l > best_match_length {
                    (Some(obj), l)
                } else {
                    (best_match, best_match_length)
                }
            }
        })
    };
}

impl LcsMap {
    pub fn new() -> LcsMap {
        LcsMap {
            delimiters: vec![' '],
            ..Default::default()
        }
    }

    /// Constructor to create an LcsMap with different set of delimiters.
    pub fn with_delimiters(delimiters: Vec<char>) -> LcsMap {
        LcsMap {
            delimiters,
            ..LcsMap::new()
        }
    }

    pub fn insert(&mut self, entry: &str) {
        let tokenized: LcsSeq = tokenize(entry, self.delimiters.as_slice())
            .map(|token| token.to_string())
            .collect();

        let line_id = self.line_id;

        match self.get_match_mut(&tokenized) {
            None => {
                let obj = LcsObject::new(tokenized, line_id);
                self.seq.push(obj);
            }
            Some(obj) => {
                obj.insert(tokenized, line_id);
            }
        }
        self.line_id += 1;
    }

    pub fn get_match(&self, tokenized: &LcsSeq) -> Option<&LcsObject> {
        let (best_match, _best_match_length) = fold_get_match!(self.seq.iter(), tokenized);
        best_match
    }

    pub fn get_match_mut(&mut self, tokenized: &LcsSeq) -> Option<&mut LcsObject> {
        let (best_match, _best_match_length) = fold_get_match!(self.seq.iter_mut(), tokenized);
        best_match
    }

    // fn object_at(&self, index: usize) -> Option<&LcsObject> {
    //     self.seq.get(index)
    // }

    fn size(&self) -> usize {
        self.seq.len()
    }
}

impl std::fmt::Display for LcsMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\t{} Objects in the LCSMap\n\n", self.size())?;

        let mut entry_count = 0;
        for (i, obj) in self.seq.iter().enumerate() {
            write!(f, "\tObject {}:\n\t\t{}\n", i, obj.to_string())?;
            entry_count += obj.count();
        }

        write!(
            f,
            "\n\t{} total entries found, {} expected.",
            entry_count, self.line_id
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixtures_input_var_log_messages_lines() -> [&'static str; 6] {
        [
            include_str!("../fixtures/input/var_log_messages_line_1.txt"),
            include_str!("../fixtures/input/var_log_messages_line_2.txt"),
            include_str!("../fixtures/input/var_log_messages_line_3.txt"),
            include_str!("../fixtures/input/var_log_messages_line_4.txt"),
            include_str!("../fixtures/input/var_log_messages_line_5.txt"),
            include_str!("../fixtures/input/var_log_messages_line_6.txt"),
        ]
    }

    fn fixtures_output_original_impl() -> [&'static str; 6] {
        [
            include_str!("../fixtures/output/original/original_after_line_1.txt"),
            include_str!("../fixtures/output/original/original_after_line_2.txt"),
            include_str!("../fixtures/output/original/original_after_line_3.txt"),
            include_str!("../fixtures/output/original/original_after_line_4.txt"),
            include_str!("../fixtures/output/original/original_after_line_5.txt"),
            include_str!("../fixtures/output/original/original_after_line_6.txt"),
        ]
    }

    #[test]
    fn compare_java() {
        let inputs = fixtures_input_var_log_messages_lines();
        let expected = fixtures_output_original_impl();

        let mut map = LcsMap::new();

        map.insert(inputs[0]);
        let to_check = map.to_string();
        assert_eq!(to_check, expected[0]);

        map.insert(inputs[1]);
        let to_check = map.to_string();
        assert_eq!(to_check, expected[1]);

        map.insert(inputs[2]);
        let to_check = map.to_string();
        assert_eq!(to_check, expected[2]);

        map.insert(inputs[3]);
        let to_check = map.to_string();
        assert_eq!(to_check, expected[3]);

        map.insert(inputs[4]);
        let to_check = map.to_string();
        assert_eq!(to_check, expected[4]);

        map.insert(inputs[5]);
        let to_check = map.to_string();
        assert_eq!(to_check, expected[5]);
    }

    #[test]
    fn parse_log() {
        let inputs = fixtures_input_var_log_messages_lines();

        let mut map = LcsMap::new();
        let expected = LcsMap {
            seq: Vec::new(),
            line_id: 0,
            delimiters: vec![' '],
        };
        assert_eq!(map, expected);

        map.insert(inputs[0]);
        let expected = LcsMap {
            seq: vec![LcsObject {
                tokens: [
                    "Jan", "22", "04:11:04", "combo", "syslogd", "1.4.1:", "restart.",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
                lines_ids: vec![0],
            }],
            line_id: 1,
            delimiters: vec![' '],
        };
        assert_eq!(map, expected);

        map.insert(inputs[1]);
        let expected = LcsMap {
            seq: vec![
                LcsObject {
                    tokens: [
                        "Jan", "22", "04:11:04", "combo", "syslogd", "1.4.1:", "restart.",
                    ]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                    lines_ids: vec![0],
                },
                LcsObject {
                    tokens: [
                        "Jan",
                        "22",
                        "04:11:04",
                        "combo",
                        "logrotate:",
                        "ALERT",
                        "exited",
                        "abnormally",
                        "with",
                        "[1]",
                    ]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                    lines_ids: vec![1],
                },
            ],
            line_id: 2,
            delimiters: vec![' '],
        };
        assert_eq!(map, expected);

        map.insert(inputs[2]);
        let expected = LcsMap {
            seq: vec![
                LcsObject {
                    tokens: [
                        "Jan", "22", "04:11:04", "combo", "syslogd", "1.4.1:", "restart.",
                    ]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                    lines_ids: vec![0],
                },
                LcsObject {
                    tokens: [
                        "Jan",
                        "22",
                        "04:11:04",
                        "combo",
                        "logrotate:",
                        "ALERT",
                        "exited",
                        "abnormally",
                        "with",
                        "[1]",
                    ]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                    lines_ids: vec![1],
                },
                LcsObject {
                    tokens: [
                        "Jan",
                        "22",
                        "04:16:40",
                        "combo",
                        "su(pam_unix)[21719]:",
                        "session",
                        "opened",
                        "for",
                        "user",
                        "news",
                        "by",
                        "(uid=0)",
                    ]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                    lines_ids: vec![2],
                },
            ],
            line_id: 3,
            delimiters: vec![' '],
        };
        assert_eq!(map, expected);

        map.insert(inputs[3]);
        let expected = LcsMap {
            seq: vec![
                LcsObject {
                    tokens: [
                        "Jan", "22", "04:11:04", "combo", "syslogd", "1.4.1:", "restart.",
                    ]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                    lines_ids: vec![0],
                },
                LcsObject {
                    tokens: [
                        "Jan",
                        "22",
                        "04:11:04",
                        "combo",
                        "logrotate:",
                        "ALERT",
                        "exited",
                        "abnormally",
                        "with",
                        "[1]",
                    ]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                    lines_ids: vec![1],
                },
                LcsObject {
                    tokens: [
                        "Jan",
                        "22",
                        "*",
                        "combo",
                        "su(pam_unix)[21719]:",
                        "session",
                        "*",
                        "for",
                        "user",
                        "news",
                    ]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                    lines_ids: vec![2, 3],
                },
            ],
            line_id: 4,
            delimiters: vec![' '],
        };
        assert_eq!(map, expected);

        map.insert(inputs[4]);
        let expected = LcsMap {
            seq: vec![
                LcsObject {
                    tokens: [
                        "Jan", "22", "04:11:04", "combo", "syslogd", "1.4.1:", "restart.",
                    ]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                    lines_ids: vec![0],
                },
                LcsObject {
                    tokens: [
                        "Jan",
                        "22",
                        "04:11:04",
                        "combo",
                        "logrotate:",
                        "ALERT",
                        "exited",
                        "abnormally",
                        "with",
                        "[1]",
                    ]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                    lines_ids: vec![1],
                },
                LcsObject {
                    tokens: ["Jan", "22", "*", "combo", "*", "user", "*"]
                        .iter()
                        .map(|s| s.to_string())
                        .collect(),
                    lines_ids: vec![2, 3, 4],
                },
            ],
            line_id: 5,
            delimiters: vec![' '],
        };
        assert_eq!(map, expected);

        map.insert(inputs[5]);
        let expected = LcsMap {
            seq: vec![
                LcsObject {
                    tokens: [
                        "Jan", "22", "04:11:04", "combo", "syslogd", "1.4.1:", "restart.",
                    ]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                    lines_ids: vec![0],
                },
                LcsObject {
                    tokens: [
                        "Jan",
                        "22",
                        "04:11:04",
                        "combo",
                        "logrotate:",
                        "ALERT",
                        "exited",
                        "abnormally",
                        "with",
                        "[1]",
                    ]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                    lines_ids: vec![1],
                },
                LcsObject {
                    tokens: ["Jan", "22", "*", "combo", "*", "user", "*"]
                        .iter()
                        .map(|s| s.to_string())
                        .collect(),
                    lines_ids: vec![2, 3, 4],
                },
                LcsObject {
                    tokens: [
                        "Jan",
                        "22",
                        "05:23:19",
                        "combo",
                        "sshd(pam_unix)[24892]:",
                        "authentication",
                        "failure;",
                        "logname=",
                        "uid=0",
                        "euid=0",
                        "tty=NODEVssh",
                        "ruser=",
                        "rhost=server3.sugolan.hu",
                    ]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                    lines_ids: vec![5],
                },
            ],
            line_id: 6,
            delimiters: vec![' '],
        };
        assert_eq!(map, expected);
    }

    #[test]
    fn parse_log_with_delimiters() {
        let inputs = fixtures_input_var_log_messages_lines();

        let mut map = LcsMap::with_delimiters(vec![' ', ':']);
        let expected = LcsMap {
            seq: Vec::new(),
            line_id: 0,
            delimiters: vec![' ', ':'],
        };
        assert_eq!(map, expected);

        map.insert(inputs[0]);
        let expected = LcsMap {
            seq: vec![LcsObject {
                tokens: [
                    "Jan", "22", "04", "11", "04", "combo", "syslogd", "1.4.1", "restart.",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
                lines_ids: vec![0],
            }],
            line_id: 1,
            delimiters: vec![' ', ':'],
        };
        assert_eq!(map, expected);
    }
}
