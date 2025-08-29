use stunt::prelude::*;

use stunt_router::Routable;


#[derive(Properties, Clone)]
pub struct AccountProperties {
    id: usize,
    name: String,
}

pub struct Account;

impl Component for Account {
    type Message = ();
    type Properties = AccountProperties;

    fn create() -> Account { Account }

    fn view(&self, properties: AccountProperties) -> Html {
        html! {
            <h1>
                { format!("{}-{}", properties.id, properties.name) }
            </h1>
        }
    }
}


