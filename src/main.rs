use mars_rover::{OrderParserIterator, Robot};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        panic!("no orders given");
    }
    let orders = (&args[1]).trim().to_uppercase();
    let mut robot = Robot::default();

    for outcome in OrderParserIterator::new(&orders) {
        match outcome {
            Ok(order) => {
                println!("robot: {}", order);
                robot.please_do(order);
            }
            Err(error) => eprintln!("error: {}", error),
        };
    }

    println!("Robot final situation: {:?}", robot);
}
