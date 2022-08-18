use super::Order;

type Orientation = (i64, i64);
type Position = (i64, i64);

/// The robot structure is responsible of the deplacements of the robot.
/// ```
/// use mars_rover::{Robot, Order};
///
/// let mut robot = Robot::default();
/// robot
///     .please_do(Order::TurnLeft)
///     .please_do(Order::MoveForward);
///
/// assert_eq!(&(-1, 0), robot.get_position());
/// assert_eq!(&(-1, 0), robot.get_orientation());
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Robot {
    position: Position,
    orientation: Orientation,
}

impl Robot {
    /// Instanciate a new Robot with given values.
    pub fn new(position: Position, orientation: Orientation) -> Self {
        Self {
            position,
            orientation,
        }
    }

    /// Instanciate a new Robot with eventually given values.
    /// Default position is (0, 0) and default orienation is (0, 1).
    pub fn initialize(
        maybe_position: Option<Position>,
        maybe_orientation: Option<Orientation>,
    ) -> Self {
        let position = maybe_position.unwrap_or((0, 0));
        let orientation = maybe_orientation.unwrap_or((0, 1));

        Self::new(position, orientation)
    }

    /// Change the Robot position using orientation vector.
    fn move_forward(&mut self) -> &mut Self {
        self.position.0 += self.orientation.0;
        self.position.1 += self.orientation.1;

        self
    }

    /// Change the Robot orientation 90° clockwise.
    ///
    /// See [Wikipedia](https://en.wikipedia.org/wiki/Rotation_matrix) about how
    /// rotation matrix works.
    fn rotate_right(&mut self) -> &mut Self {
        self.orientation = (self.orientation.1, -self.orientation.0);

        self
    }

    /// change the Robot orientation 90° anticlockwise
    ///
    /// See [Wikipedia](https://en.wikipedia.org/wiki/Rotation_matrix) about how
    /// rotation matrix works.
    fn rotate_left(&mut self) -> &mut Self {
        self.orientation = (-self.orientation.1, self.orientation.0);

        self
    }

    /// Gently ask the Robot to execute the given order.
    ///
    /// sudo make me a sandwich (the [reference](https://xkcd.com/149))
    pub fn please_do(&mut self, order: Order) -> &mut Self {
        match order {
            Order::MoveForward => self.move_forward(),
            Order::TurnLeft => self.rotate_left(),
            Order::TurnRight => self.rotate_right(),
        }
    }

    /// position getter
    pub fn get_position(&self) -> &Position {
        &self.position
    }

    /// orientation getter
    pub fn get_orientation(&self) -> &Orientation {
        &self.orientation
    }
}

impl Default for Robot {
    fn default() -> Self {
        Self::initialize(None, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let robot = Robot::default();
        let robot_ref = Robot::initialize(None, None);

        assert_eq!(robot_ref, robot);
    }

    #[test]
    fn test_initialize_no_info() {
        let robot = Robot::initialize(None, None);

        assert_eq!((0, 1), robot.orientation);
        assert_eq!((0, 0), robot.position);
    }

    #[test]
    fn test_move_forward() {
        let mut robot = Robot::default();
        robot.move_forward();

        assert_eq!((0, 1), robot.orientation);
        assert_eq!((0, 1), robot.position);
    }

    #[test]
    fn test_rotate_right() {
        let mut robot = Robot::default();
        robot.rotate_right();

        assert_eq!(Robot::initialize(Some((0, 0)), Some((1, 0))), robot);
    }

    #[test]
    fn test_rotate_left() {
        let mut robot = Robot::default();
        robot.rotate_left();

        assert_eq!(Robot::initialize(Some((0, 0)), Some((-1, 0))), robot);
    }

    #[test]
    fn test_please_do() {
        let mut robot = Robot::default();
        let mut robot_ref = Robot::default();

        assert_eq!(
            robot_ref.move_forward(),
            robot.please_do(Order::MoveForward)
        );
        assert_eq!(robot_ref.rotate_left(), robot.please_do(Order::TurnLeft));
        assert_eq!(robot_ref.rotate_right(), robot.please_do(Order::TurnRight));
    }
}
