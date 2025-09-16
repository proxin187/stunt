use web_sys::{Request, RequestInit, RequestMode, Response, Window};
use web_sys::js_sys::JsString;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::prelude::*;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use spin::Mutex;


pub struct Status {
    ready: AtomicBool,
    json: Mutex<Result<String, JsValue>>,
}

impl Status {
    pub fn new() -> Status {
        Status {
            ready: AtomicBool::new(false),
            json: Mutex::new(Ok(String::default())),
        }
    }

    pub fn wait(&self) -> Result<String, JsValue> {
        while self.ready.load(Ordering::Relaxed) != true {}

        self.json.lock().clone()
    }
}

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
pub fn post(path: String, body: String) -> Arc<Status> {
    let status = Arc::new(Status::new());
    let window = web_sys::window().expect("no window found");
    let url = Url::new(&window, path).expect("failed to create url");
    let opts = RequestInit::new();

    opts.set_method("POST");

    opts.set_mode(RequestMode::Cors);

    opts.set_body(&JsString::from(body));

    let clone = status.clone();

    wasm_bindgen_futures::spawn_local((async move || {
        match fetch(format!("https://jsonplaceholder.typicode.com/todos/1"), opts, window).await {
            Ok(json) => {
                let string = json.dyn_into::<JsString>().expect("failed to cast");

                *clone.json.lock() = Ok(string.into());
            },
            Err(err) => {
                *clone.json.lock() = Err(err);
            },
        }

        clone.ready.store(true, Ordering::Relaxed);
    })());

    Arc::clone(&status)
}

async fn fetch(url: String, opts: RequestInit, window: Window) -> Result<JsValue, JsValue> {
    let request = Request::new_with_str_and_init(&url, &opts)?;

    request.headers().set("Accept", "application/json")?;

    request.headers().set("Content-Type", "application/json")?;

    let value = JsFuture::from(window.fetch_with_request(&request)).await?;

    let response = value.dyn_into::<Response>().expect("failed to cast");

    JsFuture::from(response.json()?).await
}


