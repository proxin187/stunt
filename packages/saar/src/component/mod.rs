use crate::html::{Html, Props};


pub trait Callback {}

impl Callback for Box<dyn Callback> {}

pub trait Component: 'static {
    type Callback: Callback + 'static;

    fn create() -> Self where Self: Sized;

    fn callback(&mut self, callback: Self::Callback);

    fn view(&self, ctx: Context) -> Html;
}

// we could maybe use the any::TypeId struct
// the issue is that we have to find a better way to represent callback maybe
// TODO: figure out how to represent the component here
pub enum ComponentRef {
    Component(Box<dyn Component<Callback = Box<dyn Callback>>>),
    Block(fn(Context) -> String),
}

impl ComponentRef {
    pub fn render(&self, context: Context) -> String {
        match self {
            ComponentRef::Component(component) => component.view(context).render(),
            ComponentRef::Block(block) => block(context),
        }
    }
}

pub struct Context {
    pub props: Props,
}

impl Context {
    pub fn new(props: Props) -> Context {
        Context {
            props,
        }
    }
}


