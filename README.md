Mars Rover in Rust
==================

This project is done to test and store Rust good practices I came accross during my work. It does not use any external crate on purpose to focus on what the language offers in terms of:

 * code organization
 * code and documenation testing
 * behaviors implementation
 * types conversion
 * error management

## Description

Mars Rover simulate a robot which takes order on the command line:

```sh
$> mars_rover FLFFqRF
order: move forward
order: turn left
order: move forward
order: move forward
ERROR: invalid order character 'Q'
order: turn right
order: move forward
Robot final situation: Robot { position: (-2, 2), orientation: (0, 1) }
```