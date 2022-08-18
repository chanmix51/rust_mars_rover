use mars_rover::{Order, OrderParserIterator, Robot};

#[test]
fn test_iterator() {
    let orders = "F FRFqLF";
    let mut robot = Robot::default();

    for (index, order) in OrderParserIterator::new(&orders)
        .filter_map(|v| v.ok())
        .enumerate()
    {
        match index {
            0 | 1 | 3 | 5 => assert_eq!(Order::MoveForward, order),
            2 => assert_eq!(Order::TurnRight, order),
            4 => assert_eq!(Order::TurnLeft, order),
            _ => panic!("unexpected order '{}' at position {}", order, index),
        }
        robot.please_do(order);
    }

    assert_eq!(&(1, 3), robot.get_position());
    assert_eq!(&(0, 1), robot.get_orientation());
}
