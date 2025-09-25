mod register;

use register::Register;

use stunt::prelude::*;
use stunt::backend::Service;

use stunt_router::Routable;

#[cfg(not(target_arch = "wasm32"))]
use actix_web::{web, HttpServer, App as ActixApp};

#[cfg(not(target_arch = "wasm32"))]
use actix_files::Files;


#[derive(Routable)]
pub enum Route {
    #[at("/register")]
    Register,
    #[at("/register/:user_id")]
    Registered {
        user_id: usize,
    },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create() -> App { App }

    fn view(&self, _: ()) -> Html {
        // TODO: implement "typecheck" for emptybuilder to fix bug
        match stunt_router::route::<Route>() {
            Route::Register => html! {
                <Register />
            },
            Route::Registered { user_id } => html! {
                <h1>
                    { format!("registered: {}", user_id) }
                </h1>
            },
            Route::NotFound => html! {
                <h1>
                    { format!("404: Not Found") }
                </h1>
            },
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    Renderer::new::<App>().render();
}

#[cfg(not(target_arch = "wasm32"))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("listening on 127.0.0.1:8080");

    HttpServer::new(|| {
        ActixApp::new()
            .route(register::Api::PATH, web::post().to(register::Api::actix_handler))
            .service(Files::new("/", "./dist"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


