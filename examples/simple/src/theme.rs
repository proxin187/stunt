

#[derive(Clone, PartialEq)]
pub enum ThemeState {
    Light,
    Dark,
}

#[derive(Clone)]
pub struct Theme {
    pub state: ThemeState,
    pub background: String,
}

impl Default for Theme {
    fn default() -> Theme {
        Theme {
            state: ThemeState::Light,
            background: String::from("#ffffffff"),
        }
    }
}


