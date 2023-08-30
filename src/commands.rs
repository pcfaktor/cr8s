use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::PasswordHasher;
use diesel::{Connection, PgConnection};

use crate::models::NewUser;
use crate::repositories::{RoleRepository, UserRepository};

fn load_db_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("Cannot read DB url from env");
    PgConnection::establish(&database_url).expect("Cannot connect to postgres")
}

pub fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut connection = load_db_connection();

    let salt = SaltString::generate(OsRng);
    let argon = argon2::Argon2::default();
    let password_hash = argon.hash_password(password.as_bytes(), &salt).unwrap();

    let new_user = NewUser {
        username,
        password: password_hash.to_string(),
    };
    let user = UserRepository::create(&mut connection, new_user, role_codes).unwrap();
    println!("User created: {:?}", user);
    let roles = RoleRepository::find_by_user(&mut connection, &user).unwrap();
    println!("Role assigned: {:?}", roles);
}

pub fn list_users() {}

pub fn delete_user(_id: i32) {}
