
<div align="center">
<br>

[![crates.io](https://img.shields.io/badge/crates.io-stunt-red?style=flat-square&logo=none)](https://crates.io/crates/stunt)
[![docs.rs](https://img.shields.io/badge/docs.rs-stunt-green?style=flat-square&logo=none)](https://docs.rs/stunt)
![GitHub License](https://img.shields.io/badge/license-MIT-red?style=flat-square&logo=none)
![dependencies](https://deps.rs/repo/github/proxin187/stunt/status.svg?style=flat-square)

<br>

<strong>stunt is a frontend web framework for developing reactive user interfaces with Rust.</strong>
</div>

## Features

* Macro for writing html with rust expressions, similar to that of JSX.
* Highly extensible [components](#component).
* Use any build tool you like eg. [trunk](https://trunkrs.dev/).
* Multiple ways to manage the state of your application.

## Goals

- [x] Optimized DOM api calls
- [x] Router implementation
- [ ] Webworker integration
- [ ] Full Documentation

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

    fn view(&self, ctx: Context, _properties: ()) -> Tree {
        html! {
            <div>
                <button event: mousedown={ Message::Add } >
                    <? { "increment" } ?>
                </button>
                <h1>
                    <? { format!("count: {}", self.count) } ?>
                </h1>
            </div>
        }
    }
}

fn main() {
    Renderer::<App>::new().render();
}
```

## License
stunt is licensed under the MIT license.


