use common::APP_HOST;
use common::{create_test_crate, create_test_rustacean, delete_test_crate, delete_test_rustacean};
use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;

#[test]
fn test_get_crates() {
    let client = Client::new();

    let rustacean = create_test_rustacean(&client);
    let crate1 = create_test_crate(&client, &rustacean);
    let crate2 = create_test_crate(&client, &rustacean);

    let response = client.get(format!("{}/crates", APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&crate1));
    assert!(json.as_array().unwrap().contains(&crate2));

    delete_test_crate(&client, crate1);
    delete_test_crate(&client, crate2);
    delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_create_crate() {
    let client = Client::new();
    let rustacean = create_test_rustacean(&client);

    let response = client
        .post(format!("{}/crates", APP_HOST))
        .json(&json!({
            "rustacean_id": rustacean["id"],
            "code": "foo",
            "name": "Foo crate",
            "version": "0.1",
            "description": "Foo crate description"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let a_crate: Value = response.json().unwrap();
    assert_eq!(
        a_crate,
        json!({
            "id": a_crate["id"],
            "code": "foo",
            "name": "Foo crate",
            "version": "0.1",
            "description": "Foo crate description",
            "rustacean_id": rustacean["id"],
            "created_at": a_crate["created_at"],
        })
    );

    delete_test_crate(&client, a_crate);
    delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_update_crate() {
    let client = Client::new();
    let rustacean = create_test_rustacean(&client);
    let rustacean2 = create_test_rustacean(&client);
    let a_crate = create_test_crate(&client, &rustacean);

    let response = client
        .put(format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .json(&json!({
            "code": "newcode",
            "name": "Crate new name",
            "version": "0.1.1",
            "description": "Lorem ipsum dolor sit amet consectetur adipiscing elit, sed do eiusmod
             tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis
             nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis
             aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla
             pariatur.",
            "rustacean_id": rustacean2["id"]
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let crate_from_response: Value = response.json().unwrap();
    assert_eq!(
        crate_from_response,
        json!({
            "id": a_crate["id"],
            "code": "newcode",
            "name": "Crate new name",
            "version": "0.1.1",
            "description": "Lorem ipsum dolor sit amet consectetur adipiscing elit, sed do eiusmod
             tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis
             nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis
             aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla
             pariatur.",
            "rustacean_id": rustacean2["id"],
            "created_at": a_crate["created_at"],
        })
    );

    delete_test_crate(&client, a_crate);
    delete_test_rustacean(&client, rustacean);
    delete_test_rustacean(&client, rustacean2);
}

#[test]
fn test_view_crate() {
    let client = Client::new();
    let rustacean = create_test_rustacean(&client);
    let a_crate = create_test_crate(&client, &rustacean);

    let response = client
        .get(&format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let crate_response: Value = response.json().unwrap();
    assert_eq!(a_crate, crate_response);

    delete_test_crate(&client, a_crate);
    delete_test_crate(&client, crate_response);
    delete_test_rustacean(&client, rustacean)
}

#[test]
fn test_view_crate_not_found() {
    let client = Client::new();

    let response = client
        .get(format!("{}/crates/{}", APP_HOST, -1))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[test]
fn test_delete_crate() {
    let client = Client::new();
    let rustacean = create_test_rustacean(&client);
    let a_crate = create_test_crate(&client, &rustacean);

    let response = client
        .delete(&format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    delete_test_crate(&client, a_crate);
    delete_test_rustacean(&client, rustacean);
}