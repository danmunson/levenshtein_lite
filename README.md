# Levenshtein Lite

This crate provides a no-frills implementatation of a Levenshtein Automata and Levenshtein Distance function.

# Example

```rust
use levenshtein_lite::{LevenshteinAutomata, levenshtein_distance};

let lda = LevenshteinAutomata::new("abc", 1);
assert!(lda.check("abx"));
assert!(!lda.check("axx"));

assert!(levenshtein_distance("abc", "abx") == 1);
assert!(levenshtein_distance("abc", "axx") == 2);
```
