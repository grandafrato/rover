mod callbacks;
mod commands;
mod speed;

// Submodules to include in the public API.
pub use callbacks::WheelCallback;
pub use commands::WheelCommand;

use callbacks::*;
use speed::*;

pub struct Wheels<'a> {
    state: WheelState,
    counter_rotation_multiplier: Option<CounterRotationMultiplier>,
    speed: WheelSpeed,
    callbacks: WheelCallbacks<'a>,
}

impl<'a> Wheels<'a> {
    pub fn new() -> Self {
        Self {
            state: WheelState::Stopped,
            counter_rotation_multiplier: None,
            speed: WheelSpeed::new(50),
            callbacks: WheelCallbacks::default(),
        }
    }

    pub fn apply_command(&mut self, command: WheelCommand) {
        match command {
            WheelCommand::MoveForward => {
                self.state = WheelState::MovingForward;
                WheelCallbacks::call(&self.callbacks.moving_forward, self.speed.0, 0);
            }
            WheelCommand::MoveBackward => {
                self.state = WheelState::MovingBackward;
                WheelCallbacks::call(&self.callbacks.moving_backward, self.speed.0, 0);
            }
            WheelCommand::RotateRight => {
                self.state = WheelState::TurningToTheRight;
                WheelCallbacks::call(
                    &self.callbacks.turning_to_the_right,
                    self.speed.0,
                    counter_speed(&self.counter_rotation_multiplier, &self.speed),
                );
            }
            WheelCommand::RotateLeft => {
                self.state = WheelState::TurningToTheLeft;
                WheelCallbacks::call(
                    &self.callbacks.turning_to_the_left,
                    self.speed.0,
                    counter_speed(&self.counter_rotation_multiplier, &self.speed),
                );
            }
            WheelCommand::Stop => {
                self.state = WheelState::Stopped;
                WheelCallbacks::call(&self.callbacks.stopped, self.speed.0, 0);
            }
            WheelCommand::RotateRightWithCounterRotation(x) => {
                self.state = WheelState::TurningToTheRight;
                self.counter_rotation_multiplier = Some(x);
                WheelCallbacks::call(
                    &self.callbacks.turning_to_the_right,
                    self.speed.0,
                    counter_speed(&self.counter_rotation_multiplier, &self.speed),
                );
            }
            WheelCommand::RotateLeftWithCounterRotation(x) => {
                self.state = WheelState::TurningToTheLeft;
                self.counter_rotation_multiplier = Some(x);
                WheelCallbacks::call(
                    &self.callbacks.turning_to_the_left,
                    self.speed.0,
                    counter_speed(&self.counter_rotation_multiplier, &self.speed),
                );
            }
            WheelCommand::ChangeSpeed(x) => self.speed = x,
        }

        fn counter_speed(
            counter: &Option<CounterRotationMultiplier>,
            base_speed: &WheelSpeed,
        ) -> u8 {
            if let Some(counter_rotation) = counter {
                counter_rotation.get_wheel_speed(base_speed).0
            } else {
                base_speed.0
            }
        }
    }

    pub fn set_move_forward_callback(&mut self, callback: WheelCallback<'a>) {
        self.callbacks.moving_forward = Some(callback);
    }

    pub fn set_move_backward_callback(&mut self, callback: WheelCallback<'a>) {
        self.callbacks.moving_backward = Some(callback);
    }

    pub fn set_turn_right_callback(&mut self, callback: WheelCallback<'a>) {
        self.callbacks.turning_to_the_right = Some(callback);
    }
    pub fn set_turn_left_callback(&mut self, callback: WheelCallback<'a>) {
        self.callbacks.turning_to_the_left = Some(callback);
    }
    pub fn set_stopped_callback(&mut self, callback: WheelCallback<'a>) {
        self.callbacks.stopped = Some(callback);
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum WheelState {
    MovingForward,
    MovingBackward,
    TurningToTheRight,
    TurningToTheLeft,
    Stopped,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_wheel() {
        let wheels = Wheels::new();
        assert_eq!(wheels.state, WheelState::Stopped);
        assert_eq!(wheels.counter_rotation_multiplier, None);
        assert_eq!(wheels.speed, WheelSpeed::new(50));
    }

    #[test]
    fn wheel_processing_commands() {
        let mut wheels = Wheels::new();
        // MoveForward,
        wheels.apply_command(WheelCommand::move_forward());
        assert_eq!(wheels.state, WheelState::MovingForward);
        // MoveBackward,
        wheels.apply_command(WheelCommand::move_backward());
        assert_eq!(wheels.state, WheelState::MovingBackward);
        // RotateRight,
        wheels.apply_command(WheelCommand::rotate_right());
        assert_eq!(wheels.state, WheelState::TurningToTheRight);
        assert_eq!(wheels.counter_rotation_multiplier, None);
        // RotateLeft,
        wheels.apply_command(WheelCommand::rotate_left());
        assert_eq!(wheels.state, WheelState::TurningToTheLeft);
        assert_eq!(wheels.counter_rotation_multiplier, None);
        // Stop,
        wheels.apply_command(WheelCommand::stop());
        assert_eq!(wheels.state, WheelState::Stopped);
        // RotateRightWithCounterRotation(CounterRotationMultiplier),
        wheels.apply_command(WheelCommand::rotate_right_with_counter(2));
        assert_eq!(wheels.state, WheelState::TurningToTheRight);
        assert_eq!(
            wheels.counter_rotation_multiplier,
            Some(CounterRotationMultiplier::new(2))
        );
        // RotateLeftWithCounterRotation(CounterRotationMultiplier),
        wheels.apply_command(WheelCommand::rotate_left_with_counter(1));
        assert_eq!(wheels.state, WheelState::TurningToTheLeft);
        assert_eq!(
            wheels.counter_rotation_multiplier,
            Some(CounterRotationMultiplier::new(1))
        );
        // ChangeSpeed(WheelSpeed),
        wheels.apply_command(WheelCommand::change_speed(23));
        assert_eq!(wheels.speed, WheelSpeed::new(23));
    }

    #[test]
    fn no_arg_callbacks_are_given_the_wheel_speed_and_zero() {
        let wheel_speed = 1;

        let test_for_zero_callback =
            || WheelCallback::new(|a, b| assert_eq!((wheel_speed, 0), (a, b)));

        let mut wheels = Wheels::new();
        wheels.speed = WheelSpeed::new(wheel_speed);
        wheels.set_move_forward_callback(test_for_zero_callback());
        wheels.set_move_backward_callback(test_for_zero_callback());
        wheels.set_stopped_callback(test_for_zero_callback());

        // These call the callbacks.
        wheels.apply_command(WheelCommand::move_forward());
        wheels.apply_command(WheelCommand::move_backward());
        wheels.apply_command(WheelCommand::stop());
    }

    #[test]
    fn rotation_callbacks_are_given_speed_and_counter_rotation_speed() {
        let wheel_speed = 1;

        let test_for_speed_and_counter_speed =
            || WheelCallback::new(|a, b| assert_eq!((wheel_speed, wheel_speed), (a, b)));

        let test_for_speed_and_modified_counter_speed =
            || WheelCallback::new(|a, b| assert_eq!((wheel_speed, wheel_speed * 2), (a, b)));

        let mut wheels = Wheels::new();
        wheels.speed = WheelSpeed::new(wheel_speed);
        wheels.set_turn_right_callback(test_for_speed_and_counter_speed());
        wheels.set_turn_left_callback(test_for_speed_and_counter_speed());

        // These call the callbacks
        wheels.apply_command(WheelCommand::rotate_right());
        wheels.apply_command(WheelCommand::rotate_left());

        wheels.set_turn_right_callback(test_for_speed_and_modified_counter_speed());
        wheels.set_turn_left_callback(test_for_speed_and_modified_counter_speed());

        // These call the callbacks
        wheels.apply_command(WheelCommand::rotate_right_with_counter(2));
        wheels.apply_command(WheelCommand::rotate_left_with_counter(2));
    }
}
