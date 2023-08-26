use common::{create_test_rustacean, APP_HOST};
use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

use crate::common::{create_test_crate, delete_test_crate, delete_test_rustacean};

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
    let rustacean = common::create_test_rustacean(&client);

    let response = client
        .post(format!("{}/crates", common::APP_HOST))
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

    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);
}
