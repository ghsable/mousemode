//! Overview
//!
//! mousemode is a daemon that let us to use mouse operations on the Keyboard.
use std::{thread, time};
use device_query::{DeviceQuery, DeviceState};
use mouse_rs::{types::keys::Keys, Mouse};

/// Entry Point
///
/// This function converts keyboard events to mouse events.
fn main() {
    let device_state = DeviceState::new();
    let mouse = Mouse::new();

    loop {
        thread::sleep(time::Duration::from_millis(100));

        let keys_state = device_state.get_keys();
        //println!("{:?}", keys_state);

        if keys_state.len() != 2 { continue; }

        let keys_0 = keys_state[0].to_string();
        let keys_1 = keys_state[1].to_string();
        let mouse_state = device_state.get_mouse();

        // ←, ↑, ↓, →
        if keys_0 == "Key1" && keys_1 == "P" { mouse.move_to(mouse_state.coords.0 - 15, mouse_state.coords.1).expect("Unable to move mouse"); }
        if keys_0 == "Key2" && keys_1 == "P" { mouse.move_to(mouse_state.coords.0, mouse_state.coords.1 - 15).expect("Unable to move mouse"); }
        if keys_0 == "Key3" && keys_1 == "P" { mouse.move_to(mouse_state.coords.0, mouse_state.coords.1 + 15).expect("Unable to move mouse"); }
        if keys_0 == "Key4" && keys_1 == "P" { mouse.move_to(mouse_state.coords.0 + 15, mouse_state.coords.1).expect("Unable to move mouse"); }
        // right-click
        if keys_0 == "Q" && keys_1 == "P" {
            mouse.press(&Keys::RIGHT).expect("Unable to press button");
            mouse.release(&Keys::RIGHT).expect("Unable to press button");
        }
        // left-click
        if keys_0 == "W" && keys_1 == "P" {
            mouse.press(&Keys::LEFT).expect("Unable to press button");
            mouse.release(&Keys::LEFT).expect("Unable to press button");
        }
    }
}
