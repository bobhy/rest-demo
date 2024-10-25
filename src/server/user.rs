//! rest path /db/users
//! (from demo)
//todo: verify _RUD, then add real database.
use axum::{
    body::Body,
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, patch, post, Router},
    Json,
};

use serde::{Deserialize, Serialize};
/// A "user"
/// (includes key "id")
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct User {
    #[serde(default = "def_id")]
    id: u64,
    #[serde(default = "def_str")]
    name: String,
    #[serde(default = "def_str")]
    email: String,
}

fn def_id() -> u64 {
    42
}
fn def_str() -> String {
    "".into()
}

impl User {
    #[allow(unused)]
    pub fn new(id: u64, name: &str, email: &str) -> Self {
        Self {
            id,
            name: name.into(),
            email: email.into(),
        }
    }
}
/// A user's attributes
/// (but not a full user)
//todo: figure out how to keep these fields in sync with [User]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct UserAttributes {
    #[serde(default = "def_str")]
    name: String,
    #[serde(default = "def_str")]
    email: String,
}
impl UserAttributes {
    #[allow(unused)]
    pub fn new(name: &str, email: &str) -> Self {
        Self {
            name: name.into(),
            email: email.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct UserTuple(Vec<User>);

/// Because real JSON requires a single root element...
#[derive(Serialize, Deserialize, Debug)]
pub struct UserList {
    users: Vec<User>,
}

pub fn sample_users() -> UserList {
    UserList {
        users: vec![
            User {
                id: 1,
                name: "Elijah".into(),
                email: "elijah@example.com".into(),
            },
            User {
                id: 2,
                name: "John".into(),
                email: "john@doe.com".into(),
            },
        ],
    }
}

pub fn sample_user_tuple() -> UserTuple {
    UserTuple(sample_users().users)
}

/// query string parameters for "/db/user"
#[derive(Serialize, Deserialize, Debug)]
pub struct UserQSParameters {
    #[serde(default = "def_str")]
    name_contains: String,
    #[serde(default = "def_str")]
    email_contains: String,
}
/// Add routes for this handler
/// get db/user ? name=<contains> & (implicit OR) email=<contains>
/// -- query list of users
/// get db/user/<id>
/// -- query single user
/// post/patch db/user <form>
/// -- create or update single user
///
/// (demo ones)
/// post db/user/create-user <any>
/// get db/user/list-users
pub fn add_routes(app: Router) -> Router {
    app.route("/db/user/create-user", post(create_user_demo))
        .route("/db/user/list-users", get(list_users))
        .route("/db/user", get(filter_users))
        .route("/db/user", post(create_user))
        .route("/db/user/:id", get(read_user))
        .route("/db/user/:id", patch(update_user))
        .route("/db/user/:id", delete(delete_user))
}

/// _RUD handlers for "/db/user/:id"
pub async fn read_user(Path(id): Path<u64>) -> Json<User> {
    Json(User {
        id,
        ..Default::default()
    })
}

pub async fn update_user(Path(id): Path<u64>, Json(user): Json<User>) -> Json<User> {
    Json(User { id, ..user })
}

pub async fn delete_user(Path(id): Path<u64>) -> Json<u64> {
    Json(id)
}

/// C___ handler for post("/db/user")
///
/// Important that the return value includes the new `id`
pub async fn create_user(Json(user): Json<UserAttributes>) -> impl IntoResponse {
    (
        StatusCode::CREATED,
        Json(User {
            id: 42,
            name: user.name,
            email: user.email,
        }),
    )
}

/// list / filter handler for get("/db/user")

pub async fn filter_users(Query(qs): Query<UserQSParameters>) -> impl IntoResponse {
    let in_users = sample_users().users;
    let out_users: Vec<User> = in_users
        .iter()
        .filter(|user| {
            user.name.contains(&qs.name_contains) && user.email.contains(&qs.email_contains)
        })
        //todo: map wants "copy", but had to do element-by-element clone??
        .map(|s| User {
            id: s.id,
            name: s.name.clone(),
            email: s.email.clone(),
        })
        .collect();

    (StatusCode::OK, Json(out_users))
}

// Handler for /create-user
pub async fn create_user_demo() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("User created successfully"))
        .unwrap()
}
// Handler for /users
pub async fn list_users() -> Json<UserList> {
    Json(sample_users())
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

    #[fixture]
    async fn gen_test_client() -> TestClient {
        let test_routes = super::add_routes(Router::new());
        TestClient::new(test_routes).await
    }

    #[rstest]
    #[case("/db/user/create-user", &json!({"any": "any"}), "User created successfully") ]
    #[case("/db/user/create-user", &json!({"foo": "bar"}), "successfully") ]
    #[tokio::test]
    #[awt]
    async fn create_user_demo(
        #[case] url: &str,
        #[case] post_body: &Value,
        #[case] expected: &str,
    ) {
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
    #[case("/db/user/list-users?foo=bar&bas=gronk", &vec!("Elijah", "John"))]
    #[tokio::test]
    async fn list_users(#[case] url: &str, #[case] expected: &[&str]) {
        let test_client = gen_test_client().await;

        let response = test_client.get(url).send().await;

        assert2!(response.status() == StatusCode::OK);
        let rt = response.json::<UserList>().await.users;
        assert2!(rt.len() > 0, "response not long enough");
        check!(rt[0].name == expected[0]);
        check!(rt[1].name == expected[1]);
    }

    #[rstest]
    #[case("/db/user", &UserAttributes::new("foo", "bar"), &User::new(42,"foo", "bar")) ]
    #[tokio::test]
    async fn create_user(
        #[case] url: &str,
        #[case] post_body: &UserAttributes,
        #[case] expected: &User,
    ) {
        let test_client = gen_test_client().await;

        let response = test_client.post(url).json(&post_body).send().await;

        check!(response.status() == StatusCode::CREATED);
        let rt = response.json::<User>().await;

        assert2!(rt == *expected);
    }

    #[rstest]
    #[case::no_filters("/db/user", &sample_user_tuple())]
    #[case::noop_filter("/db/user?email_contains=", &sample_user_tuple())]
    #[case::noop_filter_name("/db/user?name_contains", &sample_user_tuple())]
    #[should_panic]
    #[case::filter_excludes_all("/db/user?email_contains=aa", &sample_user_tuple())]
    #[case::filter_selects_name("/db/user?name_contains=Jo", & UserTuple(vec!(User {
        id: 2,
        name: "John".into(),
        email: "john@doe.com".into(),
    })))]
    #[tokio::test]
    async fn filter_users(#[case] url: &str, #[case] expected: &UserTuple) {
        let test_client = gen_test_client().await;

        let response = test_client.get(url).send().await;

        check!(response.status() == StatusCode::OK);
        let rt = response.json::<UserTuple>().await;

        assert2!(rt == *expected);
    }

    #[test]
    fn test_contains() {
        check!("abc".contains("b"));
        check!("abc".contains("bc"));
        check!("abc".contains(""));
    }
}
