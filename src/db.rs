
use mongodb::{Client, ThreadedClient};
use mongodb::db::{ThreadedDatabase, DatabaseInner};
use mongodb::coll::options::IndexOptions;
use bson::*;

pub fn setup_mongo_db() -> std::sync::Arc<DatabaseInner> {
    println!("Connecting to mongodb");
    let mongo_client = Client::with_uri("mongodb://todo:todo@10.1.1.25/todo")
        .expect("Failed to initialize standalone client.");
    println!("Connected");

    let db = mongo_client.db("todo");
	db.auth("todo", "todo").expect("mongodb auth failed");
    println!("Authenticated\nSetting up index");

	let mut user_options = IndexOptions::new();
	user_options.unique = Some(true);

	db.collection("users").create_index(
		doc!{"email": 1},
		Some(user_options)
	).expect("Couldn't create index on users collection.");
	
	mongo_client.db("todo")
}