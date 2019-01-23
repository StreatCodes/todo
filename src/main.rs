use actix_web::{server, App};

mod user;
mod app;
mod db;

fn main() {
    let todo_db = db::setup_mongo_db();

    println!("Ready.\n\n");
    server::new( move ||{
        App::with_state(app::State{db: todo_db.clone()})
            .resource("/users", |r| {
                r.method(http::Method::GET).f(user::get_all_handle);
                r.method(http::Method::PUT).f(user::create_handle)
            })
            .resource("/users/{id}", |r| {
                r.method(http::Method::GET).f(user::get_handle);
                r.method(http::Method::PATCH).f(user::update_handle);
                r.method(http::Method::DELETE).f(user::delete_handle)
            })
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run();
}
