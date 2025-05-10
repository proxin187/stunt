use crate::component::Component;


pub struct Renderer<T: Component> {
    component: T,
}

impl<T: Component> Renderer<T> {
    pub fn new() -> Renderer<T> {
        Renderer {
            component: T::create(),
        }
    }

    pub fn render(&mut self) {
        let raw = self.component.view().render();

        loop {
        }
    }
}


