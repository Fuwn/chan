#![feature(proc_macro_hygiene, decl_macro, option_result_contains)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;
extern crate serde_json;

mod api;
mod ui;
mod db;
mod structures;

use rocket_contrib::templates::Template;
use actix_web::{
    error::ErrorInternalServerError, get, middleware::Logger, web,
    App, Error, HttpResponse, HttpServer
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // rocket::ignite()
    //     .attach(Template::fairing())
    //     .register(catchers![
    //         ui::not_found
    //     ])
    //     .mount("/", routes![
    //         ui::index,
    //         ui::boards,
    //         ui::board
    //     ])
    //     .mount("/api/v1/", routes![
    //         api::post
    //     ])
    //     .launch();

    HttpServer::new(move || App::new().wrap(Logger::default())
        .service(ui::index)
        .service(ui::boards)
    )
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
