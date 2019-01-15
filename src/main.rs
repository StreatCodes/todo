use actix_web::{server, App};

mod user;
mod app;
mod db;

fn main() {
    let todo_db = db::setup_mongo_db();

    server::new( move ||{
        App::with_state(app::State{db: todo_db.clone()})
            .resource("/users", |r| r.f(user::get_all_handle))
            .resource("/users/{id}", |r| r.f(user::get_handle))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run();
}
