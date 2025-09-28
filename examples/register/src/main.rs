mod register;

use stunt::prelude::*;
use stunt_router::Routable;

use register::Register;

#[cfg(not(target_arch = "wasm32"))]
use register::RegisterApi;

#[cfg(not(target_arch = "wasm32"))]
use actix_web::{web, HttpServer, App as ActixApp};

#[cfg(not(target_arch = "wasm32"))]
use actix_files::{NamedFile, Files};


#[derive(Routable)]
pub enum Route {
    #[at("/")]
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

    // TODO: implement once
    fn once(&mut self, link: Link) {
        stunt_router::register_callback(move || link.callback::<App>(()));
    }

    fn view(&self, _: ()) -> Html {
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
            .route(RegisterApi::PATH, web::post().to(RegisterApi::actix_handler))
            .service(Files::new("/static/", "./dist"))
            .default_service(web::get().to(async || NamedFile::open("./dist/index.html")))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


