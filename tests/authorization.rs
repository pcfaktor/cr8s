use reqwest::{blocking::Client, StatusCode};
use rocket::form::validate::Len;
use serde_json::{json, Value};
use std::process::Command;

pub mod common;

#[test]
fn test_login() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg("test_admin")
        .arg("1234")
        .arg("admin")
        .output()
        .unwrap();

    print!("{:?}", output);

    let client = Client::new();

    let response = client
        .post(format!("{}/login", common::APP_HOST))
        .json(&json!({
            "username":"test_admin",
            "password":"1234"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();
    assert!(json.get("token").is_some());
    assert_eq!(json["token"].as_str().len(), 128);

    let stdout = String::from_utf8(output.stdout).unwrap();
    let pattern = "User created: User { id: ";
    let start_bytes = stdout.find(pattern).unwrap_or(0) + pattern.len();
    let end_bytes = stdout.find(", username: ").unwrap_or(stdout.len());
    let user_id = &stdout[start_bytes..end_bytes];
    println!("user_id:{}", user_id);

    let _ = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("delete")
        .arg(user_id)
        .status();
}
