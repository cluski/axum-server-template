use axum::{
    Router,
    body::Body,
    extract::DefaultBodyLimit,
    http::{HeaderName, Method, Request},
    routing::get,
};
use tower::ServiceBuilder;
use tower_http::{
    cors::{self, CorsLayer},
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use tracing::{error, info, info_span};

const REQUEST_ID_HEADER: &str = "x-request-id";

pub fn get_router() -> Router {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::PUT,
        ])
        .allow_origin(cors::Any)
        .allow_headers(cors::Any);

    let x_request_id = HeaderName::from_static(REQUEST_ID_HEADER);

    let request_id_middleware = ServiceBuilder::new()
        .layer(SetRequestIdLayer::new(
            x_request_id.clone(),
            MakeRequestUuid,
        ))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                // Log the request id as generated.
                let request_id = request.headers().get(REQUEST_ID_HEADER);

                match request_id {
                    Some(request_id) => info_span!(
                        "http_request",
                        request_id = ?request_id,
                    ),
                    None => {
                        error!("could not extract request_id");
                        info_span!("http_request")
                    }
                }
            }),
        )
        // send headers from request to response headers
        .layer(PropagateRequestIdLayer::new(x_request_id));

    Router::new()
        .route(
            "/hello",
            get(async || {
                info!("hello api called");
                "Hello, World!"
            }),
        )
        .layer(DefaultBodyLimit::max(100 * 1024 * 1024)) // 100MB 限制
        .layer(cors)
        .layer(request_id_middleware)
}
