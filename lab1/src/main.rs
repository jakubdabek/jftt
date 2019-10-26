use automaton;
fn main() {
    let pattern: Vec<_> = "aaa".chars().collect();
    let alphabet: Vec<_> = "aąbcd".chars().collect();
    let text: Vec<_> = "abacaaabaaabaaaąbabacb".chars().collect();
    let matcher = automaton::Matcher::new(&alphabet, &pattern, &text);

    println!("{:?}", &matcher);
    for (s, e) in &matcher {
        println!("match found at {}..{}", s, e);
    }
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

//    trace_macros!(true);

    make_tests! {
        test aaa: { pattern = "aaa", alphabet = "a", text = "aaa", expected = &[0usize] }
        test aaaa: { pattern = "aaa", alphabet = "a", text = "aaaa", expected = &[0usize, 1] }
        test aa: { pattern = "aaa", alphabet = "a", text = "aa", expected = &[] }

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


//    macro_rules! make_tests {
//        ($name: ident, $pattern: expr, $alphabet: expr, $text: expr, $expected: expr) => {
//            mod $name {
//                #[test]
//                fn test_automaton() {
//                    make_test!(automaton::Matcher::new, $pattern, $alphabet, $text, $expected);
//                }
//
//                #[test]
//                fn test_kmp() {
//                    make_test!(knuth_morris_pratt::Matcher::new, $pattern, $alphabet, $text, $expected);
//                }
//            }
//        };
//    }
//
////    trace_macros!(false);
//
//    make_tests!(aaa, "aaa", "a", "aaa", &[0usize]);
//    make_tests!(aaaa, "aaa", "a", "aaaa", &[0usize, 1]);
//    make_tests!(aa, "aaa", "a", "aa", &[]);
//
//    make_tests!(greek1, "δ", "αβγδ", "αβαβγβαβαβαβαβγ", &[]);
//    make_tests!(greek2, "γδ", "αβγδ", "αβαβγβαβαβαβαβγ", &[]);
//    make_tests!(greek3, "αβ", "αβγδ", "αβαβγβαβαβαβαβγ", &[0usize,2,6,8,10,12]);
//    make_tests!(greek4, "αβαβ", "αβγδ", "αβαβγβαβαβαβαβγ", &[0usize,6,8,10]);
//
//    make_tests!(numbers1, "0", "7890", "78787999997878787879", &[]);
//    make_tests!(numbers2, "9", "7890", "78787999997878787879", &[5usize,6,7,8,9,19]);
//    make_tests!(numbers3, "787", "7890", "78787999997878787879", &[0usize,2,10,12,14,16]);
//    make_tests!(numbers4, "99", "7890", "78787999997878787879", &[5usize,6,7,8]);
//    make_tests!(numbers5, "879", "7890", "78787999997878787879", &[3usize,17]);
//    make_tests!(numbers6, "978", "7890", "78787999997878787879", &[9usize]);
//
//    make_tests!(xyzw1, "W", "XYZW", "XYXYYXYYYXYYYYXYXYXYXZYXZ", &[]);
//    make_tests!(xyzw2, "YW", "XYZW", "XYXYYXYYYXYYYYXYXYXYXZYXZ", &[]);
//    make_tests!(xyzw3, "YX", "XYZW", "XYXYYXYYYXYYYYXYXYXYXZYXZ", &[1usize,4,8,13,15,17,19,22]);
//    make_tests!(xyzw4, "YY", "XYZW", "XYXYYXYYYXYYYYXYXYXYXZYXZ", &[3usize,6,7,10,11,12]);
//    make_tests!(xyzw5, "XYX", "XYZW", "XYXYYXYYYXYYYYXYXYXYXZYXZ", &[0usize,14,16,18]);
//    make_tests!(xyzw6, "XYXY", "XYZW", "XYXYYXYYYXYYYYXYXYXYXZYXZ", &[0usize,14,16]);
}
