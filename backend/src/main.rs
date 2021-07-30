use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
#[allow(unused_imports)]
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json;
#[allow(unused_imports)]
use std::error::Error;
#[allow(unused_imports)]
use std::result::Result;
use wither::bson::{doc, oid::ObjectId};
use wither::mongodb::{Client, Database};
use wither::prelude::*;

async fn hash_password(password: String) -> String {
    let hashed = hash(&password, DEFAULT_COST);
    hashed.unwrap()
}

#[derive(Debug, Default, Deserialize, Model, Serialize)]
#[model(index(keys = r#"doc!{"email": 1}"#, options = r#"doc!{"unique": true}"#))]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    password: String,
    pub pub_key: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ChangePassword {
    pub email: String,
    pub password: String,
    pub new_password: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ChangePubKey {
    pub email: String,
    pub password: String,
    pub new_pub_key: String,
}

impl User {
    pub async fn new(email: String, password: String, pub_key: String) -> Self {
        Self {
            id: None,
            email,
            password,
            pub_key,
        }
    }

    pub async fn register(&mut self, db: &Database) -> Result<User, wither::WitherError> {
        let hashed = hash_password(self.password.clone()).await;
        self.password = hashed.clone();
        match self.save(db, None).await {
            Ok(()) => Ok(User {
                id: self.id.clone(),
                email: self.email.clone(),
                password: hashed,
                pub_key: self.pub_key.clone(),
            }),
            Err(e) => Err(e),
        }
    }

    pub async fn change_password(
        db: &Database,
        email: String,
        password: String,
        new_password: String,
    ) -> Result<(), wither::WitherError> {
        let user = User::find_one(db, doc! {"email": email}, None)
            .await
            .unwrap();

        if let Some(user) = user {
            if bcrypt::verify(password, &user.password).unwrap() {
                let new_hashed_password = hash_password(new_password).await;
                user.update(
                    db,
                    None,
                    doc! {"$set": doc!{"password": new_hashed_password}},
                    None,
                )
                .await
                .unwrap();
            }
        }

        Ok(())
    }
    pub async fn chagen_pub_key(
        db: &Database,
        email: String,
        password: String,
        new_pub_key: String,
    ) -> Result<(), wither::WitherError> {
        let user = User::find_one(db, doc! {"email": email}, None)
            .await
            .unwrap();

        if let Some(user) = user {
            println!("{:?}", &user);
            if bcrypt::verify(password, &user.password).unwrap() {
                user.update(db, None, doc! {"$set": doc!{"pub_key": new_pub_key}}, None)
                    .await
                    .unwrap();
            }
        } else {
            format!("User not found");
        }

        Ok(())
    }
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn register(form: web::Json<User>, database: web::Data<Database>) -> impl Responder {
    let mut user = form.into_inner();
    match user.register(&database).await {
        Ok(user) => serde_json::to_string_pretty(&user).unwrap(),
        Err(e) => e.to_string(),
    }
}

async fn change_password(
    form: web::Json<ChangePassword>,
    database: web::Data<Database>,
) -> impl Responder {
    let user = form.into_inner();
    let chgpasswd =
        User::change_password(&database, user.email, user.password, user.new_password).await;

    match chgpasswd {
        Ok(_) => format!("Password changed"),
        Err(e) => e.to_string(),
    }
}

async fn change_pubkey(
    form: web::Json<ChangePubKey>,
    database: web::Data<Database>,
) -> impl Responder {
    let user = form.into_inner();
    let chg = User::chagen_pub_key(&database, user.email, user.password, user.new_pub_key).await;

    match chg {
        Ok(_) => format!("Public key changed"),
        Err(e) => e.to_string(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Client::with_uri_str("mongodb://localhost:27017/")
        .await
        .unwrap()
        .database("sel");
    User::sync(&db).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .route("/", web::get().to(greet))
            .route("/register", web::post().to(register))
            .route("/change_password", web::post().to(change_password))
            .route("/change_pub_key", web::post().to(change_pubkey))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
