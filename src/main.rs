mod lib;
use lib::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), Number(123));
    }

    #[test]
    fn parse_add_op() {
        assert_eq!(Op::new("+"), Op::Add);
    }

    #[test]
    fn parse_sub_op() {
        assert_eq!(Op::new("-"), Op::Sub);
    }

    #[test]
    fn parse_mul_op() {
        assert_eq!(Op::new("*"), Op::Mul);
    }

    #[test]
    fn parse_div_op() {
        assert_eq!(Op::new("/"), Op::Div);
    }
}
