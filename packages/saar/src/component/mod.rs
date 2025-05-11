use crate::html::Html;


pub trait Callback {}

impl Callback for Box<dyn Callback> {}

pub trait Component: 'static {
    type Callback: Callback + 'static;

    fn create() -> Self where Self: Sized;

    fn callback(&mut self, callback: Self::Callback);

    fn view<'a>(&self, ctx: Context<'a>) -> Html<'a>;
}

pub enum ComponentRef {
    Component(Box<dyn Component<Callback = Box<dyn Callback>>>),
    Block(fn() -> String),
}

impl ComponentRef {
    pub fn render(&self, context: Context) -> String {
        match self {
            ComponentRef::Component(component) => component.view(context).render(),
            ComponentRef::Block(block) => block(),
        }
    }
}

pub struct Context<'a> {
    pub props: &'a [Html<'a>],
}

impl<'a> Context<'a> {
    pub fn new(props: &'a [Html<'a>]) -> Context<'a> {
        Context {
            props,
        }
    }
}


