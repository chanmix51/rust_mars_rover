use std::{error::Error, fmt::Display};

/// Error type for order parsing. The following example prints an error message on the STDERR:
/// ```
/// use mars_rover::Order;
///
/// let input = 't';
///
/// if let Err(error) = Order::try_from(input) {
///     eprintln!("{}", error); // invalid order character 't'
/// }
/// ```
#[derive(Debug)]
pub struct InvalidOrder(
    /// the faulty character
    pub char,
);

impl Display for InvalidOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid order character '{}'", self.0)
    }
}

impl Error for InvalidOrder {}
