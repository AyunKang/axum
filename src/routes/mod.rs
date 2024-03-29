mod always_error;
mod custom_json_extractor;
mod get_json;
mod hello_world;
mod middleware_message;
mod mirror_body_json;
mod mirror_body_string;
mod mirror_custom_header;
mod mirror_user_agent;
mod path_variable;
mod query_params;
mod read_middleware_custom_header;
mod returns_201;
mod set_middleware_custom_header;
mod validate_with_serde;

use always_error::always_error;
use axum::{
    http::Method,
    middleware,
    routing::{get, patch, post},
    Extension, Router,
};
use custom_json_extractor::custom_json_extractor;
use get_json::get_json;
use hello_world::hello_world;
use middleware_message::middleware_message;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use mirror_custom_header::mirror_custom_header;
use mirror_user_agent::mirror_user_agent;
use path_variable::path_variable;
use query_params::query_params;
use read_middleware_custom_header::read_middleware_custom_header;
use returns_201::returns_201;
use set_middleware_custom_header::set_middleware_custom_header;
use tower_http::cors::{Any, CorsLayer};
use validate_with_serde::validate_with_serde;

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_router() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let shared_data = SharedData {
        message: "Hi I am Lucy Kang.".to_owned(),
    };

    Router::new()
        .route(
            "/read_middleware_custom_header",
            get(read_middleware_custom_header),
        )
        .route_layer(middleware::from_fn(set_middleware_custom_header))
        .route("/", patch(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variable/:id", get(path_variable))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .route("/always_error", get(always_error))
        .route("/returns_201", post(returns_201))
        .route("/get_json", get(get_json))
        .route("/validate_data", post(validate_with_serde))
        .route("/custom_json_extractor", post(custom_json_extractor))
        .layer(Extension(shared_data))
        .layer(cors)
}
