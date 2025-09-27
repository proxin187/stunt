
<div align="center">
<br>
<a href="https://github.com/proxin187/stunt">
    <img src="https://raw.githubusercontent.com/proxin187/stunt/refs/heads/main/assets/stunt_logo.png?" width="200">
</a>

<h1>stunt</h1>

<br>
<br>

[![crates.io](https://img.shields.io/badge/crates.io-stunt-red?style=for-the-badge&logo=none)](https://crates.io/crates/stunt)
[![docs.rs](https://img.shields.io/badge/docs.rs-stunt-green?style=for-the-badge&logo=none)](https://docs.rs/stunt)
![GitHub License](https://img.shields.io/badge/license-MIT-red?style=for-the-badge&logo=none)

<br>

<strong>An isomorphic web framework for Rust.</strong>
</div>

## Features

* Isomorphic, define backend services and easily call into them from the frontend.
* Macro for writing html with rust expressions, similar to that of JSX.
* Highly extensible components with compile-time type checked properties.
* Use any build tool you like eg. [trunk](https://trunkrs.dev/).

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
stunt = "0.1.3"
```
Or running the following Cargo command in your project directory:
```bash
cargo add stunt
```

## Example
More examples can be found at [examples](https://github.com/proxin187/stunt/tree/main/examples).

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

## Contributing
We highly appreciate all contributions whether its a bug fix, feature, or documentation.
If you encounter any bugs or otherwise weird behaviour we would really appreciate if you submitted an issue for us to look into.

## License
stunt is licensed under the MIT license.


