use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Matcher<'a> {
    alphabet: &'a[char],
    pattern: &'a[char],
    text: &'a[char],
    transitions: BTreeMap<i32, i32>,
}

pub struct MatcherIter<'a> {
    matcher: &'a Matcher<'a>,
    current_state: i32,
    current_index: usize,
}

fn make_transitions(pattern: &[char]) -> BTreeMap<i32, i32> {
    let m = pattern.len();
    let mut transitions = BTreeMap::new();
    transitions.insert(1, 0);
    let mut k = 0i32;

    for q in 2..=(m as i32) {
        while k > 0 && pattern[k as usize] != pattern[q as usize - 1] {
            k = transitions[&k];
        }

        if pattern[k as usize] == pattern[q as usize - 1] {
            k += 1;
        }

        transitions.insert(q, k);
    }

    transitions
}

impl Matcher<'_> {
    pub fn new<'a>(alphabet: &'a[char], pattern: &'a[char], text: &'a[char]) -> Matcher<'a> {
        Matcher {
            alphabet,
            pattern,
            text,
            transitions: make_transitions(pattern),
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
        let ref mut q = self.current_state;
        while self.current_index < self.matcher.text.len() {
            while *q > 0 && self.matcher.pattern[*q as usize] != self.matcher.text[self.current_index] {
                *q = self.matcher.transitions[q];
            }

            if self.matcher.pattern[*q as usize] == self.matcher.text[self.current_index] {
                *q += 1;
            }

            self.current_index += 1;

            if *q as usize == self.matcher.pattern.len() {
                *q = self.matcher.transitions[q];
                return Some((self.current_index - self.matcher.pattern.len(), self.current_index))
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
