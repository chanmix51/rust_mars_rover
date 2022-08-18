use mars_rover::{OrderParserIterator, Robot};

#[test]
fn test_iterator() {
    let orders = "F FRFqLF";
    let mut robot = Robot::default();

    for order in OrderParserIterator::new(&orders).filter_map(|v| v.ok()) {
        robot.please_do(order);
    }

    assert_eq!(&(1, 3), robot.get_position());
    assert_eq!(&(0, 1), robot.get_orientation());
}
