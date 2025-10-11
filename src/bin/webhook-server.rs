use axum::{
    extract::Request, http::status::StatusCode, response::IntoResponse, routing::post, Router,
};
// use serde_json::Value;
async fn webhook_handler(request: Request) -> impl IntoResponse {
    println!("{request:?}");
    // let body: Value = request.body().into_data_stream().try_into();
    (StatusCode::ACCEPTED, "")
}

#[tokio::main]
async fn main() {
    let app = Router::<()>::new()
        .without_v07_checks()
        .route("/", post(webhook_handler))
        .route("/webhook", post(webhook_handler));

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
