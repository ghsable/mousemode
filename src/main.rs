extern crate device_query;

use std::{thread, time};
use device_query::{DeviceQuery, DeviceState};
use mouse_rs::{types::keys::Keys, Mouse};

fn main() {
    let device_state = DeviceState::new();
    let mouse = Mouse::new();

    let mut prev_keys = vec![];

    loop {
        thread::sleep(time::Duration::from_millis(100));

        let keys = device_state.get_keys();
        let mouses = device_state.get_mouse();

        if keys.is_empty() || keys == prev_keys { continue; }

        if keys.len() == 2 {

            let keys_0 = keys[0].to_string();
            let keys_1 = keys[1].to_string();

            //Move Up
            if keys_0 == "Key2" && keys_1 == "P" { mouse.move_to(mouses.coords.0, mouses.coords.1 - 20).expect("Unable to move mouse"); }
            //Move Down
            if keys_0 == "Key3" && keys_1 == "P" { mouse.move_to(mouses.coords.0, mouses.coords.1 + 20).expect("Unable to move mouse"); }
            //Move Left
            if keys_0 == "Key1" && keys_1 == "P" { mouse.move_to(mouses.coords.0 - 20, mouses.coords.1).expect("Unable to move mouse"); }
            //Move Right
            if keys_0 == "Key4" && keys_1 == "P" { mouse.move_to(mouses.coords.0 + 20, mouses.coords.1).expect("Unable to move mouse"); }
            //Press Left Button
            if keys_0 == "Q" && keys_1 == "P" {
                mouse.press(&Keys::LEFT).expect("Unable to press button");
                mouse.release(&Keys::LEFT).expect("Unable to press button");
            }
            //Press Right Button
            if keys_0 == "W" && keys_1 == "P" {
                mouse.press(&Keys::RIGHT).expect("Unable to press button");
                mouse.release(&Keys::RIGHT).expect("Unable to press button");
            }
        }
        //-- Debug
        //println!("{:?}", keys);
        prev_keys = keys;
    }
}
