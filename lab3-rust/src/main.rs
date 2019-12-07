use lalrpop_util::lalrpop_mod;
use std::io::BufRead;

lalrpop_mod!(pub calc);

fn main() {
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        println!("{:?}", calc::ExprParser::new().parse(&line.unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::calc;

    fn parse<'input>(
        input: &'input str,
    ) -> Result<i32, lalrpop_util::ParseError<usize, calc::Token<'input>, &'static str>> {
        calc::ExprParser::new().parse(input)
    }

    fn eq(input: &str, expected: i32) {
        assert_eq!(parse(input).unwrap(), expected);
    }

    #[test]
    fn numbers() {
        eq("0", 0);
        eq("3", 3);
        eq("10", 10);
        eq("12345", 12345);
        eq("987", 987);
    }

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
    fn precedence() {
        eq("2+2*2", 6);
        eq("2+2*2+2", 8);
        eq("2*2+2", 6);
        eq("2+2*2/3+2", 5);
        eq("2+2-2", 2);
        eq("2+2-2/2", 3);
        eq("2*2-2/2", 3);
        eq("2*(2-2)/2", 0);
        eq("2*2-2/2", 3);
        eq("2*6/3*4", 16);
    }
}
