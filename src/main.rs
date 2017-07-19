#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate dotenv;
extern crate rocket;
extern crate pwhash;
extern crate chrono;

mod db;
mod controllers;
mod services;
mod helpers;

fn main() {
    rocket::ignite()
        .mount("/", routes![
            controllers::statics::index,
            controllers::statics::about,
            controllers::statics::post,
            controllers::statics::admin,
            controllers::statics::login,
            controllers::statics::signup,
            controllers::statics::four_oh_four,
            controllers::users::get_user,
            controllers::users::register,
            controllers::users::login,
            controllers::posts::get_posts,
            controllers::posts::get_post,
            controllers::posts::create_post,
            controllers::statics::dist_files
        ])
        .catch(errors![controllers::statics::not_found])
        .launch();
}
