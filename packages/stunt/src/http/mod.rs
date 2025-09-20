use web_sys::{Request, RequestInit, RequestMode, Response, Window};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::prelude::*;

use serde::Serialize;
use serde::de::DeserializeOwned;


struct Url {
    protocol: String,
    hostname: String,
    path: String,
}

impl std::fmt::Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.write_fmt(format_args!("{}//{}{}", self.protocol, self.hostname, self.path))
    }
}

impl Url {
    fn new(window: &Window, path: String) -> Result<Url, JsValue> {
        Ok(Url {
            protocol: window.location().protocol()?,
            hostname: window.location().hostname()?,
            path,
        })
    }
}

#[inline]
pub async fn post<Input: Serialize + ?Sized, Output: DeserializeOwned>(path: String, input: &Input) -> Result<Output, JsValue> {
    let window = web_sys::window().expect("no window found");
    let url = Url::new(&window, path)?;
    let opts = RequestInit::new();

    opts.set_method("POST");

    opts.set_mode(RequestMode::Cors);

    opts.set_body(&serde_wasm_bindgen::to_value(&input)?);

    fetch(url.to_string(), opts, window).await
        .and_then(|value| serde_wasm_bindgen::from_value(value).map_err(|err| JsValue::from_str(&err.to_string())))
}

async fn fetch(url: String, opts: RequestInit, window: Window) -> Result<JsValue, JsValue> {
    let request = Request::new_with_str_and_init(&url, &opts)?;

    request.headers().set("Accept", "application/json")?;

    request.headers().set("Content-Type", "application/json")?;

    let value = JsFuture::from(window.fetch_with_request(&request)).await?;

    let response = value.dyn_into::<Response>().expect("failed to cast");

    JsFuture::from(response.json()?).await
}


