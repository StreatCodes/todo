use actix_web::{HttpResponse, HttpRequest, Body};
use actix_web::{AsyncResponder, FutureResponse, HttpMessage};
use actix_web::{Path, FromRequest};
use bson::*;
use bson::oid::ObjectId;
use mongodb::db::{ThreadedDatabase};
use futures::Future;
use bytes::Bytes;
use http;

use crate::app;

pub struct User {
	name: String,
	email: String,
	password: String,
	admin: bool
}

/// get all user endpoint
pub fn get_all_handle(req: &HttpRequest<app::State>) -> HttpResponse {
    let coll = req.state().db.collection("users");
    let mut cur = coll.find(None, None).expect("Error finding");
    let users = cur.drain_current_batch().unwrap();
    
    let res = serde_json::to_string(&users).unwrap();

    HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(res)
}

/// get user endpoint
pub fn get_handle(req: &HttpRequest<app::State>) -> HttpResponse {
    let coll = req.state().db.collection("users");
    let params = Path::<String>::extract(req).expect("No ID found in URL parameter");
    let id = params.into_inner();

    let filter = doc! {
        "_id": ObjectId::with_string(&id).expect("Error converting ID to OID")
    };

    let mut cur = coll.find_one(Some(filter), None).expect("Error finding");

    let user = cur.expect("No results");
    let res = serde_json::to_string(&user).expect("Error converting user to json");

    HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(res)
}

/// Create user endpoint
pub fn create_handle(req: &HttpRequest<app::State>) -> FutureResponse<HttpResponse> {
    println!("{:?}", req);
    let coll = req.state().db.collection("movies");

    req.body().limit(2048).from_err().and_then(|b: Bytes| {
        Ok(HttpResponse::Ok().body(b).into())
    }).responder()
}

/// Create user endpoint
pub fn update_handle(req: &HttpRequest<app::State>) -> HttpResponse {
    println!("{:?}", req);

    let coll = req.state().db.collection("movies");
    println!("Collection set");
    let mut cur = coll.find(None, None).expect("Error finding");
    println!("queried movies collection");

    // for item in cur {
    //     let i = item.expect("it cool");
    //     println!("{}", i);
    // }
    let res = cur.next().unwrap().unwrap();

    let j = serde_json::to_string(&res).unwrap();
    let to = req.match_info().get("name").unwrap_or("World");

    let name = req.match_info().get("name").unwrap_or("World");
    HttpResponse::Ok().body(format!("Hi {}, heres your document:\n\n{}", name, j))
}

/// Create user endpoint
pub fn delete_handle(req: &HttpRequest<app::State>) -> HttpResponse {
    println!("{:?}", req);

    let coll = req.state().db.collection("movies");
    println!("Collection set");
    let mut cur = coll.find(None, None).expect("Error finding");
    println!("queried movies collection");

    // for item in cur {
    //     let i = item.expect("it cool");
    //     println!("{}", i);
    // }
    let res = cur.next().unwrap().unwrap();

    let j = serde_json::to_string(&res).unwrap();
    let to = req.match_info().get("name").unwrap_or("World");

    let name = req.match_info().get("name").unwrap_or("World");
    HttpResponse::Ok().body(format!("Hi {}, heres your document:\n\n{}", name, j))
}