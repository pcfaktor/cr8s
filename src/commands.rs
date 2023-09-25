use chrono::{Datelike, Utc};
use diesel::{Connection, PgConnection};
use lettre::transport::smtp::authentication::Credentials;
use std::str::FromStr;
use tera::{Context, Tera};

use crate::auth;
use crate::mail::HtmlMailer;
use crate::models::{NewUser, RoleCode};
use crate::repositories::{CrateRepository, RoleRepository, UserRepository};

pub fn load_db_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("Cannot read DB url from env");
    PgConnection::establish(&database_url).expect("Cannot connect to postgres")
}

fn load_template_engine() -> Tera {
    Tera::new("templates/**/*.html").unwrap_or_else(|e| {
        panic!("Parsing error(s): {}", e);
    })
}

pub fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut connection = load_db_connection();

    let password_hash = auth::hash_password(password).unwrap();
    let new_user = NewUser {
        username,
        password: password_hash.to_string(),
    };

    let role_codes = role_codes
        .iter()
        .map(|v| RoleCode::from_str(&v).unwrap())
        .collect();

    let user = UserRepository::create(&mut connection, new_user, role_codes).unwrap();
    println!("User created: {:?}", user);
    let roles = RoleRepository::find_by_user(&mut connection, &user).unwrap();
    for role in roles {
        println!("Role assigned: {:?}", role);
    }
}

pub fn list_users() {
    let mut connection = load_db_connection();

    let users = UserRepository::find_with_roles(&mut connection).unwrap();
    for user in users {
        println!("User: {:?}", user.0);
        println!("Roles:");
        for role in user.1.iter() {
            println!("\t{:?}", role.1);
        }
    }
}

pub fn delete_user(id: i32) {
    let mut connection = load_db_connection();

    UserRepository::delete(&mut connection, id).unwrap();
}

pub fn send_digest(to: Vec<String>, hours_since: i32, subject: Option<String>) {
    let mut connection = load_db_connection();

    let crates = CrateRepository::find_since(&mut connection, hours_since).unwrap();

    if crates.len() > 0 {
        println!("Sending digest for {} crates", crates.len());

        let mut context = Context::new();
        context.insert("crates", &crates);
        let year = Utc::now().year();
        context.insert("year", &year);

        let smtp_host = std::env::var("SMTP_HOST").expect("Cannot load SMTP host from env");
        let smtp_username =
            std::env::var("SMTP_USERNAME").expect("Cannot load SMTP username from env");
        let smtp_password =
            std::env::var("SMTP_PASSWORD").expect("Cannot load SMTP password from env");

        let credentials = Credentials::new(smtp_username, smtp_password);
        let tera = load_template_engine();

        let mailer = HtmlMailer {
            smtp_host,
            credentials,
            template_engine: tera,
        };

        mailer
            .send(to, subject, "email/digest.html", &context)
            .unwrap();
    }
}
