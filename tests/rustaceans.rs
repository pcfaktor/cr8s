use std::ops::Add;

use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{json, Value};

use crate::common::{create_test_rustacean, delete_test_rustacean};

const BASE_URL: &str = "http://127.0.0.1:8000";

pub mod common;

#[test]
fn test_get_rustaceans() {
    let client = Client::new();
    let rustacean1 = create_test_rustacean(&client);
    let rustacean2 = create_test_rustacean(&client);

    let response = client
        .get(BASE_URL.to_string().add("/rustaceans"))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&rustacean1));
    assert!(json.as_array().unwrap().contains(&rustacean2));

    delete_test_rustacean(&client, rustacean1);
    delete_test_rustacean(&client, rustacean2);
}

#[test]
fn test_create_rustacean() {
    let client = Client::new();
    let response = client
        .post(BASE_URL.to_string().add("/rustaceans"))
        .json(&json!({
            "name":"John",
            "email":"j.doe@gmail.com"
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let rustacean: Value = response.json().unwrap();
    assert_eq!(
        rustacean,
        json!({
            "id": rustacean["id"],
            "name":"John",
            "email":"j.doe@gmail.com",
            "created_at": rustacean["created_at"]
        })
    );

    delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_view_rustacean() {
    let client = Client::new();
    let rustacean: Value = create_test_rustacean(&client);

    let response = client
        .get(format!("{}/rustaceans/{}", BASE_URL, rustacean["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let rustacean: Value = response.json().unwrap();

    assert_eq!(
        rustacean,
        json!({
            "id": rustacean["id"],
            "name":"John",
            "email":"j.doe@gmail.com",
            "created_at": rustacean["created_at"]
        })
    );

    delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_update_rustacean() {
    let client = Client::new();
    let rustacean: Value = create_test_rustacean(&client);

    let response = client
        .put(format!("{}/rustaceans/{}", BASE_URL, rustacean["id"]))
        .json(&json!({
            "name":"Gunrock",
            "email":"gunrock@gmail.com"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let rustacean: Value = response.json().unwrap();

    assert_eq!(
        rustacean,
        json!({
            "id": rustacean["id"],
            "name":"Gunrock",
            "email":"gunrock@gmail.com",
            "created_at": rustacean["created_at"]
        })
    );

    delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_delete_rustacean() {
    let client = Client::new();
    let rustacean: Value = create_test_rustacean(&client);

    let response = client
        .delete(format!("{}/rustaceans/{}", BASE_URL, rustacean["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
