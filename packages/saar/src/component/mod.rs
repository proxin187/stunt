use crate::html::Html;

use std::collections::HashMap;


pub trait Callback {}

pub trait Component: 'static {
    type Callback: Callback + 'static;

    fn create() -> Self where Self: Sized;

    fn callback(&mut self, callback: Self::Callback);

    fn view(&self) -> Html;
}

pub struct BaseComponent {
    name: String,
    attributes: HashMap<String, String>,
}

impl BaseComponent {
    pub fn new(name: &str, attributes: &[(String, String)]) -> BaseComponent {
        BaseComponent {
            name: name.to_string(),
            attributes: attributes.into_iter().cloned().collect(),
        }
    }
}

// TODO: we also need a block component type where the user can pass in a function that returns a
// string
//
// this type might replace the base type

pub enum ComponentRef {
    Component(Box<dyn Component<Callback = Box<dyn Callback>>>),
    Base(BaseComponent),
}

impl ComponentRef {
}


