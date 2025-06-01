use crate::dom::component::Component;
use crate::dom::tree::Tree;
use crate::dom::state::{self, State};
use crate::scheduler;

use web_sys::HtmlElement;

use wasm_bindgen::JsValue;

use std::marker::PhantomData;
use std::sync::Arc;


pub struct Renderer<T: Component + 'static> {
    body: HtmlElement,
    _marker: PhantomData<T>,
}

impl<T: Component + 'static> Renderer<T> {
    pub fn new() -> Renderer<T> {
        let window = web_sys::window().expect("no global window exists");
        let document = window.document().expect("should have a document on window");

        Renderer {
            body: document.body().expect("document should have a body"),
            _marker: PhantomData,
        }
    }

    #[inline]
    fn render(&self) {
        web_sys::console::log_1(&format!("trying to render").into());

        let raw = state::with(|state| state[0].render());

        web_sys::console::log_1(&format!("raw: {:?}", raw).into());

        self.body.set_inner_html(&raw);
    }

    // the issue is not here
    fn create(&self) {
        let component = T::create();

        let tree = Tree::new(component.view(), 0);

        state::with(|state| state.push(State::new(Arc::new(component), tree)));
    }

    pub fn init(self) -> Result<(), JsValue> {
        self.create();

        self.render();

        loop {
            match scheduler::with(|scheduler| scheduler.recv()) {
                Ok(callback) => {
                    // TODO: we have to have some way to represent the job, eg. say whether its a
                    // callback or whatever it is
                    //
                    // maybe we can only support callbacks for now

                    self.render();
                },
                Err(err) => {
                    web_sys::console::log_1(&format!("scheduler error: {:?}", err).into());
                },
            }
        }
    }
}


