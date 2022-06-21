use std::sync::Mutex;

use actix_web::{get, post, web, App, HttpServer, Responder};

struct ShareAppState {
    counter: Mutex<i32>,
}

async fn index(data: web::Data<ShareAppState>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("counter {}", counter)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(ShareAppState {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
