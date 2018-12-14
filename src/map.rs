use crate::{object::LcsObject, tokenize, LcsSeq, LineId};

#[derive(Default, Debug)]
pub struct LcsMap {
    seq: Vec<LcsObject>,
    line_id: LineId,
}

impl LcsMap {
    pub fn new() -> LcsMap {
        Default::default()
    }

    pub fn insert(&mut self, entry: &str) {
        let tokenized: LcsSeq = tokenize(entry).map(|token| token.to_string()).collect();

        self.line_id += 1;
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
