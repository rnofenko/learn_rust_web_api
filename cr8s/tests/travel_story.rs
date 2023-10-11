use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{serde_json::json, Value};

#[test]
fn test_reservation_and_feedback() {
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
