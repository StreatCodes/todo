use actix_web::{HttpResponse, HttpRequest, Body};
use actix_web::{AsyncResponder, FutureResponse, HttpMessage};
use actix_web::{Path, FromRequest};
use bson::*;
use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;
use mongodb::db::{ThreadedDatabase};
use futures::Future;
use bytes::Bytes;
use http;
use bcrypt;

use crate::app;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    name: String,
    email: String,
    password: String,
    admin: bool
}

/// Get all users
pub fn get_all_handle(req: &HttpRequest<app::State>) -> HttpResponse {
    let coll = req.state().db.collection("users");
    let mut cur = coll.find(None, None).expect("Error finding");
    let users = cur.drain_current_batch().unwrap();
    
    let res = serde_json::to_string(&users).unwrap();

    HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(res)
}

/// Get user
pub fn get_handle(req: &HttpRequest<app::State>) -> HttpResponse {
    let coll = req.state().db.collection("users");
    let params = Path::<String>::extract(req).expect("No ID found in URL parameter");
    let id = params.into_inner();

    let filter = doc! {
        "_id": ObjectId::with_string(&id).expect("Error converting ID to OID")
    };

    let cur = coll.find_one(Some(filter), None).expect("Error finding");

    let user = cur.expect("No results");
    let res = serde_json::to_string(&user).expect("Error converting user to json");

    HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(res)
}

/// Create user endpoint
pub fn create_handle(req: &HttpRequest<app::State>) -> FutureResponse<HttpResponse> {
    let coll = req.state().db.collection("users");
    req.body().limit(2048).from_err().and_then(move |b: Bytes| {
        let body = std::str::from_utf8(&b).expect("Couldn't decode body");
        let mut user: User = serde_json::from_str(body)
            .expect("Error parsing json body");

        user.password = bcrypt::hash(&user.password, bcrypt::DEFAULT_COST)
            .expect("Error hashing password");

        let serialized = bson::to_bson(&user).expect("Error encoding to bson");

        if let bson::Bson::Document(document) = serialized {
            coll.insert_one(document, None).expect("Failed to insert");
        } else {
            println!("Error converting the BSON object into a MongoDB document");
        }

        Ok(HttpResponse::Ok().into())
    }).responder()
}

/// Update user endpoint
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

/// Delete user endpoint
pub fn delete_handle(req: &HttpRequest<app::State>) -> HttpResponse {
    let coll = req.state().db.collection("users");
    let params = Path::<String>::extract(req).expect("No ID found in URL parameter");
    let id = params.into_inner();

    let filter = doc! {
        "_id": ObjectId::with_string(&id).expect("Error converting ID to OID")
    };

    coll.delete_one(filter, None).expect("Error finding");

    HttpResponse::Ok().into()
}