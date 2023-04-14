use alloc::boxed::Box;

pub struct WheelCallback<'a>(Box<dyn Fn(u8, u8) + 'a>);

impl<'a> WheelCallback<'a> {
    pub fn new<F: Fn(u8, u8) + 'a>(callback: F) -> Self {
        Self(Box::new(callback))
    }

    fn call(&self, a: u8, b: u8) {
        self.0(a, b);
    }
}

#[derive(Default)]
pub struct WheelCallbacks<'a> {
    pub moving_forward: Option<WheelCallback<'a>>,
    pub moving_backward: Option<WheelCallback<'a>>,
    pub turning_to_the_right: Option<WheelCallback<'a>>,
    pub turning_to_the_left: Option<WheelCallback<'a>>,
    pub stopped: Option<WheelCallback<'a>>,
}

impl<'a> WheelCallbacks<'a> {
    pub fn call(callback: &Option<WheelCallback>, a: u8, b: u8) {
        match callback {
            Some(callback) => callback.call(a, b),
            None => (),
        }
    }
}
