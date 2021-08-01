use actix_web::{get, http, web, App, HttpRequest, HttpServer, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
#[allow(unused_imports)]
use futures::stream::StreamExt;
use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use wither::bson::{doc, oid::ObjectId};
use wither::mongodb::{
    options::{ClientOptions, StreamAddress},
    Client, Database,
};
use wither::prelude::*;

// issuer
use bip32::{Error, ExtendedPrivateKey, ExtendedPublicKey, Language};
use bip32::{Mnemonic, Prefix, XPrv};
use k256::ecdsa::signature::Signature;
use k256::ecdsa::{SigningKey, VerifyingKey};
use rand_core::OsRng;
use rsa::{
    PaddingScheme, PrivateKeyPemEncoding, PublicKey, PublicKeyPemEncoding, RSAPrivateKey,
    RSAPublicKey,
};
use sha1;
use sha2::Sha256;
#[allow(unused_imports)]
use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::result::Result;
use std::str::from_utf8;
use std::str::FromStr;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Register {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: String, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
    nbf: usize, // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}

impl Claims {
    pub fn new(sub: String) -> Self {
        let iat: usize = Utc::now().timestamp_millis() as usize;
        let exp: usize = iat + 86_400_000;
        let aud: String = String::from("org.koompi.sel");
        let nbf: usize = iat + 1_000;
        let iss: String = String::from("https://sel.koompi.org");
        Self {
            aud,
            exp,
            iat,
            iss,
            nbf,
            sub,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ChangePassword {
    pub email: String,
    pub password: String,
    pub new_password: String,
    pub token: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ChangePubKey {
    pub email: String,
    pub password: String,
    pub new_pub_key: String,
    pub token: String,
}

pub enum AppError {
    SignInError(SignInError),
    DatabaseError(wither::WitherError),
    InvalidToken,
    InvalidPublicKey(InvalidPublicKey),
}

pub enum InvalidPublicKey {
    InvalidBase64,
    InvalidRSA,
    Others(String),
}

pub enum SignInError {
    InvalidEmail,
    InvalidPassword,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignAddress {
    pub token: String,
    pub url: String,
    pub user_pub_key: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SignIn {
    pub email: String,
    pub password: String,
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

    pub async fn sign_in(
        db: &Database,
        email: String,
        password: String,
    ) -> Result<String, AppError> {
        let user = User::find_one(db, doc! {"email": email}, None).await;

        match user {
            Ok(user) => {
                if let Some(user) = user {
                    if verify(password, &user.password).unwrap() {
                        let k = "SECRET";
                        let value = dotenv::var(k).unwrap();
                        let key = EncodingKey::from_secret(value.as_bytes());
                        let token =
                            encode(&Header::default(), &Claims::new(user.email.clone()), &key)
                                .unwrap();
                        let bearer_token = format!("{}", token);
                        Ok(bearer_token)
                    } else {
                        Err(AppError::SignInError(SignInError::InvalidPassword))
                    }
                } else {
                    Err(AppError::SignInError(SignInError::InvalidEmail))
                }
            }
            Err(e) => Err(AppError::DatabaseError(e)),
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

async fn register(form: web::Json<Register>, database: web::Data<Database>) -> impl Responder {
    let data = form.into_inner();
    let mut user = User {
        id: None,
        email: data.email,
        password: data.password,
        pub_key: String::new(),
    };
    match user.register(&database).await {
        Ok(user) => serde_json::to_string_pretty(&user).unwrap(),
        Err(e) => e.to_string(),
    }
}

async fn sign_in(form: web::Json<SignIn>, database: web::Data<Database>) -> impl Responder {
    let data = form.into_inner();
    let signing_in = User::sign_in(&database, data.email, data.password).await;

    match signing_in {
        Ok(token) => serde_json::to_string_pretty(&Token { token: token }).unwrap(),
        Err(e) => match e {
            AppError::DatabaseError(db_err) => db_err.to_string(),
            AppError::SignInError(s_err) => match s_err {
                SignInError::InvalidEmail => String::from("Incorrect email or user not exists"),
                SignInError::InvalidPassword => String::from("Invalid password"),
            },
            _ => String::from("Other error"),
        },
    }
}

async fn change_password(
    form: web::Json<ChangePassword>,
    database: web::Data<Database>,
) -> impl Responder {
    let user = form.into_inner();
    if !user.token.is_empty() {
        if validate_token(&user.token).is_ok() {
            let chgpasswd =
                User::change_password(&database, user.email, user.password, user.new_password)
                    .await;

            match chgpasswd {
                Ok(_) => format!("Password changed"),
                Err(e) => e.to_string(),
            }
        } else {
            format!("Invalid token")
        }
    } else {
        format!("Unauthorized")
    }
}

async fn change_pubkey(
    form: web::Json<ChangePubKey>,
    database: web::Data<Database>,
) -> impl Responder {
    let user = form.into_inner();
    if !user.token.is_empty() {
        if validate_token(&user.token).is_ok() {
            let chg =
                User::chagen_pub_key(&database, user.email, user.password, user.new_pub_key).await;

            match chg {
                Ok(_) => format!("Public key changed"),
                Err(e) => e.to_string(),
            }
        } else {
            format!("Invalid token")
        }
    } else {
        format!("Unauthorized")
    }
}

pub async fn sign_address(
    form: web::Json<SignAddress>,
    database: web::Data<Database>,
) -> impl Responder {
    // use protocol::rsa_sec::rsa_encrypt;

    let data = form.into_inner();
    if !data.token.is_empty() {
        if validate_token(&data.token).is_ok() {
            let input = data.url.clone();
            let user_pub = &data.user_pub_key;
            let enc_input = rsa_encrypt(input, &user_pub);
            match enc_input {
                Ok(enc_input) => {
                    let enc_str = base64::encode_config(&enc_input, base64::URL_SAFE_NO_PAD);
                    let sigb = sign_byte(&enc_input);
                    let sigb_str = base64::encode_config(&sigb, base64::URL_SAFE_NO_PAD);
                    #[cfg(debug_assertions)]
                    return format!("http://localhost:8080/init/{}/{}", &sigb_str, &enc_str);
                    #[cfg(not(debug_assertions))]
                    return format!("http://sel.koompi.org/init/{}/{}", &sigb_str, &enc_str);
                }
                Err(e) => match e {
                    AppError::InvalidPublicKey(k) => match k {
                        InvalidPublicKey::InvalidBase64 => {
                            format!("Invalid based64 format of public key")
                        }
                        InvalidPublicKey::InvalidRSA => format!("Invalid RSA public key"),
                        InvalidPublicKey::Others(s) => format!("Signing error: {}", s),
                    },
                    _ => return format!("Unknown error at Signing"),
                },
            }
        } else {
            format!("Invalid token")
        }
    } else {
        format!("Unauthorized")
    }
}

pub fn validate_token(token: &str) -> Result<(), AppError> {
    let k = "SECRET";
    let value = dotenv::var(k).unwrap();
    let res = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(value.as_ref()),
        &Validation::new(Algorithm::HS256),
    );
    match res {
        Ok(_) => Ok(()),
        Err(_) => Err(AppError::InvalidToken),
    }
}

#[derive(Deserialize)]
struct Info {
    cert: String,
    cmd: String,
}

#[get("/init/{cert}/{cmd}")] // <- define path parameters
async fn to_sel_uri(info: web::Path<Info>) -> actix_web::HttpResponse {
    let location = format!("sel://init/{}?cmd={}", &info.cert, &info.cmd);
    let content = format!(
    "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>SEL Redirect</title><script type=\"text/javascript\">window.location.replace(\"{}\")</script></head><body></body></html>",
    location
    );
    actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body(content)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use dotenv;
    dotenv::dotenv().ok();
    #[cfg(debug_assertions)]
    let uri = format!("mongodb://localhost:27017/");
    #[cfg(not(debug_assertions))]
    let uri = format!("mongodb://localhost:27020/");
    let db = Client::with_uri_str(&uri).await.unwrap().database("sel");
    User::sync(&db).await.unwrap();

    HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600);
        App::new()
            .wrap(cors)
            .data(db.clone())
            .service(
                web::scope("/api")
                    .route("/change_password", web::post().to(change_password))
                    .route("/change_pub_key", web::post().to(change_pubkey))
                    .route("/sign", web::post().to(sign_address)),
            )
            .route("/sign_in", web::post().to(sign_in))
            .route("/register", web::post().to(register))
            .route("/", web::get().to(greet))
            .service(to_sel_uri)
    })
    .bind(("0.0.0.0", 3690))?
    .run()
    .await
}

pub fn rsa_encrypt(data: String, pubkey: &str) -> Result<Vec<u8>, AppError> {
    let mut rng = OsRng;
    let der_bytes = base64::decode(&pubkey);
    match der_bytes {
        Ok(der_bytes) => {
            let public_key = RSAPublicKey::from_pkcs8(&der_bytes);
            match public_key {
                Ok(public_key) => {
                    let padding = PaddingScheme::new_oaep::<Sha256>();
                    let enc_data = public_key.encrypt(&mut rng, padding, data.as_bytes());

                    match enc_data {
                        Ok(d) => Ok(d),
                        Err(e) => Err(AppError::InvalidPublicKey(InvalidPublicKey::Others(
                            e.to_string(),
                        ))),
                    }
                }
                Err(e) => Err(AppError::InvalidPublicKey(InvalidPublicKey::InvalidRSA)),
            }
        }
        Err(e) => Err(AppError::InvalidPublicKey(InvalidPublicKey::InvalidBase64)),
    }
}

pub fn sign_byte(input: &[u8]) -> Vec<u8> {
    let k = "PRIVATE";
    let value = dotenv::var(k).unwrap();
    let mut prv_str = String::new();
    let mut prv_file = File::open(&value).unwrap();
    prv_file.read_to_string(&mut prv_str).unwrap();

    use bip32::secp256k1::ecdsa::{signature::Signer, Signature};

    let prv: ExtendedPrivateKey<SigningKey> = ExtendedPrivateKey::from_str(&prv_str).unwrap();
    let signature: Signature = prv.private_key().sign(&input);
    let sig_string = signature.as_bytes();
    sig_string.to_vec()
    // let mut sig_file = File::create(cert_name).unwrap();
    // sig_file.write_all(sig_string).unwrap();
}
