use actix_web::{HttpResponse, HttpRequest};
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
use serde_json::*;

use crate::app;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    name: Option<String>,
    email: Option<String>,
    password: Option<String>,
    admin: Option<bool>
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

        let pass = user.password.expect("Error expected password in request");

        user.password = Some(bcrypt::hash(&pass, bcrypt::DEFAULT_COST)
            .expect("Error hashing password"));

        let serialized = bson::to_bson(&user).expect("Error encoding to bson");

        let result = if let bson::Bson::Document(document) = serialized {
            let res = coll.insert_one(document, None).expect("Failed to insert");

            if let Some(ex) = res.write_exception {
                match ex.write_error {
                    Some(err) => {
                        HttpResponse::Ok().body(
                            json!({"error": {"code": err.code, "message": err.message}}).to_string()
                        )
                    },
                    _ => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR)
                }
            } else {
                HttpResponse::Ok().body(json!({"id": res.inserted_id}).to_string())
            }
        } else {
            HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR)
        };

        Ok(result.into())
    }).responder()
}

/// Update user endpoint
pub fn update_handle(req: &HttpRequest<app::State>) -> FutureResponse<HttpResponse> {
    let coll = req.state().db.collection("users");

    let params = Path::<String>::extract(req).expect("No ID found in URL parameter");
    let id = params.into_inner();

    req.body().limit(2048).from_err().and_then(move |b: Bytes| {
        let body = std::str::from_utf8(&b).expect("Couldn't decode body");
        let mut user: User = serde_json::from_str(body)
            .expect("Error parsing json body");

        match user.password {
            Some(pass) => {
                user.password = Some(bcrypt::hash(&pass, bcrypt::DEFAULT_COST)
                .expect("Error hashing password"));
            }, 
            None => ()
        }

        let serialized = bson::to_bson(&user).expect("Error encoding to bson");

        let filter = doc! {
            "_id": ObjectId::with_string(&id).expect("Error converting ID to OID")
        };

        let result = if let bson::Bson::Document(document) = serialized {
            let res = coll.update_one(filter, doc!{"$set": document}, None).expect("Failed to insert");

            if let Some(ex) = res.write_exception {
                match ex.write_error {
                    Some(err) => {
                        HttpResponse::Ok().body(
                            json!({"error": {"code": err.code, "message": err.message}}).to_string()
                        )
                    },
                    _ => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR)
                }
            } else {
                HttpResponse::new(http::StatusCode::OK)
            }
        } else {
            HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR)
        };

        Ok(result.into())
    }).responder()
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