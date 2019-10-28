use automaton;
use std::io::stdin;
use std::io;

fn prompt(text: &str) -> io::Result<Vec<char>> {
    println!("{}", text);
    let mut pattern = String::new();
    stdin().read_line(&mut pattern)?;

    let pattern = pattern.trim_end_matches(&['\r', '\n'][..]);

    Ok(pattern.chars().collect())
}

fn main() -> io::Result<()> {
    let pattern = prompt("Enter pattern:")?;
    let alphabet = prompt("Enter alphabet:")?;
    let text = prompt("Enter text to search:")?;

    let ref matcher_aut = automaton::Matcher::new(&alphabet, &pattern, &text);
    let ref matcher_kmp = knuth_morris_pratt::Matcher::new(&alphabet, &pattern, &text);

    println!("{:?} {:?}", matcher_aut, matcher_kmp);
    for (match1, match2) in matcher_aut.iter().zip(matcher_kmp) {
        assert_eq!(match1, match2);
        let (start, end) = match1;
        println!("match found at {}..{}", start, end);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    macro_rules! make_test {
        ($p: path, $pattern: expr, $alphabet: expr, $text: expr, $expected: expr) => {
            let pattern: Vec<_> = $pattern.chars().collect();
            let alphabet: Vec<_> = $alphabet.chars().collect();
            let text: Vec<_> = $text.chars().collect();
            let matcher = $p(&alphabet, &pattern, &text);

            let result: Vec<_> = matcher.iter().collect();

            assert!(result.iter().all(|&(start, end)| end - start == pattern.len()));
            assert_eq!(result.iter().map(|x|x.0).collect::<Vec<_>>(), $expected, "searching for {} in {}", $pattern, $text);
        }
    }

    macro_rules! make_tests {
        ($(test $name: ident: $args: tt)*) => {
            mod automaton_tests {
                $(
                    make_tests!(automaton::Matcher::new, $name, $args);
                )*
            }

            mod kmp_tests {
                $(
                    make_tests!(knuth_morris_pratt::Matcher::new, $name, $args);
                )*
            }
        };

        ($p: path, $name: ident, {pattern = $pattern: expr, alphabet = $alphabet: expr, text = $text: expr, expected = $expected: expr}) => {
            #[test]
            fn $name() {
                make_test!(automaton::Matcher::new, $pattern, $alphabet, $text, $expected);
            }

        }
    }

    make_tests! {
        test aaa: { pattern = "aaa", alphabet = "a", text = "aaa", expected = &[0usize] }
        test aaaa: { pattern = "aaa", alphabet = "a", text = "aaaa", expected = &[0usize, 1] }
        test aa: { pattern = "aaa", alphabet = "a", text = "aa", expected = &[] }

        test abab: { pattern = "abab", alphabet = "ab", text = "abababab", expected = &[0usize,2,4] }

        test greek1: { pattern = "δ", alphabet = "αβγδ", text = "αβαβγβαβαβαβαβγ", expected = &[] }
        test greek2: { pattern = "γδ", alphabet = "αβγδ", text = "αβαβγβαβαβαβαβγ", expected = &[] }
        test greek3: { pattern = "αβ", alphabet = "αβγδ", text = "αβαβγβαβαβαβαβγ", expected = &[0usize,2,6,8,10,12] }
        test greek4: { pattern = "αβαβ", alphabet = "αβγδ", text = "αβαβγβαβαβαβαβγ", expected = &[0usize,6,8,10] }

        test numbers1: { pattern = "0", alphabet = "7890", text = "78787999997878787879", expected = &[] }
        test numbers2: { pattern = "9", alphabet = "7890", text = "78787999997878787879", expected = &[5usize,6,7,8,9,19] }
        test numbers3: { pattern = "787", alphabet = "7890", text = "78787999997878787879", expected = &[0usize,2,10,12,14,16] }
        test numbers4: { pattern = "99", alphabet = "7890", text = "78787999997878787879", expected = &[5usize,6,7,8] }
        test numbers5: { pattern = "879", alphabet = "7890", text = "78787999997878787879", expected = &[3usize,17] }
        test numbers6: { pattern = "978", alphabet = "7890", text = "78787999997878787879", expected = &[9usize] }

        test xyzw1: { pattern = "W", alphabet = "XYZW", text = "XYXYYXYYYXYYYYXYXYXYXZYXZ", expected = &[] }
        test xyzw2: { pattern = "YW", alphabet = "XYZW", text = "XYXYYXYYYXYYYYXYXYXYXZYXZ", expected = &[] }
        test xyzw3: { pattern = "YX", alphabet = "XYZW", text = "XYXYYXYYYXYYYYXYXYXYXZYXZ", expected = &[1usize,4,8,13,15,17,19,22] }
        test xyzw4: { pattern = "YY", alphabet = "XYZW", text = "XYXYYXYYYXYYYYXYXYXYXZYXZ", expected = &[3usize,6,7,10,11,12] }
        test xyzw5: { pattern = "XYX", alphabet = "XYZW", text = "XYXYYXYYYXYYYYXYXYXYXZYXZ", expected = &[0usize,14,16,18] }
        test xyzw6: { pattern = "XYXY", alphabet = "XYZW", text = "XYXYYXYYYXYYYYXYXYXYXZYXZ", expected = &[0usize,14,16] }
    }
}
