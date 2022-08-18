use std::{fmt::Display, str::Chars};

use super::InvalidOrder;

#[derive(Debug, Copy, PartialEq, Eq, Clone)]
pub enum Order {
    MoveForward,
    TurnLeft,
    TurnRight,
}

impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Order::MoveForward => write!(f, "move forward"),
            Order::TurnLeft => write!(f, "turn left"),
            Order::TurnRight => write!(f, "turn right"),
        }
    }
}

/// converting chars into orders
/// this allows the following usage:
/// ```
/// use mars_rover::Order;
///
/// let order = Order::try_from('F')
///     .expect("'F' is a valid order hence that should not fail");
/// assert_eq!(Order::MoveForward, order);
/// ```
impl TryFrom<char> for Order {
    type Error = InvalidOrder;

    fn try_from(letter: char) -> Result<Self, Self::Error> {
        match letter {
            'F' => Ok(Order::MoveForward),
            'R' => Ok(Order::TurnRight),
            'L' => Ok(Order::TurnLeft),
            l => Err(InvalidOrder(l)),
        }
    }
}

/// an iterator on &str that pops Order instances out of it
/// ```
/// use mars_rover::OrderParserIterator;
///
/// let orders = "FLFFqRL";
///
/// for outcome in OrderParserIterator::new(orders) {
///     println!("{:?}", outcome);
/// }
/// ```
pub struct OrderParserIterator<'a> {
    iter: Chars<'a>,
}

impl<'a> OrderParserIterator<'a> {
    pub fn new(orders: &'a str) -> Self {
        Self {
            iter: orders.chars(),
        }
    }
}
impl<'a> Iterator for OrderParserIterator<'a> {
    type Item = Result<Order, InvalidOrder>;

    fn next(self: &mut OrderParserIterator<'a>) -> Option<Self::Item> {
        self.iter.next().map(Order::try_from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_iterator() {
        let outcomes: Vec<Order> = OrderParserIterator::new("LFR")
            .map(|o| o.unwrap())
            .collect();
        let outcomes = outcomes.as_slice();

        assert_eq!(3, outcomes.len());
        assert_eq!(Order::TurnLeft, outcomes[0]);
        assert_eq!(Order::MoveForward, outcomes[1]);
        assert_eq!(Order::TurnRight, outcomes[2]);
    }

    #[test]
    fn test_string_with_errors() {
        let outcomes: Vec<Result<Order, InvalidOrder>> = OrderParserIterator::new("Fg").collect();
        let outcomes = outcomes.as_slice();

        assert_eq!(2, outcomes.len());
        assert!(outcomes[1].is_err());
    }
}
