//! rest path /db/users
//! (from demo)
//! todo next up: parameterize gets, parameterize posts, generate errors

use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post, Router},
    Json,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    id: u64,
    name: String,
    email: String,
}

/// Add routes for this handler
pub fn add_routes(app: Router) -> Router {
    app.route("/db/user/create-user", post(create_user))
        .route("/db/user/list-users", get(list_users))
}

// Handler for /create-user
pub async fn create_user() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("User created successfully"))
        .unwrap()
}
// Handler for /users
pub async fn list_users() -> Json<Vec<User>> {
    let users = vec![
        User {
            id: 1,
            name: "Elijah".to_string(),
            email: "elijah@example.com".to_string(),
        },
        User {
            id: 2,
            name: "John".to_string(),
            email: "john@doe.com".to_string(),
        },
    ];
    Json(users)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::assert as assert2;
    use assert2::check;
    use axum::Router;
    use axum_test_helper::TestClient;
    use serde_json::json;
    use serde_json::Value;

    use rstest::*;

    async fn gen_test_client() -> TestClient {
        let test_routes = super::add_routes(Router::new());
        TestClient::new(test_routes).await
    }

    #[rstest]
    #[case("/db/user/create-user", &json!({"any": "any"}), "User created successfully") ]
    #[case("/db/user/create-user", &json!({"foo": "bar"}), "unsuccessfully") ]
    #[tokio::test]
    async fn create_user(#[case] url: &str, #[case] post_body: &Value, #[case] expected: &str) {
        let test_client = gen_test_client().await;

        let response = test_client.post(url).json(post_body).send().await;

        assert_eq!(response.status(), StatusCode::CREATED);
        let rt = response.text().await;

        assert2!(
            rt.contains(expected),
            "response doesn't contain expected <{}>",
            expected
        );
    }

    #[rstest]
    #[case("/db/user/list-users", &vec!("Elijah", "John"))]
    #[case("/db/user/list-users?foo=bar&bas=gronk", &vec!("Elijahx", "John"))]
    #[tokio::test]
    async fn list_users(#[case] url: &str, #[case] expected: &[&str]) {
        let test_client = gen_test_client().await;

        let response = test_client.get(url).send().await;

        assert2!(response.status() == StatusCode::OK);
        let rt: Vec<User> = response.json().await;
        assert2!(rt.len() > 0, "response not long enough");
        check!(rt[0].name == expected[0]);
        check!(rt[1].name == expected[1]);
    }
}
