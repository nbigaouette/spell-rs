use serde_derive::*;

use crate::{object::LcsObject, tokenize, LcsSeq, LineId};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LcsMap {
    pub seq: Vec<LcsObject>,
    pub line_id: LineId,
}

impl LcsMap {
    pub fn new() -> LcsMap {
        Default::default()
    }

    pub fn insert(&mut self, entry: &str) {
        let tokenized: LcsSeq = tokenize(entry).map(|token| token.to_string()).collect();

        let line_id = self.line_id;

        match self.get_match(&tokenized) {
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

    fn get_match(&mut self, tokenized: &LcsSeq) -> Option<&mut LcsObject> {
        let (best_match, _best_match_length) =
            self.seq
                .iter_mut()
                .fold((None, 0), |(best_match, best_match_length), obj| {
                    // Pruning as described in paper
                    if obj.length() < tokenized.len() / 2 || obj.length() > tokenized.len() * 2 {
                        (best_match, best_match_length)
                    } else {
                        let l = obj.get_lcs(tokenized);
                        if l >= tokenized.len() / 2 && l > best_match_length {
                            (Some(obj), l)
                        } else {
                            (best_match, best_match_length)
                        }
                    }
                });
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
    // use crate::{object::LcsObject, LineId};

    #[test]
    fn compare_java() {
        let mut map = LcsMap::new();

        map.insert("Jan 22 04:11:04 combo syslogd 1.4.1: restart.");
        let to_check = map.to_string();
        let expected = "\t1 Objects in the LCSMap

\tObject 0:
\t\tJan 22 04:11:04 combo syslogd 1.4.1: restart.
\t\t{0}

\t1 total entries found, 1 expected.";
        assert_eq!(to_check, expected);

        map.insert("Jan 22 04:11:04 combo logrotate: ALERT exited abnormally with [1]");
        let to_check = map.to_string();
        let expected = "\t2 Objects in the LCSMap

\tObject 0:
\t\tJan 22 04:11:04 combo syslogd 1.4.1: restart.
\t\t{0}
\tObject 1:
\t\tJan 22 04:11:04 combo logrotate: ALERT exited abnormally with [1]
\t\t{1}

\t2 total entries found, 2 expected.";
        assert_eq!(to_check, expected);

        map.insert(
            "Jan 22 04:16:40 combo su(pam_unix)[21719]: session opened for user news by (uid=0)",
        );
        let to_check = map.to_string();
        let expected = "\t3 Objects in the LCSMap

\tObject 0:
\t\tJan 22 04:11:04 combo syslogd 1.4.1: restart.
\t\t{0}
\tObject 1:
\t\tJan 22 04:11:04 combo logrotate: ALERT exited abnormally with [1]
\t\t{1}
\tObject 2:
\t\tJan 22 04:16:40 combo su(pam_unix)[21719]: session opened for user news by (uid=0)
\t\t{2}

\t3 total entries found, 3 expected.";
        assert_eq!(to_check, expected);

        map.insert("Jan 22 04:16:41 combo su(pam_unix)[21719]: session closed for user news");
        let to_check = map.to_string();
        let expected = "\t3 Objects in the LCSMap

\tObject 0:
\t\tJan 22 04:11:04 combo syslogd 1.4.1: restart.
\t\t{0}
\tObject 1:
\t\tJan 22 04:11:04 combo logrotate: ALERT exited abnormally with [1]
\t\t{1}
\tObject 2:
\t\tJan 22 * combo su(pam_unix)[21719]: session * for user news
\t\t{2, 3}

\t4 total entries found, 4 expected.";
        assert_eq!(to_check, expected);

        map.insert("Jan 22 05:23:19 combo sshd(pam_unix)[24892]: check pass; user unknown");
        let to_check = map.to_string();
        let expected = "\t3 Objects in the LCSMap

\tObject 0:
\t\tJan 22 04:11:04 combo syslogd 1.4.1: restart.
\t\t{0}
\tObject 1:
\t\tJan 22 04:11:04 combo logrotate: ALERT exited abnormally with [1]
\t\t{1}
\tObject 2:
\t\tJan 22 * combo * user *
\t\t{2, 3, 4}

\t5 total entries found, 5 expected.";
        assert_eq!(to_check, expected);

        map.insert("Jan 22 05:23:19 combo sshd(pam_unix)[24892]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=server3.sugolan.hu");
        let to_check = map.to_string();
        let expected = "\t4 Objects in the LCSMap

\tObject 0:
\t\tJan 22 04:11:04 combo syslogd 1.4.1: restart.
\t\t{0}
\tObject 1:
\t\tJan 22 04:11:04 combo logrotate: ALERT exited abnormally with [1]
\t\t{1}
\tObject 2:
\t\tJan 22 * combo * user *
\t\t{2, 3, 4}
\tObject 3:
\t\tJan 22 05:23:19 combo sshd(pam_unix)[24892]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=server3.sugolan.hu
\t\t{5}

\t6 total entries found, 6 expected.";
        assert_eq!(to_check, expected);
    }

    #[test]
    fn parse_log() {
        let mut map = LcsMap::new();
        let expected = LcsMap {
            seq: Vec::new(),
            line_id: 0,
        };
        assert_eq!(map, expected);

        map.insert("Jan 22 04:11:04 combo syslogd 1.4.1: restart.");
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
        };
        assert_eq!(map, expected);

        map.insert("Jan 22 04:11:04 combo logrotate: ALERT exited abnormally with [1]");
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
        };
        assert_eq!(map, expected);

        map.insert(
            "Jan 22 04:16:40 combo su(pam_unix)[21719]: session opened for user news by (uid=0)",
        );
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
        };
        assert_eq!(map, expected);

        map.insert("Jan 22 04:16:41 combo su(pam_unix)[21719]: session closed for user news");
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
        };
        assert_eq!(map, expected);

        map.insert("Jan 22 05:23:19 combo sshd(pam_unix)[24892]: check pass; user unknown");
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
        };
        assert_eq!(map, expected);

        map.insert("Jan 22 05:23:19 combo sshd(pam_unix)[24892]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=server3.sugolan.hu");
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
        };
        assert_eq!(map, expected);
    }
}
