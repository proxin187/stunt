
<div align="center">
<br>
<a href="https://github.com/proxin187/stunt">
    <img src="https://raw.githubusercontent.com/proxin187/stunt/refs/heads/main/assets/logo.png?" width="200">
</a>

<br>
<br>

[![crates.io](https://img.shields.io/badge/crates.io-stunt-red?style=for-the-badge&logo=none)](https://crates.io/crates/stunt)
[![docs.rs](https://img.shields.io/badge/docs.rs-stunt-green?style=for-the-badge&logo=none)](https://docs.rs/stunt)
![GitHub License](https://img.shields.io/badge/license-MIT-red?style=for-the-badge&logo=none)

<br>

<strong>A frontend web framework for developing reactive user interfaces with Rust.</strong>
</div>

## Features

* Macro for writing html with rust expressions, similar to that of JSX.
* Highly extensible components with support for passing down properties.
* Use any build tool you like eg. [trunk](https://trunkrs.dev/).
* Multiple ways to manage the state of your application.

## Goals

- [x] Optimized DOM api calls
- [x] Router implementation
- [ ] Webworker integration
- [ ] Support for desktop and mobile.

## Usage
This crate is on [crates.io](https://crates.io/crates/stunt) and can be added either through
adding `stunt` to your dependencies in `Cargo.toml`:
```toml
[dependencies]
stunt = "0.1.1"
```
Or running the following Cargo command in your project directory:
```bash
cargo add stunt
```

## Example
```rust
use stunt::prelude::*;

pub enum Message {
    Add,
}

pub struct App {
    count: usize,
}

impl Component for App {
    type Message = Message;
    type Properties = ();

    fn create() -> App {
        App {
            count: 0,
        }
    }

    fn callback(&mut self, message: &Message) {
        match message {
            Message::Add => {
                self.count += 1;
            },
        }
    }

    fn view(&self, _: ()) -> Html {
        html! {
            <div>
                <button onclick={ Message::Add } >
                    { "increment" }
                </button>
                <h1>
                    { self.count }
                </h1>
            </div>
        }
    }
}

fn main() {
    Renderer::new::<App>().render();
}
```

## License
stunt is licensed under the MIT license.


