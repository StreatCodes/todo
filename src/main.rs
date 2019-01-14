use actix_web::{server, App};
use mongodb::{Client, ThreadedClient};
use mongodb::db::{ThreadedDatabase};

mod user;
mod app;

fn main() {
    println!("Connecting to mongodb");
    let mongo_client = Client::with_uri("mongodb://todo:todo@10.1.1.25/todo")
        .expect("Failed to initialize standalone client.");
    println!("Connected");

    mongo_client.db("todo").auth("todo", "todo").expect("mongodb auth failed");
    println!("Authenticated");
    
    
    let names = mongo_client.database_names().expect("Couldn't acquire DB names");
    for name in names {
        println!("DB: {}", name);
    }


    server::new( move ||{
        App::with_state(app::State{db: mongo_client.db("todo").clone()})
            .resource("/users", |r| r.f(user::get_all_handle))
            .resource("/users/{id}", |r| r.f(user::get_handle))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run();
}
