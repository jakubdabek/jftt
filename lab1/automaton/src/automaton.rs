use std::collections::BTreeMap;
use std::cmp::min;

#[derive(Debug, Clone)]
struct Automaton<'a> {
    alphabet: &'a[char],
    pattern: &'a[char],
    transitions: BTreeMap<i32, BTreeMap<char, i32>>,
}

impl<'a> Automaton<'a> {
    fn new(alphabet: &'a[char], pattern: &'a[char]) -> Automaton<'a> {
        Automaton {
            alphabet,
            pattern,
            transitions: BTreeMap::new(),
        }
    }

    fn accept(&self, current_state: i32, a: char) -> i32 {
        self.transitions[&current_state][&a]
    }

    fn is_accepting(&self, current_state: i32) -> bool {
        current_state as usize == self.pattern.len()
    }
}

fn create_automaton<'a>(alphabet: &'a[char], pattern: &'a[char]) -> Automaton<'a> {
    let mut automaton = Automaton::new(alphabet, pattern);

    let m = pattern.len() + 1;
    let mut pattern_prefix = Vec::with_capacity(pattern.len() + 1);
    for q in 0..m {
        pattern_prefix.clear();
        pattern_prefix.extend_from_slice(&pattern[..q]);
        let original_length = pattern_prefix.len();

        for &a in alphabet {
            pattern_prefix.truncate(original_length);
            pattern_prefix.push(a);
            let mut k = min(q+1, m-1) + 1;
            loop {
                k -= 1;
                if pattern_prefix.ends_with(&pattern[..k]) {
                    break;
                }
            }

            automaton.transitions.entry(q as i32).or_insert_with(||BTreeMap::new()).insert(a, k as i32);
        }
    };

    automaton
}

#[derive(Debug)]
pub struct Matcher<'a> {
    text: &'a[char],
    automaton: Automaton<'a>,
}

pub struct MatcherIter<'a> {
    matcher: &'a Matcher<'a>,
    current_state: i32,
    current_index: usize,
}

impl Matcher<'_> {
    pub fn new<'a>(alphabet: &'a[char], pattern: &'a[char], text: &'a[char]) -> Matcher<'a> {
        Matcher {
            text,
            automaton: create_automaton(alphabet, pattern),
        }
    }

    pub fn iter(&self) -> MatcherIter {
        MatcherIter {
            matcher: self,
            current_state: 0,
            current_index: 0,
        }
    }
}

impl Iterator for MatcherIter<'_> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_index < self.matcher.text.len() {
            self.current_state = self.matcher.automaton.accept(self.current_state, self.matcher.text[self.current_index]);
//            println!("{} {} {}", self.current_index, self.text.chars().nth(self.current_index).unwrap(), self.automaton.state);
            self.current_index += 1;
            if self.matcher.automaton.is_accepting(self.current_state) {
                return Some((self.current_index - self.matcher.automaton.pattern.len(), self.current_index))
            }
        }

        None
    }
}

impl <'a> IntoIterator for &'a Matcher<'a> {
    type Item = <MatcherIter<'a> as Iterator>::Item;
    type IntoIter = MatcherIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
