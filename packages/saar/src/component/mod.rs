use crate::html::Html;


pub trait Component<T> {
    fn update(&self, msg: T);

    fn view(&self) -> Html;
}


