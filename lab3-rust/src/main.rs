extern crate ast;
extern crate num;
extern crate regex;

use lalrpop_util::lalrpop_mod;
use std::io::BufRead;
use regex::Regex;
use ast::{Expr, ExprEvaluator, RPNEvaluator};

lalrpop_mod!(pub calc);

fn parse<'input>(
    input: &'input str,
) -> Result<Box<Expr>, lalrpop_util::ParseError<usize, calc::Token<'input>, &'static str>> {
    calc::ExprParser::new().parse(input)
}

fn main() {
    let stdin = std::io::stdin();

    let comment_regex = Regex::new(r"^#.*").unwrap();
    let mut current_line = String::new();
    let mut broken_line = false;

    for line in stdin.lock().lines() {
        let line = &line.unwrap();

        if line.ends_with('\\') {
            if !broken_line {
                current_line.clear();
            }
            current_line.push_str(&line[..line.len()-1]);
            broken_line = true;
            continue;
        } else if broken_line {
            current_line.push_str(&line);
            broken_line = false;
        } else {
            current_line.clear();
            current_line.push_str(&line);
        }

        if comment_regex.is_match(&current_line) {
            continue;
        }

        println!("{:?}", current_line);
        match parse(&current_line) {
            Ok(result) => {
                fn eval(result: &Expr) -> bool {
                    let mut value_evaluator = ExprEvaluator::new();

                    result.walk(&mut value_evaluator);
                    let value = value_evaluator.value();

                    match value {
                        Ok(value) => {
                            println!("= {}", value);
                            true
                        },
                        Err(error) => {
                            println!("Error: {}", error);
                            false
                        },
                    }
                }

                if eval(&result) {
                    let mut rpn_evaluator = RPNEvaluator::new();
                    result.walk(&mut rpn_evaluator);
                    let rpn = rpn_evaluator.value();

                    println!("{}", rpn);
                }
            },
            Err(_err) => println!("Syntax error"),
        };
    };
}

#[cfg(test)]
mod general_tests {
    use super::parse;

    #[test]
    fn invalid_symbol() {
        assert!(parse("a").is_err());
        assert!(parse("=").is_err());
        assert!(parse("!").is_err());
        assert!(parse("abcdef").is_err());
        assert!(parse("3+a").is_err());
        assert!(parse("1+10*8-!").is_err());
        assert!(parse("(!)").is_err());
        assert!(parse("(( !  ))").is_err());
    }

    #[test]
    fn invalid_expression() {
        assert!(parse("1-+1").is_err());
        assert!(parse("1++1").is_err());
        assert!(parse("1-+1").is_err());
        assert!(parse("1-+-1").is_err());
        assert!(parse("*1").is_err());
        assert!(parse("1+").is_err());
        assert!(parse("1-").is_err());
        assert!(parse("1*").is_err());
        assert!(parse("1/").is_err());
        assert!(parse("(").is_err());
        assert!(parse(")").is_err());
        assert!(parse("()").is_err());
        assert!(parse("(1))").is_err());
        assert!(parse("(1").is_err());
        assert!(parse("((1)").is_err());
        assert!(parse("1+()").is_err());
        assert!(parse("((1+)1)").is_err());
        assert!(parse("((1+)-1)").is_err());
        assert!(parse("((1)-1-)").is_err());
    }
}

#[cfg(test)]
mod value_tests {
    use super::calc;
    use ast::ExprEvaluator;

    fn parse<'input>(
        input: &'input str,
    ) -> Result<i32, lalrpop_util::ParseError<usize, calc::Token<'input>, &'static str>> {
        let mut evaluator = ExprEvaluator::new();
        calc::ExprParser::new().parse(input)?.walk(&mut evaluator);
        Ok(evaluator.value().unwrap())
    }

    fn eq(input: &str, expected: i32) {
        assert_eq!(parse(input).unwrap(), expected);
    }

    mod without_negation {
        use super::eq;

        #[test]
        fn numbers() {
            eq("0", 0);
            eq("3", 3);
            eq("10", 10);
            eq("12345", 12345);
            eq("987", 987);
        }

        #[test]
        fn parenthesized_numbers() {
            eq("(0)", 0);
            eq("(3)", 3);
            eq("(10)", 10);
            eq("(12345)", 12345);
            eq("(987)", 987);
            eq("((987))", 987);
            eq("(((987)))", 987);
            eq("((((987))))", 987);
        }

        #[test]
        fn addition() {
            eq("1+2", 3);
            eq("1+4", 5);
            eq("10+10", 20);
            eq("1+2+3", 6);
            eq("1+2+3+4", 10);
            eq("1+(2+3+4)", 10);
            eq("1+((2+3)+4)", 10);
            eq("1+(2+(3+4))", 10);
        }

        #[test]
        fn subtraction() {
            eq("2-1", 1);
            eq("1-2", -1);
            eq("4-1", 3);
            eq("1-4", -3);
            eq("10-10", 0);
            eq("3-2-1", 0);
            eq("1-2-3-4", -8);
            eq("1-(2-3-4)", 6);
            eq("1-((2-3)-4)", 6);
            eq("1-(2-(3-4))", -2);
        }

        #[test]
        fn multiplication() {
            eq("1*2", 2);
            eq("1*4", 4);
            eq("10*10", 100);
            eq("1*2*3", 6);
            eq("1*2*3*4", 24);
            eq("1*(2*3*4)", 24);
            eq("1*((2*3)*4)", 24);
            eq("1*(2*(3*4))", 24);
        }

        #[test]
        fn division() {
            eq("1/2", 0);
            eq("1/4", 0);
            eq("10/10", 1);
            eq("1/2/3", 0);
            eq("1/2/3/4", 0);
            eq("10/1", 10);
            eq("10/2", 5);
            eq("10/3", 3);
            eq("10/4", 2);
            eq("10/2/2", 2);
            eq("10/(10/2)", 2);
            eq("10/(10/3)", 3);
            eq("10/(10/4)", 5);
        }

        #[test]
        fn modulo() {
            eq("1%2", 1);
            eq("1%4", 1);
            eq("10%10", 0);
            eq("1%2%3", 1);
            eq("1%2%3%4", 1);
            eq("10%1", 0);
            eq("10%2", 0);
            eq("10%3", 1);
            eq("10%4", 2);
            eq("10%2%2", 0);
            eq("10%4%3", 2);
            eq("10%4%2", 0);
            eq("10%(10%3)", 0);
            eq("10%(10%4)", 0);
        }

        #[test]
        fn exponentiation() {
            eq("2^0", 1);
            eq("2^1", 2);
            eq("2^2", 4);
            eq("2^3", 8);
            eq("2^4", 16);
            eq("0^1", 0);
            eq("0^5", 0);
            eq("0^10", 0);
            eq("3^3", 27);
            eq("3^4", 81);
        }

        #[test]
        fn precedence() {
            eq("2+2*2", 6);
            eq("2+2*2+2", 8);
            eq("2*2+2", 6);
            eq("2+2*2/3+2", 5);
            eq("2+2-2", 2);
            eq("2+2-2/2", 3);
            eq("2*2-2/2", 3);
            eq("2*(2-2)/2", 0);
            eq("2*(2-2/2)", 2);
            eq("2*6/3*4", 16);
            eq("2*6/5%3*4", 8);

            eq("2^2^2", 16);
            eq("3^2^2", 81);
            eq("2^2^3", 256);
            eq("2^3^2", 512);
        }
    }

    mod with_negation {
        use super::eq;

        mod without_precedence {
            use super::eq;

            #[test]
            fn numbers() {
                eq("-0", 0);
                eq("-3", -3);
                eq("-10", -10);
                eq("-12345", -12345);
                eq("-987", -987);
            }

            #[test]
            fn parenthesized_numbers() {
                eq("(-0)", -0);
                eq("(-3)", -3);
                eq("(-10)", -10);
                eq("(-12345)", -12345);
                eq("(-987)", -987);
                eq("((-987))", -987);
                eq("(((-987)))", -987);
                eq("((((-987))))", -987);

                eq("-(0)", -0);
                eq("-(3)", -3);
                eq("-(10)", -10);
                eq("-(12345)", -12345);
                eq("-(987)", -987);
                eq("-((987))", -987);
                eq("-(((987)))", -987);
                eq("(-((987)))", -987);
                eq("((-(987)))", -987);
            }

            #[test]
            fn addition() {
                eq("-1+2", 1);
                eq("-1+4", 3);
                eq("-10+10", 0);
                eq("-1+2+3", 4);
                eq("-1+2+3+4", 8);
                eq("1+(-2+3+4)", 6);
                eq("1+((-2+3)+4)", 6);
                eq("-1+(-2+(-3+4))", -2);
            }

            #[test]
            fn subtraction() {
                eq("-2-1", -3);
                eq("-1-2", -3);
                eq("-4-1", -5);
                eq("-1-4", -5);
                eq("-10-10", -20);
                eq("-3-2-1", -6);
                eq("-1-2-3-4", -10);
                eq("1-(-2-3-4)", 10);
                eq("1-((-2-3)-4)", 10);
                eq("1-(-2-(-3-4))", -4);
            }

            #[test]
            fn multiplication() {
                eq("-1*2", -2);
                eq("-1*4", -4);
                eq("-10*10", -100);
                eq("-1*2*3", -6);
                eq("-1*2*3*4", -24);
                eq("-1*(-2*3*4)", 24);
                eq("-1*((-2*3)*4)", 24);
                eq("-1*(-2*(-3*4))", -24);
            }

            #[test]
            fn division() {
                eq("-1/2", -1);
                eq("-1/4", -1);
                eq("-10/10", -1);
                eq("-1/2/3", -1);
                eq("-1/2/3/4", -1);
                eq("-10/1", -10);
                eq("-10/2", -5);
                eq("-10/3", -4);
                eq("-10/4", -3);
                eq("-10/2/2", -3);
                eq("-10/(10/2)", -2);
                eq("-10/(-10/2)", 2);
                eq("-10/(10/3)", -4);
                eq("-10/(-10/3)", 2);
                eq("-10/(10/4)", -5);
                eq("-10/(-10/4)", 3);
            }
        }

        mod with_precedence {
            use super::eq;

            #[test]
            fn numbers() {
                eq("---0", 0);
                eq("---3", -3);
                eq("---10", -10);
                eq("---12345", -12345);
                eq("---987", -987);
            }

            #[test]
            fn parenthesized_numbers() {
                eq("(-0)", -0);
                eq("(-3)", -3);
                eq("(-10)", -10);
                eq("(-12345)", -12345);
                eq("(-987)", -987);
                eq("((-987))", -987);
                eq("(((-987)))", -987);
                eq("((((-987))))", -987);

                eq("-(0)", -0);
                eq("-(3)", -3);
                eq("-(10)", -10);
                eq("-(12345)", -12345);
                eq("-(987)", -987);
                eq("-((987))", -987);
                eq("-(((987)))", -987);
                eq("(-((987)))", -987);
                eq("((-(987)))", -987);

                eq("-(-0)", 0);
                eq("-(-3)", 3);
                eq("-(-10)", 10);
                eq("-(-12345)", 12345);
                eq("-(-987)", 987);
                eq("-((-987))", 987);
                eq("-(((-987)))", 987);
                eq("(-((-987)))", 987);
                eq("((-(-987)))", 987);
                eq("(-(-(-987)))", -987);
                eq("-(-(-(-987)))", 987);

                eq("-(--0)", -0);
                eq("-(--3)", -3);
                eq("-(--10)", -10);
                eq("-(--12345)", -12345);
                eq("-(--987)", -987);
                eq("-((--987))", -987);
                eq("-(((--987)))", -987);
                eq("(---((987)))", -987);
                eq("((---(987)))", -987);
            }

            #[test]
            fn addition() {
                eq("1+-2", -1);
                eq("-1+-2", -3);
                eq("-1+-4", -5);
                eq("1+-4", -3);
                eq("-10+-10", -20);
                eq("10+-10", 0);
                eq("-1+-2+3", 0);
                eq("1+-2+3+-4", -2);
                eq("1+-(-2+3+4)", -4);
                eq("1+(-(-2+3)+4)", 4);
                eq("1+(--(-2+3)+4)", 6);
                eq("1+-(-(-2+3)+4)", -2);
                eq("-1+-(-2+-(-3+4))", 2);
            }

            #[test]
            fn subtraction() {
                eq("-2--1", -1);
                eq("-2---1", -3);
                eq("-4--1", -3);
                eq("-4---1", -5);
                eq("--4----1", 5);
                eq("-1--4", 3);
                eq("-10-10", -20);
                eq("-3--2-1", -2);
                eq("-3--2--1", 0);
                eq("1--(-2-3-4)", -8);
                eq("1---(-2-3-4)", 10);
                eq("1--((-2-3)-4)", -8);
                eq("1--(----2---(--3-------4))", 4);
                eq("((1)-1)", 0);
                eq("((1)-(1))", 0);
                eq("((1)--(1))", 2);
            }

            #[test]
            fn multiplication() {
                eq("-1*-2", 2);
                eq("-1*--2", -2);
                eq("-1*-(4)", 4);
                eq("-1*-(-(4))", -4);
                eq("-1*2*-(3*4)", 24);
                eq("-1*-(-2*3*--4)", -24);
                eq("-1*--(-(-2*3)*-4)", 24);
            }

            #[test]
            fn division() {
                eq("-1/-2", 0);
                eq("-1/-4", 0);
                eq("-1/--4", -1);
                eq("-10/-10", 1);
                eq("-1/2/-3", 0);
                eq("-1/2/--3", -1);
                eq("10/-1", -10);
                eq("10/-2", -5);
                eq("10/-3", -4);
                eq("10/-4", -3);
                eq("10/2/-2", -3);
            }

            #[test]
            fn modulo() {
                eq("-1%2", 1);
                eq("1%-2", -1);
                eq("-1%-2", -1);
                eq("-1%4", 3);
                eq("1%-4", -3);
                eq("-10%10", 0);
                eq("10%-10", 0);
                eq("-10%-10", 0);
                eq("1%-2%3", 2);
                eq("-10%1", 0);
                eq("10%-1", 0);
                eq("-10%2", 0);
                eq("10%-2", 0);
                eq("-10%3", 2);
                eq("10%-3", -2);
                eq("-10%4", 2);
                eq("10%-4", -2);
            }

            #[test]
            fn exponentiation() {
                eq("-2^0", -1);
                eq("-2^1", -2);
                eq("-2^2", -4);
                eq("-2^3", -8);
                eq("-2^4", -16);
                eq("0^1", 0);
                eq("0^5", 0);
                eq("0^10", 0);
                eq("-3^3", -27);
                eq("-3^4", -81);

                // eq("2^-2^2", 16);
                // eq("-2^-2^2", 16);
                eq("-3^2^2", -81);
                eq("-2^2^3", -256);
                eq("-2^3^2", -512);
            }
        }

        #[test]
        fn precedence() {
            eq("2+-2/3", 1);
            eq("2+2*-2+2", 0);

            eq("3+3^3", 30);
            eq("3^3+3", 30);
            eq("3*2^2", 12);
            eq("3*-2^2", -12);
            eq("-3*-2^2", 12);
            eq("2^2*3", 12);
            eq("-2^2*3", -12);
            eq("-2^3^2*3", -1536);
        }
    }
}

#[cfg(test)]
mod rpn_tests {
    use super::calc;
    use ast::RPNEvaluator;

    fn parse<'input>(
        input: &'input str,
    ) -> Result<String, lalrpop_util::ParseError<usize, calc::Token<'input>, &'static str>> {
        let mut evaluator = RPNEvaluator::new();
        calc::ExprParser::new().parse(input)?.walk(&mut evaluator);
        Ok(evaluator.value())
    }

    fn eq(input: &str, expected: &str) {
        assert_eq!(parse(input).unwrap(), expected);
    }

    mod without_negation {
        use super::eq;

        #[test]
        fn numbers() {
            eq("0", "0");
            eq("3", "3");
            eq("10", "10");
            eq("12345", "12345");
            eq("987", "987");
        }

        #[test]
        fn parenthesized_numbers() {
            eq("(0)", "0");
            eq("(3)", "3");
            eq("(10)", "10");
            eq("(12345)", "12345");
            eq("(987)", "987");
            eq("((987))", "987");
            eq("(((987)))", "987");
            eq("((((987))))", "987");
        }

        #[test]
        fn addition() {
            eq("1+2", "1 2 +");
            eq("1+4", "1 4 +");
            eq("10+10", "10 10 +");
            eq("1+2+3", "1 2 + 3 +");
            eq("1+2+3+4", "1 2 + 3 + 4 +");
            eq("1+(2+3+4)", "1 2 3 + 4 + +");
            eq("1+((2+3)+4)", "1 2 3 + 4 + +");
            eq("1+(2+(3+4))", "1 2 3 4 + + +");
        }

        #[test]
        fn subtraction() {
            eq("2-1", "2 1 -");
            eq("1-2", "1 2 -");
            eq("4-1", "4 1 -");
            eq("1-4", "1 4 -");
            eq("10-10", "10 10 -");
            eq("3-2-1", "3 2 - 1 -");
            eq("1-2-3-4", "1 2 - 3 - 4 -");
            eq("1-(2-3-4)", "1 2 3 - 4 - -");
            eq("1-((2-3)-4)", "1 2 3 - 4 - -");
            eq("1-(2-(3-4))", "1 2 3 4 - - -");
        }

        #[test]
        fn multiplication() {
            eq("1*2", "1 2 *");
            eq("1*4", "1 4 *");
            eq("10*10", "10 10 *");
            eq("1*2*3", "1 2 * 3 *");
            eq("1*2*3*4", "1 2 * 3 * 4 *");
            eq("1*(2*3*4)", "1 2 3 * 4 * *");
            eq("1*((2*3)*4)", "1 2 3 * 4 * *");
            eq("1*(2*(3*4))", "1 2 3 4 * * *");
        }

        #[test]
        fn division() {
            eq("1/2", "1 2 /");
            eq("1/4", "1 4 /");
            eq("10/10", "10 10 /");
            eq("1/2/3", "1 2 / 3 /");
            eq("1/2/3/4", "1 2 / 3 / 4 /");
            eq("1/(2/3/4)", "1 2 3 / 4 / /");
            eq("1/((2/3)/4)", "1 2 3 / 4 / /");
            eq("1/(2/(3/4))", "1 2 3 4 / / /");
        }

        #[test]
        fn modulo() {
            eq("1%2", "1 2 %");
            eq("1%4", "1 4 %");
            eq("10%10", "10 10 %");
            eq("1%2%3", "1 2 % 3 %");
            eq("1%2%3%4", "1 2 % 3 % 4 %");
            eq("1%(2%3%4)", "1 2 3 % 4 % %");
            eq("1%((2%3)%4)", "1 2 3 % 4 % %");
            eq("1%(2%(3%4))", "1 2 3 4 % % %");
        }

        #[test]
        fn exponentiation() {
            eq("1^2", "1 2 ^");
            eq("1^4", "1 4 ^");
            eq("10^10", "10 10 ^");
            eq("1^2^3", "1 2 3 ^ ^");
            eq("1^2^3^4", "1 2 3 4 ^ ^ ^");
            eq("1^(2^3^4)", "1 2 3 4 ^ ^ ^");
            eq("1^((2^3)^4)", "1 2 3 ^ 4 ^ ^");
            eq("1^(2^(3^4))", "1 2 3 4 ^ ^ ^");
        }

        #[test]
        fn precedence() {
            eq("2+2*2", "2 2 2 * +");
            eq("2+2*2+2", "2 2 2 * + 2 +");
            eq("2*2+2", "2 2 * 2 +");
            eq("2+2*2/3+2", "2 2 2 * 3 / + 2 +");
            eq("2+2-2", "2 2 + 2 -");
            eq("2+2-2/2", "2 2 + 2 2 / -");
            eq("2*2-2/2", "2 2 * 2 2 / -");
            eq("2*(2-2)/2", "2 2 2 - * 2 /");
            eq("2*(2-2/2)", "2 2 2 2 / - *");
            eq("2*6/3*4", "2 6 * 3 / 4 *");
            eq("2*6/5%3*4", "2 6 * 5 / 3 % 4 *");

            eq("2^2^2", "2 2 2 ^ ^");
            eq("3^2^2", "3 2 2 ^ ^");
            eq("2^2^3", "2 2 3 ^ ^");
            eq("2^3^2", "2 3 2 ^ ^");
        }
    }

    mod with_negation {
        use super::eq;

        mod without_precedence {
            use super::eq;

            #[test]
            fn numbers() {
                eq("-0", "-0");
                eq("-3", "-3");
                eq("-10", "-10");
                eq("-12345", "-12345");
                eq("-987", "-987");
            }

            #[test]
            fn parenthesized_numbers() {
                eq("-(0)", "-0");
                eq("-(3)", "-3");
                eq("-(10)", "-10");
                eq("-(12345)", "-12345");
                eq("-(987)", "-987");
                eq("-((987))", "-987");
                eq("-(((987)))", "-987");
                eq("(-((987)))", "-987");
                eq("((-(987)))", "-987");

                eq("-(-0)", "-0 ~");
                eq("-(-3)", "-3 ~");
                eq("-(-10)", "-10 ~");
                eq("-(-12345)", "-12345 ~");
                eq("-(-987)", "-987 ~");
                eq("-((-987))", "-987 ~");
                eq("-(((-987)))", "-987 ~");
                eq("(-((-987)))", "-987 ~");
                eq("((-(-987)))", "-987 ~");
                eq("(-(-(-987)))", "-987 ~ ~");
                eq("-(-(-(-987)))", "-987 ~ ~ ~");
            }

            #[test]
            fn addition() {
                eq("-1+2", "-1 2 +");
                eq("-1+4", "-1 4 +");
                eq("-10+10", "-10 10 +");
                eq("-1+2+3", "-1 2 + 3 +");
                eq("-1+2+3+4", "-1 2 + 3 + 4 +");
                eq("1+(-2+3+4)", "1 -2 3 + 4 + +");
                eq("1+((-2+3)+4)", "1 -2 3 + 4 + +");
                eq("-1+(-2+(-3+4))", "-1 -2 -3 4 + + +");
            }

            #[test]
            fn subtraction() {
                eq("-2-1", "-2 1 -");
                eq("-1-2", "-1 2 -");
                eq("-4-1", "-4 1 -");
                eq("-1-4", "-1 4 -");
                eq("-10-10", "-10 10 -");
                eq("-3-2-1", "-3 2 - 1 -");
                eq("-1-2-3-4", "-1 2 - 3 - 4 -");
                eq("1-(-2-3-4)", "1 -2 3 - 4 - -");
                eq("1-((-2-3)-4)", "1 -2 3 - 4 - -");
                eq("1-(-2-(-3-4))", "1 -2 -3 4 - - -");
            }

            #[test]
            fn multiplication() {
                eq("-2*1", "-2 1 *");
                eq("-1*2", "-1 2 *");
                eq("-4*1", "-4 1 *");
                eq("-1*4", "-1 4 *");
                eq("-10*10", "-10 10 *");
                eq("-3*2*1", "-3 2 * 1 *");
                eq("-1*2*3*4", "-1 2 * 3 * 4 *");
                eq("1*(-2*3*4)", "1 -2 3 * 4 * *");
                eq("1*((-2*3)*4)", "1 -2 3 * 4 * *");
                eq("1*(-2*(-3*4))", "1 -2 -3 4 * * *");
            }

            #[test]
            fn division() {
                eq("-2/1", "-2 1 /");
                eq("-1/2", "-1 2 /");
                eq("-4/1", "-4 1 /");
                eq("-1/4", "-1 4 /");
                eq("-10/10", "-10 10 /");
                eq("-3/2/1", "-3 2 / 1 /");
                eq("-1/2/3/4", "-1 2 / 3 / 4 /");
                eq("1/(-2/3/4)", "1 -2 3 / 4 / /");
                eq("1/((-2/3)/4)", "1 -2 3 / 4 / /");
                eq("1/(-2/(-3/4))", "1 -2 -3 4 / / /");
            }
        }

        mod with_precedence {
            use super::eq;

            #[test]
            fn numbers() {
                eq("---0", "-0 ~ ~");
                eq("---3", "-3 ~ ~");
                eq("---10", "-10 ~ ~");
                eq("---12345", "-12345 ~ ~");
                eq("---987", "-987 ~ ~");
            }

            #[test]
            fn parenthesized_numbers() {
                eq("-(--0)", "-0 ~ ~");
                eq("-(--3)", "-3 ~ ~");
                eq("-(--10)", "-10 ~ ~");
                eq("-(--12345)", "-12345 ~ ~");
                eq("-(--987)", "-987 ~ ~");
                eq("-((--987))", "-987 ~ ~");
                eq("-(((--987)))", "-987 ~ ~");
                eq("(---((987)))", "-987 ~ ~");
                eq("((---(987)))", "-987 ~ ~");
            }

            #[test]
            fn addition() {
                eq("1+-2", "1 -2 +");
                eq("-1+-2", "-1 -2 +");
                eq("-1+-4", "-1 -4 +");
                eq("1+-4", "1 -4 +");
                eq("-10+-10", "-10 -10 +");
                eq("10+-10", "10 -10 +");
                eq("-1+-2+3", "-1 -2 + 3 +");
                eq("1+-2+3+-4", "1 -2 + 3 + -4 +");
                eq("1+-(-2+3+4)", "1 -2 3 + 4 + ~ +");
                eq("1+(-(-2+3)+4)", "1 -2 3 + ~ 4 + +");
                eq("1+(--(-2+3)+4)", "1 -2 3 + ~ ~ 4 + +");
                eq("1+-(-(-2+3)+4)", "1 -2 3 + ~ 4 + ~ +");
                eq("-1+-(-2+-(-3+4))", "-1 -2 -3 4 + ~ + ~ +");
            }

            #[test]
            fn subtraction() {
                eq("-2--1", "-2 -1 -");
                eq("-2---1", "-2 -1 ~ -");
                eq("-4--1", "-4 -1 -");
                eq("-4---1", "-4 -1 ~ -");
                eq("--4----1", "-4 ~ -1 ~ ~ -");
                eq("-1--4", "-1 -4 -");
                eq("-10-10", "-10 10 -");
                eq("-3--2-1", "-3 -2 - 1 -");
                eq("-3--2--1", "-3 -2 - -1 -");
                eq("1--(-2-3-4)", "1 -2 3 - 4 - ~ -");
                eq("1---(-2-3-4)", "1 -2 3 - 4 - ~ ~ -");
                eq("1--((-2-3)-4)", "1 -2 3 - 4 - ~ -");
                eq(
                    "1--(----2---(--3------4))",
                    "1 -2 ~ ~ ~ -3 ~ -4 ~ ~ ~ ~ - ~ ~ - ~ -",
                );
                eq("((1)-1)", "1 1 -");
                eq("((1)-(1))", "1 1 -");
                eq("((1)--(1))", "1 -1 -");
            }

            #[test]
            fn multiplication() {
                eq("-1*-2", "-1 -2 *");
                eq("-1*--2", "-1 -2 ~ *");
                eq("-1*-(4)", "-1 -4 *");
                eq("-1*-(-(4))", "-1 -4 ~ *");
                eq("-1*2*-(3*4)", "-1 2 * 3 4 * ~ *");
                eq("-1*-(-2*3*--4)", "-1 -2 3 * -4 ~ * ~ *");
                eq("-1*--(-(-2*3)*-4)", "-1 -2 3 * ~ -4 * ~ ~ *");
            }

            #[test]
            fn division() {
                eq("-1/-2", "-1 -2 /");
                eq("-1/--2", "-1 -2 ~ /");
                eq("-1/-(4)", "-1 -4 /");
                eq("-1/-(-(4))", "-1 -4 ~ /");
                eq("-1/2/-(3/4)", "-1 2 / 3 4 / ~ /");
                eq("-1/-(-2/3/--4)", "-1 -2 3 / -4 ~ / ~ /");
                eq("-1/--(-(-2/3)/-4)", "-1 -2 3 / ~ -4 / ~ ~ /");
            }

            #[test]
            fn modulo() {
                eq("-1%-2", "-1 -2 %");
                eq("-1%--2", "-1 -2 ~ %");
                eq("-1%-(4)", "-1 -4 %");
                eq("-1%-(-(4))", "-1 -4 ~ %");
                eq("-1%2%-(3%4)", "-1 2 % 3 4 % ~ %");
                eq("-1%-(-2%3%--4)", "-1 -2 3 % -4 ~ % ~ %");
                eq("-1%--(-(-2%3)%-4)", "-1 -2 3 % ~ -4 % ~ ~ %");
            }

            #[test]
            fn exponentiation() {
                eq("-2^0", "2 0 ^ ~");
                eq("-2^1", "2 1 ^ ~");
                eq("-2^2", "2 2 ^ ~");
                eq("-2^3", "2 3 ^ ~");
                eq("-2^4", "2 4 ^ ~");
                eq("0^1", "0 1 ^");
                eq("0^5", "0 5 ^");
                eq("0^10", "0 10 ^");
                eq("-3^3", "3 3 ^ ~");
                eq("-3^4", "3 4 ^ ~");

                // eq("2^-2^2", "2 -2 2 ^ ^");
                // eq("-2^-2^2", "-2 -2 2 ^ ^");
                // eq("-2^--2^2", "-2 -2 ~ 2 ^ ^");
                // eq("-3^2^2", "-3 2 2 ^ ^");
            }
        }

        #[test]
        fn precedence() {
            eq("2+-2/3", "2 -2 3 / +");
            eq("2+2*-2+2", "2 2 -2 * + 2 +");

            eq("1+2^3", "1 2 3 ^ +");
            eq("1^2+3", "1 2 ^ 3 +");
            eq("1*2^3", "1 2 3 ^ *");
            eq("1*-2^3", "1 2 3 ^ ~ *");
            eq("-1*-2^3", "-1 2 3 ^ ~ *");
            eq("-1^2^3*4", "1 2 3 ^ ^ ~ 4 *");
        }
    }
}
