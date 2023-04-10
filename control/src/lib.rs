#![no_std]
#![forbid(unsafe_code)]

use tiny_fn::tiny_fn;

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

    pub fn process_command(&mut self, WheelCommand(command): WheelCommand) {
        match command {
            Commands::MoveForward => {
                self.state = WheelState::MovingForward;
                WheelCallbacks::call(&self.callbacks.moving_forward, 0, 0);
            }
            Commands::MoveBackward => {
                self.state = WheelState::MovingBackward;
                WheelCallbacks::call(&self.callbacks.moving_backward, 0, 0);
            }
            Commands::RotateRight => {
                self.state = WheelState::TurningToTheRight;
                WheelCallbacks::call(&self.callbacks.turning_to_the_right, 0, 0);
            }
            Commands::RotateLeft => {
                self.state = WheelState::TurningToTheLeft;
                WheelCallbacks::call(&self.callbacks.turning_to_the_left, 0, 0);
            }
            Commands::Stop => {
                self.state = WheelState::Stopped;
                WheelCallbacks::call(&self.callbacks.stopped, 0, 0);
            }
            Commands::RotateRightWithCounterRotation(x) => {
                self.state = WheelState::TurningToTheRight;
                self.counter_rotation_multiplier = Some(x);
                WheelCallbacks::call(&self.callbacks.turning_to_the_right, 0, 0);
            }
            Commands::RotateLeftWithCounterRotation(x) => {
                self.state = WheelState::TurningToTheLeft;
                self.counter_rotation_multiplier = Some(x);
                WheelCallbacks::call(&self.callbacks.turning_to_the_left, 0, 0);
            }
            Commands::ChangeSpeed(x) => self.speed = x,
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

tiny_fn! {
    pub struct WheelCallback = Fn(a: u32, b: u32);
}

#[derive(Default)]
struct WheelCallbacks<'a> {
    moving_forward: Option<WheelCallback<'a>>,
    moving_backward: Option<WheelCallback<'a>>,
    turning_to_the_right: Option<WheelCallback<'a>>,
    turning_to_the_left: Option<WheelCallback<'a>>,
    stopped: Option<WheelCallback<'a>>,
}

impl<'a> WheelCallbacks<'a> {
    fn call(callback: &Option<WheelCallback>, a: u32, b: u32) {
        match callback {
            Some(callback) => callback.call(a, b),
            None => (),
        }
    }
}

#[derive(Debug, PartialEq)]
struct WheelSpeed(u8);

impl WheelSpeed {
    const MAX_SPEED: u8 = 100;
    fn new(speed: u8) -> Self {
        if speed > Self::MAX_SPEED || speed < 1 {
            panic!("Speed must be between 1 and 100.")
        } else {
            WheelSpeed(speed)
        }
    }

    fn multiply(&self, other_wheel: &Self) -> Self {
        Self(match self.0.checked_mul(other_wheel.0) {
            Some(speed) => speed.clamp(1, Self::MAX_SPEED),
            None => Self::MAX_SPEED,
        })
    }
}

#[derive(Debug, PartialEq)]
struct CounterRotationMultiplier(WheelSpeed);

impl CounterRotationMultiplier {
    fn new(counter_rotation_multiplier: u8) -> Self {
        Self(WheelSpeed::new(counter_rotation_multiplier))
    }

    fn get_wheel_speed(&self, base_speed: &WheelSpeed) -> WheelSpeed {
        self.0.multiply(base_speed)
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
    fn wheel_speed_can_be_between_1_and_100() {
        assert_eq!(WheelSpeed::new(50).0, 50);
    }

    #[test]
    #[should_panic]
    fn wheel_speed_cannot_be_over_max_speed() {
        WheelSpeed::new(WheelSpeed::MAX_SPEED + 1);
    }

    #[test]
    #[should_panic]
    fn wheel_speed_cannot_be_under_1() {
        WheelSpeed::new(0);
    }

    #[test]
    fn get_counter_rotation_speed() {
        assert_eq!(
            CounterRotationMultiplier::new(4).get_wheel_speed(&WheelSpeed::new(2)),
            WheelSpeed::new(8)
        );
    }

    #[test]
    fn get_counter_rotation_speed_will_not_excede_max_speed() {
        assert_eq!(
            CounterRotationMultiplier::new(4)
                .get_wheel_speed(&WheelSpeed::new(WheelSpeed::MAX_SPEED)),
            WheelSpeed::new(WheelSpeed::MAX_SPEED)
        );
    }

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
        wheels.process_command(WheelCommand::move_forward());
        assert_eq!(wheels.state, WheelState::MovingForward);
        // MoveBackward,
        wheels.process_command(WheelCommand::move_backward());
        assert_eq!(wheels.state, WheelState::MovingBackward);
        // RotateRight,
        wheels.process_command(WheelCommand::rotate_right());
        assert_eq!(wheels.state, WheelState::TurningToTheRight);
        assert_eq!(wheels.counter_rotation_multiplier, None);
        // RotateLeft,
        wheels.process_command(WheelCommand::rotate_left());
        assert_eq!(wheels.state, WheelState::TurningToTheLeft);
        assert_eq!(wheels.counter_rotation_multiplier, None);
        // Stop,
        wheels.process_command(WheelCommand::stop());
        assert_eq!(wheels.state, WheelState::Stopped);
        // RotateRightWithCounterRotation(CounterRotationMultiplier),
        wheels.process_command(WheelCommand::rotate_right_with_counter(2));
        assert_eq!(wheels.state, WheelState::TurningToTheRight);
        assert_eq!(
            wheels.counter_rotation_multiplier,
            Some(CounterRotationMultiplier::new(2))
        );
        // RotateLeftWithCounterRotation(CounterRotationMultiplier),
        wheels.process_command(WheelCommand::rotate_left_with_counter(1));
        assert_eq!(wheels.state, WheelState::TurningToTheLeft);
        assert_eq!(
            wheels.counter_rotation_multiplier,
            Some(CounterRotationMultiplier::new(1))
        );
        // ChangeSpeed(WheelSpeed),
        wheels.process_command(WheelCommand::change_speed(23));
        assert_eq!(wheels.speed, WheelSpeed::new(23));
    }
}
