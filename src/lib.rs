mod rpn {

    use std::str::FromStr;

    fn plus(numbers: &mut Vec<f64>) {
        let right = numbers.pop().unwrap();
        let left = numbers.pop().unwrap();
        numbers.push(left + right);
    }
    fn minus(numbers: &mut Vec<f64>) {
        let right = numbers.pop().unwrap();
        let left = numbers.pop().unwrap();
        numbers.push(left - right);
    }
    fn multiply(numbers: &mut Vec<f64>) {
        let right = numbers.pop().unwrap();
        let left = numbers.pop().unwrap();
        numbers.push(left * right);
    }
    fn divide(numbers: &mut Vec<f64>) {
        let right = numbers.pop().unwrap();
        let left = numbers.pop().unwrap();
        numbers.push(left / right);
    }

    pub fn compute(expression: &str) -> f64 {

        let tokens: Vec<&str> = expression.split(" ").collect();
        let mut numbers: Vec<f64> = Vec::new();

        for token in tokens {
            match token {
                "+" => plus(&mut numbers),
                "-" => minus(&mut numbers),
                "*" => multiply(&mut numbers),
                "/" => divide(&mut numbers),
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
    fn it_should_add_two_numbers() {
        assert_eq!(2.0, rpn::compute("1 1 +"));
        assert_eq!(4.0, rpn::compute("2 2 +"));
    }

    #[test]
    fn it_should_substract_two_numbers() {
        assert_eq!(-4.0, rpn::compute("1 5 -"));
        assert_eq!(1.5, rpn::compute("2 0.5 -"));
    }

    #[test]
    fn it_should_divide_and_multiply_two_numbers() {
        assert_eq!(5.0, rpn::compute("1 5 *"));
        assert_eq!(1.0, rpn::compute("2 2 /"));
    }

    #[test]
    fn it_should_no_divide_by_zero() {
        assert_eq!(f64::NEG_INFINITY, rpn::compute("-1 0 /"));
    }

    #[test]
    fn it_should_compute_expressions() {
        assert_eq!(7.0, rpn::compute("1 2 3 * +"));
    }

}
