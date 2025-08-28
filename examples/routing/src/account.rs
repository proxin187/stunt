use stunt::prelude::*;

// use stunt_router::Routable;


#[derive(Properties, Clone)]
pub struct AccountProperties {
    id: usize,
    name: String,
}

/*
impl stunt::component::Properties for AccountProperties {}

impl stunt::component::Buildable for AccountProperties {
    type Builder = _AccountPropertiesBuilder;

    fn builder() -> Self::Builder {
        _AccountPropertiesBuilder {
            id: None,
            name: None,
        }
    }
}

#[allow(non_camel_case_types)]
pub struct HasProp_id<Token>(Token);

#[allow(non_camel_case_types)]
pub struct HasProp_name<Token>(Token);

pub struct _AccountPropertiesBuilder {
    id: Option<usize>,
    name: Option<String>,
}

impl _AccountPropertiesBuilder {
    pub fn id<Token>(&mut self, token: Token, value: usize) -> HasProp_id<Token> {
        self.id.replace(value);

        HasProp_id(token)
    }

    pub fn name<Token>(&mut self, token: Token, value: String) -> HasProp_name<Token> {
        self.name.replace(value);

        HasProp_name(token)
    }

    pub fn build(self, _token: HasProp_name<HasProp_id<()>>) -> AccountProperties {
        AccountProperties {
            id: self.id.unwrap(),
            name: self.name.unwrap(),
        }
    }
}
*/

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


