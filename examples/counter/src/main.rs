use stunt::prelude::*;
use stunt::backend::{Service, NullTransport};

use serde::{Serialize, Deserialize};

#[cfg(not(target_arch = "wasm32"))]
use actix_web::{web, HttpServer, App as ActixApp};

#[cfg(not(target_arch = "wasm32"))]
use actix_files::Files;


#[derive(Clone, Serialize, Deserialize)]
pub struct Register {
    username: String,
    id: usize,
}

impl Register {
    pub fn new(username: String, id: usize) -> Register {
        Register {
            username,
            id,
        }
    }
}

impl Service for Register {
    const PATH: &'static str = "/api/register";

    type Output = NullTransport;

    fn handle(self) -> NullTransport {
        println!("username: {}, id: {}", self.username, self.id);

        NullTransport
    }
}

pub enum Message {
    Registered,
    Add,
}

pub struct App {
    count: usize,
}

impl Component for App {
    type Message = Message;
    type Properties = ();

    fn create() -> App {
        App {
            count: 0,
        }
    }

    fn callback(&mut self, message: &Message, link: Link) {
        match message {
            Message::Registered => {
            },
            Message::Add => {
                self.count += 1;

                Register::new(String::from("user"), 123)
                    .call(move |_| link.callback::<App>(Message::Registered));
            },
        }
    }

    fn view(&self, _: ()) -> Html {
        html! {
            <div>
                <button onclick={ Message::Add } >
                    { "increment" }
                </button>
                <h1>
                    { self.count }
                </h1>
            </div>
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
    println!("listening on 127.0.0.1:8080, dir: {:?}", std::env::current_dir());

    HttpServer::new(|| {
        ActixApp::new()
            .service(Files::new("/dist", "."))
            .route(Register::PATH, web::post().to(Register::actix_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


