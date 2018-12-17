# spell-rs

_spell-rs_ is a _Spell_ implementation in Rust.

Spell is a _**S**treaming **P**arser for **E**vent **L**ogs using an **L**CS_ (Longest Common Subsequence).

[![Build Status](https://travis-ci.org/nbigaouette/spell-rs.svg?branch=master)](https://travis-ci.org/nbigaouette/spell-rs)
## Reference

Min Du and Feifei Li. Spell: Streaming Parsing of System Event Logs. 2016 IEEE 16th International
Conference on Data Mining (ICDM) pp. 859-864, Barcelona, Spain, December, 2016.
[DOI: 10.1109/ICDM.2016.0103](https://www.doi.org/10.1109/ICDM.2016.0103).
https://ieeexplore.ieee.org/document/7837916

* Original paper: https://www.cs.utah.edu/~lifeifei/papers/spell.pdf
* Presentation slides: https://www.cs.utah.edu/~mind/papers/spell_slides.pdf

## Details

Spell ingests log lines one by one and build up a list of formatting strings that can be used to
analyze the logs. For example (see [presentation slides](https://www.cs.utah.edu/~mind/papers/spell_slides.pdf),
page 19), given the two log entries:

```text
Temperature (41C) exceeds warning threshold
Temperature (42C, 43C) exceeds warning threshold
```

the LCS is:

```text
Temperature * exceeds warning threshold
```

which can then be used to analyze the logs.

As of cedf57a7a73b1052de937d13150c2b9d8a03237c, the Rust implementation gives the exact same results
as the [Java one](https://github.com/EddiePi/Spell).

## Other Implementations

* [Spell](https://github.com/EddiePi/Spell), Java version
* [pyspell](https://github.com/bave/pyspell), a Python version
* [spell](https://github.com/logforensicator/spell), another Python version

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your option.
