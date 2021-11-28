extern crate device_query;

use device_query::{DeviceQuery, DeviceState};
use mouse_rs::{types::keys::Keys, Mouse};

fn main() {
    let device_state = DeviceState::new();
    let mouse = Mouse::new();

    let mut prev_keys = vec![];

    loop {
        let keys = device_state.get_keys();
        let mouses = device_state.get_mouse();

        if keys.is_empty() { continue; }

        if keys != prev_keys && keys.len() == 2 {

            let keys_0 = keys[0].to_string();
            let keys_1 = keys[1].to_string();

            if keys_0 == "LShift" || keys_0 == "RShift" {
                if keys_1 == "Up"    { mouse.move_to(mouses.coords.0, mouses.coords.1 - 20).expect("Unable to move mouse"); }
                if keys_1 == "Down"  { mouse.move_to(mouses.coords.0, mouses.coords.1 + 20).expect("Unable to move mouse"); }
                if keys_1 == "Left"  { mouse.move_to(mouses.coords.0 - 20, mouses.coords.1).expect("Unable to move mouse"); }
                if keys_1 == "Right" { mouse.move_to(mouses.coords.0 + 20, mouses.coords.1).expect("Unable to move mouse"); }
                if keys_1 == "RShift" {
                    mouse.press(&Keys::RIGHT).expect("Unable to press button");
                    mouse.release(&Keys::RIGHT).expect("Unable to press button");
                }
            }

            if keys_0 == "A" {
                if keys_1 == "S" {
                    mouse.press(&Keys::LEFT).expect("Unable to press button");
                    mouse.release(&Keys::LEFT).expect("Unable to press button");
                }
            }

            //-- Debug
            //println!("{:?}", keys)
        }
        prev_keys = keys;
    }
}
