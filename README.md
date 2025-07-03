
<div align="center">
<br>

![GitHub License](https://img.shields.io/badge/license-MIT-red?style=for-the-badge&logo=none)
![dependencies](https://deps.rs/repo/github/proxin187/puri/status.svg?style=for-the-badge)
[![crates.io](https://img.shields.io/badge/crates.io-puri-red?style=for-the-badge&logo=none)](https://crates.io/crates/puri)
<h4>A declarative web framework for Rust/Wasm</h4>
</div>


## About
puri is a frontend web framework for developing reactive user interfaces in Rust/Wasm

* Features a macro for writing html with rust expressions, similar to that of JSX.
* Everything is a [component](#component)
* Use any build tool you like eg. [trunk](https://trunkrs.dev/)

## Example
The following example shows a button that increments a counter when pressed.

```rust
use puri::prelude::*;

use std::sync::Arc;
use std::any::Any;


pub enum Message {
    Add,
}

pub struct App {
    count: usize,
}

impl Component for App {
    fn create() -> App {
        App {
            count: 0,
        }
    }

    fn callback(&mut self, callback: &Arc<dyn Any + Send + Sync>) {
        match callback.downcast_ref::<Message>() {
            Some(Message::Add) => {
                self.count += 5;
            },
            None => unreachable!(),
        }
    }

    fn view(&self, ctx: Context) -> Node {
        html! {
            <div>
                <h1 style={ "background-color: blue;" }>
                    <template { format!("count: {}", self.count) } />
                </h1>
                <button event: mousedown={ Arc::new(Message::Add) }>
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


