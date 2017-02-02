mod rpn {

    use std::str::FromStr;

    enum Expression<N, O> where
        N: f64,
        O: FnMut(f64, f64) -> f64,
    {
        Number(N),
        Operator(O)
    } 

    fn parseToken(token: &str) -> Expression {
        match token {
            "+" => |x, y| x + y,
            _ => panic!("")
        }
    }

    pub fn compute(expression: &str) -> f64 {

        let tokens: Vec<&str> = expression.split(" ").collect();
        let mut numbers:Vec<Expression> = Vec::new();


        let left = f64::from_str(tokens[0]).unwrap();
        let right =
        f64::from_str(tokens[1]).unwrap();
        
        match tokens[2] {
            "+" => return left + right,
            "-" => return left - right,
            "*" => return left * right,
            "/" => return left / right,
            _ => panic!("cannot parse operator")
        }
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
