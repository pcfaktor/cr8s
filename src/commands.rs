use diesel::{Connection, PgConnection};
use std::str::FromStr;

use crate::auth;
use crate::models::{NewUser, RoleCode};
use crate::repositories::{RoleRepository, UserRepository};

pub fn load_db_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("Cannot read DB url from env");
    PgConnection::establish(&database_url).expect("Cannot connect to postgres")
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
