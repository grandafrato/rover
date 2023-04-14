#[derive(Debug, PartialEq)]
pub struct WheelSpeed(pub u8);

impl WheelSpeed {
    const MAX_SPEED: u8 = 100;
    pub fn new(speed: u8) -> Self {
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
pub struct CounterRotationMultiplier(WheelSpeed);

impl CounterRotationMultiplier {
    pub fn new(counter_rotation_multiplier: u8) -> Self {
        Self(WheelSpeed::new(counter_rotation_multiplier))
    }

    pub fn get_wheel_speed(&self, base_speed: &WheelSpeed) -> WheelSpeed {
        self.0.multiply(base_speed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
