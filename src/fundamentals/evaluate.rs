//! # Dijkstra's two-stack algorithm for expression evaluation

use super::stack::Stack;

/// Evaluate expression
///
/// Assume the input expression is splitted by ' '
pub fn evaluate(exp: &str) -> f64 {
    let mut ops = Stack::default();
    let mut vals: Stack<f64> = Stack::default();

    for token in exp.split(' ') {
        match token {
            "(" => {}
            "+" | "-" | "*" | "/" | "sqrt" => ops.push(token),
            ")" => {
                let op = ops.pop();
                let mut v = vals.pop();
                if op == "+" {
                    v += vals.pop();
                } else if op == "-" {
                    v -= vals.pop();
                } else if op == "*" {
                    v *= vals.pop();
                } else if op == "/" {
                    v = vals.pop() / v;
                } else if op == "sqrt" {
                    v = v.sqrt();
                }
                vals.push(v);
            }
            _ => vals.push(token.parse::<f64>().unwrap()),
        }
    }
    vals.pop()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expr1() {
        let s = "( 1 + ( ( 2 + 3 ) * ( 4 * 5 ) ) )";
        println!("{}", evaluate(s));
        assert!((evaluate(s) - 101.0).abs() < f64::EPSILON);
    }

    #[test]
    fn expr2() {
        let s = "( ( 1 + sqrt ( 5.0 ) ) / 2.0 )";
        assert!((evaluate(s) - 1.618033988749895).abs() < f64::EPSILON);
    }
}
