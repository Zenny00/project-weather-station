pub mod server {
    use axum::Router;
    use tower_http::services::ServeDir;

    pub fn routes_static() -> Router {
        Router::new().nest_service(
            "/pages",
            ServeDir::new(std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/src/pages"),
        )
    }
}
