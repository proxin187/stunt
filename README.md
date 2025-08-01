
<div align="center">
<br>

![GitHub License](https://img.shields.io/badge/license-MIT-red?style=for-the-badge&logo=none)
![dependencies](https://deps.rs/repo/github/proxin187/puri/status.svg?style=for-the-badge)
[![crates.io](https://img.shields.io/badge/crates.io-puri-red?style=for-the-badge&logo=none)](https://crates.io/crates/puri)
<h4>A declarative web framework for Rust/Wasm</h4>
</div>


## About
stunt is a frontend web framework for developing reactive user interfaces in Rust/Wasm

* Features a macro for writing html with rust expressions, similar to that of JSX.
* Highly extensible using [components](#component)
* Use any build tool you like eg. [trunk](https://trunkrs.dev/)
* Templates are injected as Text Nodes making cross-site scripting impossible

## Goals

- [x] Optimized DOM api calls
- [x] Router implementation
- [ ] Full Documentation

## Example
The following example shows a button that increments a counter when pressed.

```rust
use puri::prelude::*;


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
                self.count += 2;
            },
        }
    }

    fn view(&self, ctx: Context, _properties: ()) -> Tree {
        html! {
            <div>
                <h1>
                    <template { format!("count: {}", self.count) } />
                </h1>
                <button class={ "btn" } event: mousedown={ Message::Add }>
                    <template { format!("increment") } />
                </button>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    web_sys::console::log_1(&"loading wasm".into());

    Renderer::<App>::new().init()
}
```

## License
puri is licensed under the MIT license.


