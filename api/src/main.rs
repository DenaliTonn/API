use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router
};
use serde::{Deserialize, Serialize};

// Use Axum framework to make an API
//
//      /       http-GET
//      /classes
//      /classes http-GET
//      /classes http-POST
#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/classes", get(get_classes))
        .route("/classes", post(create_class));
        
    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    //tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_class(Json(payload): Json<CreateClass>,) -> (StatusCode, Json<Class>) {
    // insert your application logic here
    let class = Class {
        crn: 3550,
        name: payload.name,
    };
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(class))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateClass {
    name: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct Class {
    crn: u64,
    name: String,
}

async fn get_classes() -> &'static str {
    "class list"
}