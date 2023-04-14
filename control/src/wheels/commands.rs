use super::speed::*;

#[derive(Debug, PartialEq)]
pub enum WheelCommand {
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
        Self::MoveForward
    }
    pub fn move_backward() -> Self {
        Self::MoveBackward
    }
    pub fn rotate_right() -> Self {
        Self::RotateRight
    }
    pub fn rotate_left() -> Self {
        Self::RotateLeft
    }
    pub fn stop() -> Self {
        Self::Stop
    }

    pub fn change_speed(speed: u8) -> Self {
        Self::ChangeSpeed(WheelSpeed::new(speed))
    }
    pub fn rotate_left_with_counter(counter_rotation_multiplier: u8) -> Self {
        Self::RotateLeftWithCounterRotation(CounterRotationMultiplier::new(
            counter_rotation_multiplier,
        ))
    }
    pub fn rotate_right_with_counter(counter_rotation_multiplier: u8) -> Self {
        Self::RotateRightWithCounterRotation(CounterRotationMultiplier::new(
            counter_rotation_multiplier,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructing_no_arg_wheel_commands() {
        assert_eq!(WheelCommand::move_forward(), WheelCommand::MoveForward);
        assert_eq!(WheelCommand::move_backward(), WheelCommand::MoveBackward);
        assert_eq!(WheelCommand::rotate_right(), WheelCommand::RotateRight);
        assert_eq!(WheelCommand::rotate_left(), WheelCommand::RotateLeft);
        assert_eq!(WheelCommand::stop(), WheelCommand::Stop);
    }

    #[test]
    fn constructing_wheel_commands_with_args() {
        let counter_rotation_multiplier = 2;

        assert_eq!(
            WheelCommand::rotate_right_with_counter(counter_rotation_multiplier),
            WheelCommand::RotateRightWithCounterRotation(CounterRotationMultiplier::new(
                counter_rotation_multiplier
            ))
        );
        assert_eq!(
            WheelCommand::rotate_left_with_counter(counter_rotation_multiplier),
            WheelCommand::RotateLeftWithCounterRotation(CounterRotationMultiplier::new(
                counter_rotation_multiplier
            ))
        );

        let speed = 50;

        assert_eq!(
            WheelCommand::change_speed(speed),
            WheelCommand::ChangeSpeed(WheelSpeed::new(speed))
        );
    }
}
