use std::process::{Command, Output};

use reqwest::{
    blocking::{Client, ClientBuilder},
    header::{self, HeaderMap, HeaderValue},
    StatusCode,
};
use serde_json::{json, Value};

pub const APP_HOST: &'static str = "http://127.0.0.1:8000";

pub fn create_test_rustacean(client: &Client) -> Value {
    let response = client
        .post(format!("{}/rustaceans", APP_HOST))
        .json(&json!({
            "name":"John",
            "email":"j.doe@gmail.com"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    response.json().unwrap()
}

pub fn create_test_crate(client: &Client, rustacean: &Value) -> Value {
    let response = client
        .post(format!("{}/crates", APP_HOST))
        .json(&json!({
            "rustacean_id": rustacean["id"],
            "code": "foo",
            "name":"Foo crate",
            "version":"0.1.0",
            "description":"Foo crate description"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    response.json().unwrap()
}

pub fn delete_test_rustacean(client: &Client, rustacean: Value) {
    let response = client
        .delete(format!("{}/rustaceans/{}", APP_HOST, rustacean["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

pub fn delete_test_crate(client: &Client, a_crate: Value) {
    let response = client
        .delete(format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

pub fn create_test_user(username: &str, password: &str) -> Output {
    Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg(username)
        .arg(password)
        .arg("admin")
        .output()
        .unwrap()
}

pub fn delete_test_user(create_stdout: String) {
    let prefix = "User { id: ";
    let suffix = ", username:";
    let start_bytes = create_stdout.find(prefix).unwrap_or(0) + prefix.len();
    let end_bytes = create_stdout.find(suffix).unwrap_or(create_stdout.len());
    let user_id = &create_stdout[start_bytes..end_bytes];

    let _ = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("delete")
        .arg(user_id)
        .status();
}

pub fn get_logged_in_client(username: &str, role: &str) -> Client {
    let password = "1234";
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg(username)
        .arg(password)
        .arg(role)
        .output()
        .unwrap();

    println!("{:?}", output);

    let client = Client::new();
    let response = client
        .post(format!("{}/login", APP_HOST))
        .json(&json!({
            "username": username,
            "password": password
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.get("token").is_some());
    let auth_header = format!("Bearer {}", json["token"].as_str().unwrap());

    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(&auth_header).unwrap(),
    );

    ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap()
}

pub fn get_client_with_logged_in_viewer() -> Client {
    get_logged_in_client("test_viewer", "viewer")
}

pub fn get_client_with_logged_in_editor() -> Client {
    get_logged_in_client("test_editor", "editor")
}
