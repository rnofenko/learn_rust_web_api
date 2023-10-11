use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{serde_json::json, Value};

fn create_test_rustacean(client: &Client) -> Value {
    let create_response = client
        .post("http://127.0.0.1:8000/rustaceans")
        .json(&json!({"name": "test_name", "email": "test@mail.com"}))
        .send()
        .unwrap();
    assert_eq!(create_response.status(), StatusCode::CREATED);
    create_response.json().unwrap()
}

#[test]
fn test_get_rustaceans() {
    let client = Client::new();
    let response = client
        .get("http://127.0.0.1:8000/rustaceans")
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[test]
fn test_create_rustacean() {
    let client = Client::new();
    let response = client
        .post("http://127.0.0.1:8000/rustaceans")
        .json(&json!({"name": "test name1", "email": "test@mail.com"}))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let rustacean: Value = response.json().unwrap();
    assert_eq!(
        rustacean,
        json!({"id": rustacean["id"], "name": "test name1", "email": "test@mail.com", "created_at": rustacean["created_at"]})
    )
}

#[test]
fn test_view_rustacean() {
    let name = "test_name";
    let email = "test@mail.com";
    let client = Client::new();

    let rustacean: Value = create_test_rustacean(&client);
    let id = &rustacean["id"];
    let created_at = &rustacean["created_at"];

    let view_response = client
        .get(format!("http://127.0.0.1:8000/rustaceans/{}", id))
        .send()
        .unwrap();
    assert_eq!(view_response.status(), StatusCode::OK);
    let rustacean_view: Value = view_response.json().unwrap();

    assert_eq!(
        rustacean_view,
        json!({"id": id, "name": name, "email": email, "created_at": created_at})
    )
}

#[test]
fn test_update_rustacean() {
    let name_new = "test_name_3";
    let email = "test@mail.com";
    let client = Client::new();
    let rustacean: Value = create_test_rustacean(&client);
    let id = &rustacean["id"];
    let created_at = &rustacean["created_at"];

    let update_response = client
        .put(format!("http://127.0.0.1:8000/rustaceans/{}", id))
        .json(&json!({"name": name_new, "email": email}))
        .send()
        .unwrap();
    assert_eq!(update_response.status(), StatusCode::OK);
    let rustacean_updated: Value = update_response.json().unwrap();

    assert_eq!(
        rustacean_updated,
        json!({"id": id, "name": name_new, "email": email, "created_at": created_at})
    )
}

#[test]
fn test_delete_rustacean() {
    let client = Client::new();
    let rustacean: Value = create_test_rustacean(&client);
    let id = &rustacean["id"];

    let response = client
        .delete(format!("http://127.0.0.1:8000/rustaceans/{}", id))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
