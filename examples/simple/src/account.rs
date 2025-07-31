use puri::prelude::*;

use puri_router::Routable;


#[derive(Properties, Routable)]
pub struct AccountProperties {
    id: usize,
    name: String,
}

pub struct Account;

impl Component for Account {
    type Message = ();
    type Properties = AccountProperties;

    fn create() -> Account { Account }

    fn callback(&mut self, _: &()) {}

    fn view(&self, ctx: Context, properties: AccountProperties) -> Tree {
        html! {
            <h1>
                <? { format!("id: {}, name: {}", properties.id, properties.name) } ?>
            </h1>
        }
    }
}


