use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;

use crate::services::auth::AuthError;
use crate::services::cloudinary::CloudinaryError;
use crate::services::ingrediente::IngredienteError;
use crate::services::receta::RecetaError;

#[derive(Debug)]
pub enum ApiError {
    LoginIncorrecto,
    NoAutorizado,
    NoEncontrado,
    UsuarioYaExiste,
    SolicitudInvalida(String),
    ErrorDelServidor(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.mensaje())
    }
}

impl ApiError {
    fn mensaje(&self) -> String {
        match self {
            ApiError::LoginIncorrecto => "credenciales inválidas".into(),
            ApiError::NoAutorizado => "no autorizado".into(),
            ApiError::NoEncontrado => "recurso no encontrado".into(),
            ApiError::UsuarioYaExiste => "el usuario ya existe".into(),
            ApiError::SolicitudInvalida(detalle) => detalle.clone(),
            ApiError::ErrorDelServidor(detalle) => detalle.clone(),
        }
    }

    fn codigo_http(&self) -> StatusCode {
        match self {
            ApiError::LoginIncorrecto => StatusCode::UNAUTHORIZED, // 401
            ApiError::NoAutorizado => StatusCode::UNAUTHORIZED,
            ApiError::NoEncontrado => StatusCode::NOT_FOUND,
            ApiError::UsuarioYaExiste => StatusCode::CONFLICT, // 409
            ApiError::SolicitudInvalida(_) => StatusCode::BAD_REQUEST, // 400
            ApiError::ErrorDelServidor(_) => StatusCode::INTERNAL_SERVER_ERROR, // 500
        }
    }
}

impl From<AuthError> for ApiError {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::InvalidCredentials => ApiError::LoginIncorrecto,
            AuthError::Unauthorized => ApiError::NoAutorizado,
            AuthError::NotFound => ApiError::NoEncontrado,
            AuthError::Conflict => ApiError::UsuarioYaExiste,
            AuthError::Database(e) => ApiError::ErrorDelServidor(e.to_string()),
            AuthError::PasswordHash(e) => ApiError::ErrorDelServidor(e.to_string()),
            AuthError::Token(e) => ApiError::ErrorDelServidor(e.to_string()),
        }
    }
}

impl From<IngredienteError> for ApiError {
    fn from(err: IngredienteError) -> Self {
        match err {
            IngredienteError::Database(e) => ApiError::ErrorDelServidor(e.to_string()),
        }
    }
}

impl From<RecetaError> for ApiError {
    fn from(err: RecetaError) -> Self {
        match err {
            RecetaError::InvalidRequest(msg) => ApiError::SolicitudInvalida(msg),
            RecetaError::NotFound => ApiError::NoEncontrado,
            RecetaError::Database(e) => ApiError::ErrorDelServidor(e.to_string()),
        }
    }
}

impl From<CloudinaryError> for ApiError {
    fn from(err: CloudinaryError) -> Self {
        ApiError::ErrorDelServidor(err.to_string())
    }
}

#[derive(Serialize)]
struct CuerpoError {
    error: String,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        self.codigo_http()
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.codigo_http()).json(CuerpoError {
            error: self.mensaje(),
        })
    }
}