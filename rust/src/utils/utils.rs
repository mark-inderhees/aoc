use rusttype::Point;
use std::str::FromStr;

/// A simple way to import and use Point type.
pub type UtilsPoint = Point<i32>;

/// Tests if this character is in the string.
pub fn char_in_string(c: &char, s: &String) -> bool {
    if s.chars().any(|x| x == *c) {
        return true;
    }
    false
}

/// Gets all values in a string.
pub fn find_vals<T>(s: &str) -> Vec<T>
where
    T: FromStr,
{
    let mut values: Vec<T> = vec![];
    let mut line: String = s.clone().to_string();

    // Remove common text seperators
    let remove = [';', ',', '=', ':', '.'];
    for r in remove {
        line = line.replace(r, " ");
    }

    // Start adding values to the list
    for item in line.split(" ") {
        let value: Result<T, _> = item.parse();
        if let Ok(v) = value {
            values.push(v);
        }
    }

    values
}

/// Get a single value, the first value found in the line.
pub fn find_val<T>(s: &str) -> T
where
    T: FromStr,
    T: Copy,
{
    let values = find_vals(s);
    values[0]
}

/// Get the taxi cab distance between two points.
pub fn manhattan_distance(p1: UtilsPoint, p2: UtilsPoint) -> i32 {
    let x = (p1.x - p2.x).abs();
    let y = (p1.y - p2.y).abs();
    x + y
}

/// An enum of math operators.
#[derive(Default, Debug, Clone, Copy)]
pub enum Operator {
    #[default]
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// Do some simple math, returns the value of result, for example:
/// result = value_left ? value_right
pub fn do_math(operator: Operator, value_left: i128, value_right: i128) -> i128 {
    match operator {
        Operator::Add => value_left + value_right,
        Operator::Subtract => value_left - value_right,
        Operator::Multiply => value_left * value_right,
        Operator::Divide => value_left / value_right,
    }
}

/// Solve a math equation, returns the value of x, for example:
/// result = x ? value
/// result = value ? x
pub fn solve_math(operator: Operator, result: i128, value: i128, solve_for_left: bool) -> i128 {
    match operator {
        Operator::Add => result - value,
        Operator::Subtract => {
            if solve_for_left {
                return result + value;
            } else {
                return value - result;
            }
        }
        Operator::Multiply => result / value,
        Operator::Divide => {
            if solve_for_left {
                return result * value;
            } else {
                return value / result;
            }
        }
    }
}
