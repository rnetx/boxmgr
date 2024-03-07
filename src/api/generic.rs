use std::sync::Arc;

use axum::{
    body::Body,
    extract::{FromRequest, Path, Request},
    http::{Response, StatusCode},
    response::IntoResponse,
    Json, RequestExt,
};

use crate::{database, manager};

#[derive(serde::Serialize)]
pub(crate) struct ErrorResponse {
    #[serde(skip)]
    pub(crate) code: StatusCode,
    pub(crate) message: String,
}

impl ErrorResponse {
    pub(crate) fn new<S: Into<String>>(code: StatusCode, message: S) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response<Body> {
        let response_content = serde_json::json!(&self).to_string();
        let mut response = Response::new(Body::from(response_content));
        *response.status_mut() = self.code;
        response.headers_mut().insert(
            http::header::CONTENT_TYPE,
            http::HeaderValue::from_str("application/json").unwrap(),
        );
        response
    }
}

pub(crate) struct GenericResponse<T: Sized + serde::Serialize + Send> {
    pub(crate) code: StatusCode,
    pub(crate) data: T,
}

impl<T: Sized + serde::Serialize + Send> serde::Serialize for GenericResponse<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        serde::ser::SerializeMap::serialize_entry(&mut map, "data", &self.data)?;
        serde::ser::SerializeMap::end(map)
    }
}

impl<T: Sized + serde::Serialize + Send> GenericResponse<T> {
    pub(crate) fn new(code: StatusCode, data: T) -> Self {
        Self { code, data }
    }
}

impl<T: Sized + serde::Serialize + Send> IntoResponse for GenericResponse<T> {
    fn into_response(self) -> Response<Body> {
        let response_content = serde_json::json!(&self).to_string();
        let mut response = Response::new(Body::from(response_content));
        *response.status_mut() = self.code;
        response.headers_mut().insert(
            http::header::CONTENT_TYPE,
            http::HeaderValue::from_str("application/json").unwrap(),
        );
        response
    }
}

pub(crate) struct RequestRawBodyContext<P = ()>
where
    P: serde::de::DeserializeOwned + Send,
{
    pub(crate) manager: Arc<manager::Manager>,
    pub(crate) req: Request<Body>,
    pub(crate) path_params: Option<Path<P>>,
}

#[async_trait::async_trait]
impl<P> FromRequest<Arc<manager::Manager>> for RequestRawBodyContext<P>
where
    P: serde::de::DeserializeOwned + Send + 'static,
{
    type Rejection = ();

    async fn from_request(
        mut req: Request,
        state: &Arc<manager::Manager>,
    ) -> Result<Self, Self::Rejection> {
        let path_params = req.extract_parts::<Path<P>>().await.ok();
        Ok(Self {
            manager: state.clone(),
            req,
            path_params,
        })
    }
}

pub(crate) struct RequestJsonContext<P, B>
where
    P: serde::de::DeserializeOwned + Send,
    B: serde::de::DeserializeOwned,
{
    pub(crate) manager: Arc<manager::Manager>,
    pub(crate) path_params: Option<Path<P>>,
    pub(crate) body: Json<B>,
}

#[async_trait::async_trait]
impl<P, B> FromRequest<Arc<manager::Manager>> for RequestJsonContext<P, B>
where
    P: serde::de::DeserializeOwned + Send + 'static,
    B: serde::de::DeserializeOwned,
{
    type Rejection = ErrorResponse;

    async fn from_request(
        mut req: Request,
        state: &Arc<manager::Manager>,
    ) -> Result<Self, Self::Rejection> {
        let path_params = req.extract_parts::<Path<P>>().await.ok();
        //
        let json_body = Json::<B>::from_request(req, state)
            .await
            .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e.to_string()))?;
        Ok(Self {
            manager: state.clone(),
            path_params,
            body: json_body,
        })
    }
}

pub(crate) struct RequestMultiPartContext {
    pub(crate) multipart: axum::extract::Multipart,
    pub(crate) manager: Arc<manager::Manager>,
}

#[async_trait::async_trait]
impl FromRequest<Arc<manager::Manager>> for RequestMultiPartContext {
    type Rejection = ErrorResponse;

    async fn from_request(
        req: Request,
        state: &Arc<manager::Manager>,
    ) -> Result<Self, Self::Rejection> {
        let multipart = axum::extract::Multipart::from_request(req, state)
            .await
            .map_err(|e| ErrorResponse::new(StatusCode::BAD_REQUEST, e.to_string()))?;
        Ok(Self {
            multipart,
            manager: state.clone(),
        })
    }
}

pub(crate) fn db_error_to_http_response(e: database::Error) -> impl IntoResponse {
    match &e {
        database::Error::DBError(e) => {
            ErrorResponse::new(StatusCode::BAD_GATEWAY, e.to_string()).into_response()
        }
        _ => ErrorResponse::new(StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}
