# mousemode
[mousemode](https://ghsable.github.io/mousemode/): mouse operations == keyboard shortcuts; `//` [🦀](https://www.rust-lang.org/)
| mouse operation | keyboard shortcut |
| :--- | :--- |
| ↑ | `P` + `2` |
| ↓ | `P` + `3` |
| ← | `P` + `1` |
| → | `P` + `4` |
| left-click  | `P` + `W` |
| right-click | `P` + `Q` |

## Installation
1. Install **Dependencies**.

    | Linux | Windows | macOS |
    | :--- | :--- | :--- |
    | [libX11](https://gitlab.freedesktop.org/xorg/lib/libx11) [📝](https://github.com/ostrosco/device_query#dependencies)<br>[libxdo-dev](https://github.com/jordansissel/xdotool) [📝](https://github.com/AltF02/mouse-rs#linux-disclaimer) | `-` | `-` |

2. Compile the current package.

    cargo build --release

3. Run the [mousemode](https://ghsable.github.io/mousemode/).

    ./target/release/mousemode

## ...!!XD
* [mousemode](https://ghsable.github.io/mousemode/) + [vimode](https://ghsable.github.io/vimode/) = `...!!XD`

## Note
### The [Rust](https://github.com/rust-lang/rust) Standard Library
| use | [Docs.rs](https://docs.rs/) |
| :---  | :--- |
| std::thread | [std](https://doc.rust-lang.org/std/)::[thread](https://doc.rust-lang.org/std/thread/) |
| std::time | [std](https://doc.rust-lang.org/std/)::[time](https://doc.rust-lang.org/std/time/) |

### [Cargo.toml](https://github.com/ghsable/mousemode/blob/main/Cargo.toml)
| use | [crates.io](https://crates.io/) | [Docs.rs](https://docs.rs/) | [GitHub](https://github.com/) |
| :---  | :--- | :--- | :--- |
| device\_query | [device\_query](https://crates.io/crates/device_query) | [device\_query](https://docs.rs/device_query/latest/device_query/) | [ostrosco/device\_query](https://github.com/ostrosco/device_query) |
| mouse-rs | [Mouse-rs](https://crates.io/crates/mouse-rs) | [Mouse-rs](https://docs.rs/mouse-rs/latest/mouse_rs/) | [AltF02/mouse-rs](https://github.com/AltF02/mouse-rs) |

