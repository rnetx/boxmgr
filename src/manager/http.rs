use std::{error::Error, future::Future, net::SocketAddr, pin::Pin, sync::Arc};

use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    response::Response,
    routing::{delete, get, patch, post, put},
    Router,
};
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;
use tower_http::{
    auth::{AsyncAuthorizeRequest, AsyncRequireAuthorizationLayer},
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
};

use crate::api;

pub(crate) struct HTTPServer {
    router: Router<()>,
    pub(crate) listen: SocketAddr,
}

impl HTTPServer {
    pub(crate) fn new(manager: Arc<super::Manager>, listen: SocketAddr, secret: String) -> Self {
        let mut api_router = Router::new();
        // API
        api_router = api_router
            .merge(Self::config_router())
            .merge(Self::kv_router())
            .merge(Self::script_router())
            .merge(Self::service_router());
        api_router = api_router.layer(AsyncRequireAuthorizationLayer::new(AuthMiddleware {
            secret,
        }));
        // Request Body Limit
        // 128 MB
        api_router = api_router.layer(RequestBodyLimitLayer::new(128 * 1024 * 1024));
        // Cors
        api_router = Self::cors(api_router);
        //
        let api_router = api_router.with_state::<()>(manager);
        //
        let mut router = Router::new();
        let assets = axum_embed::ServeEmbed::<Assets>::new();
        router = router.nest_service("/", assets);
        router = router.nest_service("/api/v1", api_router);
        //
        Self { router, listen }
    }

    pub(crate) async fn run(
        self,
        cancel_token: CancellationToken,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let tcp_listener = TcpListener::bind(self.listen).await?;
        axum::serve(tcp_listener, self.router)
            .with_graceful_shutdown(async move { cancel_token.cancelled().await })
            .await
            .map_err(|e| format!("{}", e).into())
    }

    fn config_router() -> Router<Arc<super::Manager>> {
        Router::new()
            .route("/config", post(api::config::add_config))
            .route("/config/:id", get(api::config::get_config))
            .route("/config/:id", patch(api::config::modify_config))
            .route("/config/:id", delete(api::config::delete_config))
            .route("/bluk_config_delete", post(api::config::bulk_delete_config))
            .route("/config", get(api::config::list_config))
            .route("/active_config/:id", put(api::config::set_active_config))
            .route("/active_config", get(api::config::get_active_config))
    }

    fn kv_router() -> Router<Arc<super::Manager>> {
        Router::new()
            .route("/kv/:key", post(api::kv::set_kv))
            .route("/kv/:key", get(api::kv::get_kv))
            .route("/kv/:key", delete(api::kv::delete_kv))
            .route("/kv", get(api::kv::list_kv))
    }

    fn script_router() -> Router<Arc<super::Manager>> {
        Router::new()
            .route("/script", post(api::script::add_script))
            .route("/script/:id", get(api::script::get_script))
            .route("/script/:id", patch(api::script::modify_script))
            .route("/script/:id", delete(api::script::delete_script))
            .route("/bluk_script_delete", post(api::script::bulk_delete_script))
            .route("/script", get(api::script::list_script))
            .route(
                "/script_run_type/:id",
                delete(api::script::clean_script_run_type),
            )
            .route(
                "/before_start_script",
                get(api::script::get_before_start_script),
            )
            .route(
                "/before_start_script/:id",
                put(api::script::set_before_start_script),
            )
            .route(
                "/after_start_script",
                get(api::script::get_after_start_script),
            )
            .route(
                "/after_start_script/:id",
                put(api::script::set_after_start_script),
            )
            .route(
                "/before_close_script",
                get(api::script::get_before_close_script),
            )
            .route(
                "/before_close_script/:id",
                put(api::script::set_before_close_script),
            )
            .route(
                "/after_close_script",
                get(api::script::get_after_close_script),
            )
            .route(
                "/after_close_script/:id",
                put(api::script::set_after_close_script),
            )
    }

    fn service_router() -> Router<Arc<super::Manager>> {
        Router::new()
            .route("/service/start", get(api::service::start_service))
            .route("/service/stop", get(api::service::stop_service))
            .route("/service/restart", get(api::service::restart_service))
            .route("/service/core_path", put(api::service::set_core_path))
            .route("/service/core_path", get(api::service::get_core_path))
            .route("/service/core_path", post(api::service::upload_core_path))
            .route("/service/config", get(api::service::get_config))
            .route("/service/auto_start", get(api::service::get_auto_start))
            .route("/service/auto_start", put(api::service::set_auto_start))
            .route("/service/status", get(api::service::get_status))
            .route("/service/log", get(api::service::get_log))
    }

    fn cors<T: Clone + Send + Sync + 'static>(router: Router<T>) -> Router<T> {
        router.layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_private_network(true)
                .allow_methods([
                    http::Method::GET,
                    http::Method::POST,
                    http::Method::PATCH,
                    http::Method::PUT,
                    http::Method::DELETE,
                    http::Method::OPTIONS,
                ])
                .allow_headers(Any),
        )
    }
}

#[derive(rust_embed::RustEmbed, Clone)]
#[folder = "ui/dist/"]
struct Assets;

#[derive(Clone)]
pub(crate) struct AuthMiddleware {
    secret: String,
}

impl AsyncAuthorizeRequest<Body> for AuthMiddleware {
    type RequestBody = Body;
    type ResponseBody = Body;
    type Future = Pin<
        Box<
            dyn Future<
                    Output = Result<Request<Self::RequestBody>, http::Response<Self::ResponseBody>>,
                > + Send,
        >,
    >;

    fn authorize(&mut self, request: http::Request<Body>) -> Self::Future {
        let secret = self.secret.clone();
        Box::pin(async move {
            // Check Secret: GET /check_secret (header: Authorization: Bearer <secret>)
            const CHECK_SECRET_PATH: &str = "/check_secret";
            if request.uri().path() == CHECK_SECRET_PATH {
                let mut response = Response::new(Body::empty());
                if let Some(v) = request.headers().get(http::header::AUTHORIZATION) {
                    if let Ok(s) = String::from_utf8(v.as_bytes().to_vec()) {
                        if s.as_str().trim_start_matches("Bearer ") == secret.as_str() {
                            *response.status_mut() = StatusCode::OK;
                            return Err(response);
                        }
                    }
                }
                *response.status_mut() = StatusCode::UNAUTHORIZED;
                return Err(response);
            }
            //
            let f = || {
                let mut response = Response::new(Body::empty());
                *response.status_mut() = StatusCode::UNAUTHORIZED;
                response
            };
            // Websocket
            if let Some(protocol) = request.headers().get("Upgrade") {
                if protocol == "websocket" {
                    let query = request.uri().query().unwrap_or("");
                    let querys = query
                        .split('&')
                        .map(|s| {
                            let (k, v) = s.split_once('=').unwrap_or(("", ""));
                            (k.to_string(), v.to_string())
                        })
                        .collect::<Vec<(String, String)>>();
                    for (k, v) in querys {
                        if k == "secret" {
                            if v == secret.as_str() {
                                return Ok(request);
                            }
                        }
                    }
                    return Err(f());
                }
            }
            //
            match request.headers().get(http::header::AUTHORIZATION) {
                Some(v) => {
                    let s = String::from_utf8(v.as_bytes().to_vec()).map_err(|_| f())?;
                    if s.trim_start_matches("Bearer ") != secret.as_str() {
                        return Err(f());
                    }
                }
                None => return Err(f()),
            }
            Ok(request)
        })
    }
}
