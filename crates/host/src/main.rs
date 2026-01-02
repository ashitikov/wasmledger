use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use wasmledger_sql::sqldb::{SqlDB, SqlDatabasePoolOption};
use wasmtime::{Engine, Store, component::Linker};

struct GlobalComponentState {
    sql: wasmledger_sql::CoreComponentState,
}

async fn setup_engine() {
    let engine = Engine::default();

    let pool_options = SqlDatabasePoolOption::default();
    let pool = pool_options.connect("").await.unwrap();

    let mut linker = Linker::new(&engine);
    let mut store = Store::new(
        &engine,
        GlobalComponentState {
            sql: wasmledger_sql::CoreComponentState::new(SqlDB::new(pool)),
        },
    );

    wasmledger_sql::CoreHost::add_to_linker(&mut linker, |s: &mut GlobalComponentState| s.sql)?;
}

#[tokio::main]
async fn main() {
    // initialize tracing
    // tracing_subscriber::fmt::init();

    setup_engine().await;

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    // tracing::debug!("listening on {}", listener.local_addr().unwrap());
    let _ = axum::serve(listener, app).await;
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
