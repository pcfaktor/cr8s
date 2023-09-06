pub mod authorization;
pub mod crates;
pub mod rustaceans;

use diesel::PgConnection;
use lettre::transport::smtp::authentication::Credentials;
use rocket::http::hyper::header;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::response::status::Custom;
use rocket::serde::json::{serde_json::json, Value};
use rocket::Request;

use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use rocket_db_pools::{deadpool_redis, Connection, Database};

use crate::mail::HtmlMailer;
use crate::models::{RoleCode, User};
use crate::repositories::{RoleRepository, UserRepository};

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConnection(PgConnection);

#[derive(Database)]
#[database("redis")]
pub struct CacheConnection(deadpool_redis::Pool);

pub fn server_error(e: Box<dyn std::error::Error>) -> Custom<Value> {
    log::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}

pub struct EditorUser(User);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for EditorUser {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = request
            .guard::<User>()
            .await
            .expect("Cannot retrieve logged in user in request guard");
        let db = request
            .guard::<DbConnection>()
            .await
            .expect("Cannot connect to postgres in request guard");

        let editor_result = db
            .run(
                |connection| match RoleRepository::find_by_user(connection, &user) {
                    Ok(roles) => {
                        log::info!("Assigned roles {:?}", roles);
                        let is_editor = roles.iter().any(|role| match role.code {
                            RoleCode::Admin => true,
                            RoleCode::Editor => true,
                            _ => false,
                        });
                        log::info!("Is editor is {:?}", is_editor);
                        is_editor.then_some(EditorUser(user))
                    }
                    _ => None,
                },
            )
            .await;
        match editor_result {
            Some(editor) => Outcome::Success(editor),
            _ => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request
            .headers()
            .get_one(header::AUTHORIZATION.as_str())
            .map(|v| v.split_whitespace().collect::<Vec<_>>())
            .filter(|v| v.len() == 2 && v[0] == "Bearer");

        if let Some(header_value) = auth_header {
            let mut cache = request
                .guard::<Connection<CacheConnection>>()
                .await
                .expect("Cannot connect to redis in request guard");
            let db = request
                .guard::<DbConnection>()
                .await
                .expect("Cannot connect to postgres in request guard");
            let result = cache
                .get::<_, i32>(format!("sessions/{}", header_value[1]))
                .await;
            if let Ok(user_id) = result {
                return match db.run(move |c| UserRepository::find(c, user_id)).await {
                    Ok(user) => Outcome::Success(user),
                    _ => Outcome::Failure((Status::Unauthorized, ())),
                };
            }
        }

        Outcome::Failure((Status::Unauthorized, ()))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for HtmlMailer {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Ok(tera) = tera::Tera::new("templates/**/*.html") {
            let smtp_host = std::env::var("SMTP_HOST").expect("Cannot load SMTP host from env");
            let smtp_username =
                std::env::var("SMTP_USERNAME").expect("Cannot load SMTP username from env");
            let smtp_password =
                std::env::var("SMTP_PASSWORD").expect("Cannot load SMTP password from env");

            let credentials = Credentials::new(smtp_username, smtp_password);
            let mailer = HtmlMailer {
                smtp_host,
                credentials,
                template_engine: tera,
            };
            return Outcome::Success(mailer);
        }
        Outcome::Failure((Status::InternalServerError, ()))
    }
}
