```toml
[fuzz]
exec = ["cargo", "run", "--release", "--", "generate"]
env = {}

[judge]
exec = ["cargo", "run", "--release", "--quiet", "--", "validate"]

[problem]
points = 5
difficulty = 1
```

# 🧬 DNA Similarity

Given two DNA sequences, determine their similarity from **0.0 to 1.0** based on what **fraction of nucleotides match**. For example:

* `TGAC` and `TGAC` is an exact match so their similarity is `1.0`.
* `CGAC` and `TGTC` have the same nucleotide in the 2nd and 4th position meaning that half of them match giving a similarity of `0.5`
* `CAGT` and `CTTG` only have the first nucleotide matching giving them a similarity of `0.25`

## Input
The input is two lines of equal length representing either DNA sequence. Each line contains a combination of `A`, `C`, `G` and `T` with each character representing a nucleotide.
```
TTATATGATTCGCTGACCCCTACATCTAGTAATCAG
GTTTGCTGGTAGGTGAAACGCTAGATTTATGGCCGT
```

## Output
Your output is a number from 0.0 to 1.0 representing the similarity.

