use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
enum TChar {
    Char(char),
    Any
}
type Xi = usize;
type Si = usize;
type Dacc = i8;
type SiDaccPairs = Vec<(Si, Dacc)>;
type Accepting = bool;
type LookupKey = (Xi, Accepting, Vec<(TChar, SiDaccPairs)>);
type NodeLookup = HashMap<LookupKey, Rc<BuildNode>>;

struct BuildNode {
    transitions: HashMap<TChar, Rc<BuildNode>>,
    accepting: bool
}

struct TempTracker {
    transitions: HashMap<TChar, SiDaccPairs>
}

impl TempTracker {
    pub fn new() -> Self {
        TempTracker {
            transitions: HashMap::new()
        }
    }

    pub fn add(&mut self, key: TChar, pair: (Si, Dacc)) {
        if let Some(v) = self.transitions.get_mut(&key) {
            v.push(pair);
        } else {
            let mut v = vec![];
            v.push(pair);
            self.transitions.insert(key, v);
        }
    }

    pub fn key(&self) -> Vec<(TChar, SiDaccPairs)> {
        self.transitions.iter()
                        .map(|(&tchar, pairs)| (tchar, pairs.clone()))
                        .collect()
    }
}

fn build<'a>(
    src: &Vec<char>,
    x_i: Xi,
    si_dacc: SiDaccPairs,
    max_distance: i8,
    lookup: &'a RefCell<NodeLookup>
) -> Rc<BuildNode> {
    let mut transitions = TempTracker::new();

    let mut accepting = false;
    for (si, dacc) in si_dacc {
        if si >= src.len() {
            if dacc < max_distance {
                // in case of a mismatch with edit budget remaining,
                // match any character at the cost of one edit,
                // without advancing the src comparison position
                transitions.add(TChar::Any, (si, dacc + 1));
            }
            // if the src comparison position is past the end of the string
            // then this position is accepting, i.e. if the input string terminates
            // while in this state, then the edit distance < max_distance
            accepting = true;
            continue;
        }
        let match_char = TChar::Char(src[si]);
        // match: advance the src comparison position at zero cost
        transitions.add(match_char, (si + 1, dacc));

        // we need to "look ahead" to match when deletions occur
        for offset_i8 in 1..(max_distance - dacc + 1) {
            let offset = offset_i8 as usize;
            if si + offset >= src.len() {
                // if deleting within our edit budget moves the
                // src comparison position past the end of the string
                // then this position is accepting
                accepting = true;
                continue;
            }
            // in case of a deletion, try to match against the character
            // "offset" positions forward
            let cmp_char = TChar::Char(src[si + offset]);
            if cmp_char != match_char {
                // if match during lookahead, advance the src comparison position
                // by 1, at the cost of "offset" edits
                transitions.add(cmp_char, (si + offset + 1, dacc + offset_i8));
                // this specific character could also represent an insertion
                // or a substitution, so account for that too
                // by:
                //  insert: retain src comparison position at cost of 1 edit
                //  sub: advancing src comparison position by 1, at the cost of 1 edit
                transitions.add(cmp_char, (si, dacc + 1));
                transitions.add(cmp_char, (si + 1, dacc + 1));
            }
        }

        if dacc < max_distance {
            // account for a possible insertion by matching against ANY;
            // retain src comparison position at cost of 1 edit
            transitions.add(TChar::Any, (si, dacc + 1));
            // account for a possible substitution by matching against ANY;
            // advance src comparison position by 1 at the cost of 1 edit
            transitions.add(TChar::Any, (si + 1, dacc + 1));
            // also, if this substitution would move the src comparison position
            // past the end of the src string, this state is accepting
            if si + 1 >= src.len() {
                accepting = true;
            }
        }
    }

    let key: LookupKey = (x_i, accepting, transitions.key());
    if lookup.borrow().contains_key(&key) {
        lookup.borrow().get(&key).unwrap().clone()
    } else {
        let mut next_transitions = HashMap::new();
        for (tchar, si_dacc) in transitions.key() {
            let next_node = {
                build(src, x_i + 1, si_dacc, max_distance, lookup)
            };
            next_transitions.insert(tchar, next_node);
        }
        lookup.borrow_mut().insert(key.clone(), Rc::new(BuildNode {
            transitions: next_transitions,
            accepting
        }));
        lookup.borrow().get(&key).unwrap().clone()
    }
}

struct Head {
    has_children: bool,
    trns_start: usize,
    trns_end: usize,
    accepting: bool
}

struct Transition {
    tchar: TChar,
    points_to: usize
}

fn flatten(
    node: Rc<BuildNode>,
    head_array: &RefCell<Vec<Head>>,
    trns_array: &RefCell<Vec<Transition>>,
    lookup: &RefCell<HashMap<usize, usize>>
) -> usize {
    let node_ptr = Rc::as_ptr(&node) as usize;
    {
        if lookup.borrow().contains_key(&node_ptr) {
            // this node has already been placed
            return *lookup.borrow().get(&node_ptr).unwrap();
        }
    }
    let (head_idx, trns_start) = {
        (head_array.borrow().len(), trns_array.borrow().len())
    };
    let has_children = node.transitions.len() > 0;
    let head = Head {
        has_children,
        trns_start,
        trns_end: trns_start + node.transitions.len() - 1,
        accepting: node.accepting
    };
    {
        head_array.borrow_mut().push(head);
        if !has_children {
            return head_idx;
        }
        let mut trns_mut = trns_array.borrow_mut();
        for (tchar, _) in node.transitions.iter() {
            trns_mut.push(Transition {
                tchar: tchar.clone(),
                points_to: 0 //tmp
            });
        }
    }
    let (start, end) = {
        let headref = &head_array.borrow()[head_idx];
        (headref.trns_start, headref.trns_end)
    };
    for idx in start..end+1 {
        let tchar = { trns_array.borrow()[idx].tchar };
        let points_to = flatten(node.transitions[&tchar].clone(), head_array, trns_array, lookup);
        let trn = &mut trns_array.borrow_mut()[idx];
        trn.points_to = points_to;
    }
    // return the head_index so the transition
    // knows where to point to, after caching the vale
    lookup.borrow_mut().insert(node_ptr, head_idx);
    head_idx
}

pub struct LevenshteinAutomata {
    src: String,
    max_distance: i8,
    heads: Vec<Head>,
    transitions: Vec<Transition>
}

impl LevenshteinAutomata {
    pub fn new(src: &str, max_distance: i8) -> Self {
        let lookup = RefCell::new(HashMap::new());
        let head = build(&src.chars().collect(),
                                        0,
                                        vec![(0, 0)],
                                        max_distance,
                                        &lookup);
        let head_array = RefCell::new(vec![]);
        let trns_array = RefCell::new(vec![]);
        let lookup = RefCell::new(HashMap::new());
        let _ = flatten(head, &head_array, &trns_array, &lookup);
        LevenshteinAutomata {
            src: src.to_string(),
            max_distance,
            heads: head_array.into_inner(),
            transitions: trns_array.into_inner()
        }
    }

    pub fn check(&self, input: &str) -> bool {
        let mut head = &self.heads[0];
        let mut accepting = head.accepting;
        for c in input.chars() {
            if !head.has_children {
                return false;
            }
            let mut match_idx: Option<usize> = Option::None;
            for idx in head.trns_start..head.trns_end+1 {
                let tref = &self.transitions[idx];
                match tref.tchar {
                    TChar::Char(tc) if tc == c => {
                        match_idx = Some(tref.points_to);
                        break;
                    },
                    TChar::Any => match match_idx {
                        None => {
                            match_idx = Some(tref.points_to)
                        },
                        _ => ()
                    },
                    _ => ()
                };
            }
            match match_idx {
                Some(i) => {
                    head = &self.heads[i];
                    accepting = head.accepting;
                },
                None => {
                    // we have no valid transitions from here,
                    // hence this isn't a match
                    return false;
                }
            }
        }
        accepting
    }

    pub fn details(&self) -> (&str, i8) {
        (&self.src, self.max_distance)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let lda = LevenshteinAutomata::new("abc", 2);

        let expect_match = vec![
            "abc", "ac", "a", "abcxx", "axc",
            "bbc", "aaa", "ccc", "abbbc", "aabbc"
        ];

        for input in expect_match {
            println!("Expecting match against '{}'", input);
            assert!(lda.check(input));
        }

        let expect_no_match = vec![
            "", "xxx", "aabbcc", "abcabc"
        ];

        for input in expect_no_match {
            println!("Expecting no match against '{}'", input);
            assert!(!lda.check(input))
        }
    }
}
