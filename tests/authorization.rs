use common::get_client_with_logged_in_viewer;
use reqwest::{blocking::Client, StatusCode};
use rocket::form::validate::Len;
use serde_json::{json, Value};

use crate::common::{create_test_user, delete_test_user};

pub mod common;

#[test]
fn test_login_success() {
    let username = format!("test_user{}", rand::random::<u32>());
    let password = "1234";
    let output = create_test_user(&username, password);

    print!("{:?}", output);

    let client = Client::new();

    let response = client
        .post(format!("{}/login", common::APP_HOST))
        .json(&json!({
            "username":username,
            "password":password
        }))
        .send()
        .unwrap();

    // Cleanup
    let stdout = String::from_utf8(output.stdout).unwrap();
    delete_test_user(stdout);

    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();
    assert!(json.get("token").is_some());
    assert_eq!(json["token"].as_str().len(), 128);
}

#[test]
fn test_login_wrong_password() {
    let username = format!("test_user{}", rand::random::<u32>());
    let output = create_test_user(&username, "1234");

    print!("{:?}", output);

    let client = Client::new();

    let response = client
        .post(format!("{}/login", common::APP_HOST))
        .json(&json!({
            "username":username,
            "password":"wrong_password"
        }))
        .send()
        .unwrap();

    // Cleanup
    let stdout = String::from_utf8(output.stdout).unwrap();
    delete_test_user(stdout);

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[test]
fn test_login_wrong_username() {
    let username = format!("test_user{}", rand::random::<u32>());
    let password = "1234";
    let output = create_test_user(&username, password);

    print!("{:?}", output);

    let client = Client::new();

    let response = client
        .post(format!("{}/login", common::APP_HOST))
        .json(&json!({
            "username":"wrong_username",
            "password":password
        }))
        .send()
        .unwrap();

    // Cleanup
    let stdout = String::from_utf8(output.stdout).unwrap();
    delete_test_user(stdout);

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[test]
fn test_me() {
    let client = get_client_with_logged_in_viewer();

    let response = client
        .get(format!("{}/me", common::APP_HOST))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.get("id").is_some());
    assert!(json.get("username").is_some());
    assert_eq!(json["username"], "test_viewer");
    assert!(json.get("created_at").is_some());
    assert!(json.get("password").is_none());
}
