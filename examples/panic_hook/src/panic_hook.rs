use stunt::prelude::*;

use std::panic;


#[derive(Default)]
pub struct PanicHook {
    message: Option<String>,
}

impl PanicHook {
    pub fn new<'a>(info: &panic::PanicHookInfo<'a>) -> PanicHook {
        PanicHook {
            message: info.payload_as_str().map(|payload| payload.to_string()),
        }
    }
}

impl Component for PanicHook {
    type Message = ();
    type Properties = ();

    fn create() -> PanicHook {
        unreachable!("create should never be called on the panic hook")
    }

    fn view(&self, _properties: ()) -> Html {
        html! {
            <p>
                { format!("panic: {:?}", self.message) }
            </p>
        }
    }
}

#[inline]
pub fn init() {
    panic::set_hook(Box::new(|info| {
        let panic_hook = PanicHook::new(info);

        Renderer::use_prepared_comp(panic_hook).render();
    }));
}


