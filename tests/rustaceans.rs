use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{json, Value};

use crate::common::{create_test_rustacean, delete_test_rustacean};

pub mod common;

#[test]
fn test_get_rustaceans() {
    let client = common::get_client_with_logged_in_user();
    let rustacean1 = create_test_rustacean(&client);
    let rustacean2 = create_test_rustacean(&client);

    let response = client
        .get(format!("{}/rustaceans", common::APP_HOST))
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
fn test_get_rustaceans_without_token() {
    let client = Client::new();
    let client_with_user = common::get_client_with_logged_in_user();
    let rustacean1 = create_test_rustacean(&client_with_user);
    let rustacean2 = create_test_rustacean(&client_with_user);

    let response = client
        .get(format!("{}/rustaceans", common::APP_HOST))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    delete_test_rustacean(&client_with_user, rustacean1);
    delete_test_rustacean(&client_with_user, rustacean2);
}

#[test]
fn test_create_rustacean() {
    let client = common::get_client_with_logged_in_user();
    let response = client
        .post(format!("{}/rustaceans", common::APP_HOST))
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
    let client = common::get_client_with_logged_in_user();
    let rustacean: Value = create_test_rustacean(&client);

    let response = client
        .get(format!(
            "{}/rustaceans/{}",
            common::APP_HOST,
            rustacean["id"]
        ))
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
fn test_view_rustacean_not_found() {
    let client = common::get_client_with_logged_in_user();

    let response = client
        .get(format!("{}/rustaceans/{}", common::APP_HOST, -1))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[test]
fn test_update_rustacean() {
    let client = common::get_client_with_logged_in_user();
    let rustacean: Value = create_test_rustacean(&client);

    let response = client
        .put(format!(
            "{}/rustaceans/{}",
            common::APP_HOST,
            rustacean["id"]
        ))
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
    let client = common::get_client_with_logged_in_user();
    let rustacean: Value = create_test_rustacean(&client);

    let response = client
        .delete(format!(
            "{}/rustaceans/{}",
            common::APP_HOST,
            rustacean["id"]
        ))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
