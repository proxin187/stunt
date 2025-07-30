

#[derive(PartialEq)]
pub enum ThemeState {
    Light,
    Dark,
}

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


