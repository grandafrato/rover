use control::wheels::{WheelCallback, WheelCommand, Wheels};
use std::sync::Mutex;

#[test]
fn callbacks_get_ran() {
    let data = Mutex::new(0);

    fn add_one_callback(data: &Mutex<u8>) -> WheelCallback {
        WheelCallback::new(|_, _| {
            let mut number = data.lock().unwrap();
            *number += 1;
        })
    }
    // Moving Forward

    let mut wheels = Wheels::new();
    wheels.set_move_forward_callback(add_one_callback(&data));

    assert_eq!(*data.lock().unwrap(), 0);

    wheels.apply_command(WheelCommand::move_forward());

    assert_eq!(*data.lock().unwrap(), 1);

    // Moving Backward

    wheels.set_move_backward_callback(add_one_callback(&data));
    wheels.apply_command(WheelCommand::move_backward());

    assert_eq!(*data.lock().unwrap(), 2);

    // Turning Right

    wheels.set_turn_right_callback(add_one_callback(&data));
    wheels.apply_command(WheelCommand::rotate_right());

    assert_eq!(*data.lock().unwrap(), 3);

    // Turning Left

    wheels.set_turn_left_callback(add_one_callback(&data));
    wheels.apply_command(WheelCommand::rotate_left());

    assert_eq!(*data.lock().unwrap(), 4);

    // Stopped

    wheels.set_stopped_callback(add_one_callback(&data));
    wheels.apply_command(WheelCommand::stop());

    assert_eq!(*data.lock().unwrap(), 5);
}
