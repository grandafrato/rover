#![no_std]
#![forbid(unsafe_code)]

pub struct Wheels {
    state: WheelState,
    speed: WheelSpeed,
}

enum WheelState {
    MovingForward,
    MovingBackward,
    TurningToTheRight(Option<CounterRotationMultiplier>),
    TurningToTheLeft(Option<CounterRotationMultiplier>),
    Stopped,
}

#[derive(Debug, PartialEq)]
struct WheelSpeed(u8);

impl WheelSpeed {
    fn new(speed: u8) -> Self {
        WheelSpeed(speed)
    }
}

#[derive(Debug, PartialEq)]
struct CounterRotationMultiplier(WheelSpeed);

impl CounterRotationMultiplier {
    fn new(counter_rotation_multiplier: u8) -> Self {
        Self(WheelSpeed::new(counter_rotation_multiplier))
    }
}

pub struct WheelCommand(Commands);

#[derive(Debug, PartialEq)]
enum Commands {
    MoveForward,
    MoveBackward,
    RotateRight,
    RotateLeft,
    Stop,
    RotateRightWithCounterRotation(CounterRotationMultiplier),
    RotateLeftWithCounterRotation(CounterRotationMultiplier),
    ChangeSpeed(WheelSpeed),
}

impl WheelCommand {
    pub fn move_forward() -> Self {
        Self(Commands::MoveForward)
    }
    pub fn move_backward() -> Self {
        Self(Commands::MoveBackward)
    }
    pub fn rotate_right() -> Self {
        Self(Commands::RotateRight)
    }
    pub fn rotate_left() -> Self {
        Self(Commands::RotateLeft)
    }
    pub fn stop() -> Self {
        Self(Commands::Stop)
    }

    pub fn change_speed(speed: u8) -> Self {
        Self(Commands::ChangeSpeed(WheelSpeed::new(speed)))
    }
    pub fn rotate_left_with_counter(counter_rotation_multiplier: u8) -> Self {
        Self(Commands::RotateLeftWithCounterRotation(
            CounterRotationMultiplier::new(counter_rotation_multiplier),
        ))
    }
    pub fn rotate_right_with_counter(counter_rotation_multiplier: u8) -> Self {
        Self(Commands::RotateRightWithCounterRotation(
            CounterRotationMultiplier::new(counter_rotation_multiplier),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructing_no_arg_wheel_commands() {
        assert_eq!(WheelCommand::move_forward().0, Commands::MoveForward);
        assert_eq!(WheelCommand::move_backward().0, Commands::MoveBackward);
        assert_eq!(WheelCommand::rotate_right().0, Commands::RotateRight);
        assert_eq!(WheelCommand::rotate_left().0, Commands::RotateLeft);
        assert_eq!(WheelCommand::stop().0, Commands::Stop);
    }

    #[test]
    fn constructing_wheel_commands_with_args() {
        let counter_rotation_multiplier = 2;

        assert_eq!(
            WheelCommand::rotate_right_with_counter(counter_rotation_multiplier).0,
            Commands::RotateRightWithCounterRotation(CounterRotationMultiplier::new(
                counter_rotation_multiplier
            ))
        );
        assert_eq!(
            WheelCommand::rotate_left_with_counter(counter_rotation_multiplier).0,
            Commands::RotateLeftWithCounterRotation(CounterRotationMultiplier::new(
                counter_rotation_multiplier
            ))
        );

        let speed = 50;

        assert_eq!(
            WheelCommand::change_speed(speed).0,
            Commands::ChangeSpeed(WheelSpeed::new(speed))
        );
    }

    #[test]
    fn wheel_speed_restrictions() {
        todo!();
    }

    #[test]
    fn counter_rotation_multiplier_restrictions() {
        todo!();
    }

    #[test]
    fn wheel_interface() {
        todo!();
    }
}
