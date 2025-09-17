use web_sys::{Request, RequestInit, RequestMode, Response, Window};
use web_sys::js_sys::JsString;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::prelude::*;


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
pub async fn post(path: String, body: String) -> Result<String, JsValue> {
    let window = web_sys::window().expect("no window found");
    let url = Url::new(&window, path).expect("failed to create url");
    let opts = RequestInit::new();

    opts.set_method("POST");

    opts.set_mode(RequestMode::Cors);

    opts.set_body(&JsString::from(body));

    /*
    wasm_bindgen_futures::spawn_local((async move || {
        match  {
            Ok(json) => {
                let string = json.dyn_into::<JsString>().expect("failed to cast");

                web_sys::console::log_1(&format!("done async: {:?}", string).into());

                *clone.json.lock() = Ok(string.into());
            },
            Err(err) => {
                web_sys::console::log_1(&format!("error async: {:?}", err).into());

                *clone.json.lock() = Err(err);
            },
        }
    */

    fetch(format!("https://jsonplaceholder.typicode.com/todos/1"), opts, window).await
        .map(|value| value.dyn_into::<JsString>().expect("failed to cast").into())
}

async fn fetch(url: String, opts: RequestInit, window: Window) -> Result<JsValue, JsValue> {
    let request = Request::new_with_str_and_init(&url, &opts)?;

    request.headers().set("Accept", "application/json")?;

    request.headers().set("Content-Type", "application/json")?;

    let value = JsFuture::from(window.fetch_with_request(&request)).await?;

    let response = value.dyn_into::<Response>().expect("failed to cast");

    JsFuture::from(response.json()?).await
}


