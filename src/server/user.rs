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

use serde::Serialize;

#[derive(Serialize)]
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
    use axum::routing::put;
    use axum::Json;
    use axum::Router;
    use axum_test::TestServer;
    use serde_json::json;
    use serde_json::Value;

    #[tokio::test]
    async fn create_user() {
        let mut test_app = Router::new();
        test_app = add_routes(test_app);

        //let server = TestServer::new(test_app)?;
        let server = TestServer::builder()
            .expect_success_by_default()
            .mock_transport()
            .build(test_app)
            .unwrap();

        let response = server
            .post("/db/user/create-user")
            .json(&json!({"any": "any"}))
            .await;

        response.assert_status_success();
        response.assert_status(StatusCode::CREATED);
        response.assert_text_contains("User created successfully");
    }
}
