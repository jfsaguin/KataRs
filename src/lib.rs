mod rpn {

    use std::str::FromStr;

    fn bi_operands(numbers: &mut Vec<f64>, calc: &Fn(f64, f64) -> f64) {
        let right = numbers.pop().unwrap();
        let left = numbers.pop().unwrap();
        numbers.push(calc(left, right));
    }

    fn mono_operands(numbers: &mut Vec<f64>, calc: &Fn(f64) -> f64) {
        let operand = numbers.pop().unwrap();
        numbers.push(calc(operand));
    }
    pub fn compute(expression: &str) -> f64 {

        let tokens: Vec<&str> = expression.split(" ").collect();
        let mut numbers: Vec<f64> = Vec::new();

        for token in tokens {
            match token {
                "+" => bi_operands(&mut numbers, &|x, y| x + y),
                "-" => bi_operands(&mut numbers, &|x, y| x - y),
                "*" => bi_operands(&mut numbers, &|x, y| x * y),
                "/" => bi_operands(&mut numbers, &|x, y| x / y),
                "sqrt" => mono_operands(&mut numbers, &|x| x.sqrt()),
                _ => numbers.push(f64::from_str(token).unwrap()),
            }

        }
        return numbers.pop().unwrap();
    }
}


#[cfg(test)]
mod tests {

    use rpn;
    use std::f64;

    #[test]
    fn it_should_add_compute_simple_operation() {
        assert_eq!(2.0, rpn::compute("1 1 +"));
        assert_eq!(1.5, rpn::compute("2 0.5 -"));
        assert_eq!(5.0, rpn::compute("1 5 *"));
        assert_eq!(1.0, rpn::compute("2 2 /"));
        assert_eq!(3.0, rpn::compute("9 sqrt"));

    }

    #[test]
    fn it_should_no_divide_by_zero() {
        assert_eq!(f64::NEG_INFINITY, rpn::compute("-1 0 /"));
    }

    #[test]
    fn it_should_compute_multiple_expressions() {
        assert_eq!(7.0, rpn::compute("1 2 3 * +"));
        assert_eq!(2.0, rpn::compute("2 4 + 3 /"));
        assert_eq!(3.0, rpn::compute("2 4 3 + + sqrt"));

    }

}
