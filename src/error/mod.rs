use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;

use crate::services::auth::AuthError;
use crate::services::cloudinary::CloudinaryError;
use crate::services::favorito::FavoritoError;
use crate::services::grupo::GrupoError;
use crate::services::ingrediente::IngredienteError;
use crate::services::logro::LogroError;
use crate::services::puntuacion::PuntuacionError;
use crate::services::receta::RecetaError;
use crate::services::utensilio::UtensilioError;

#[derive(Debug)]
pub enum ApiError {
    LoginIncorrecto,
    NoAutorizado,
    Prohibido,
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
            ApiError::Prohibido => "no tienes permiso para esta acción".into(),
            ApiError::NoEncontrado => "recurso no encontrado".into(),
            ApiError::UsuarioYaExiste => "el usuario ya existe".into(),
            ApiError::SolicitudInvalida(detalle) => detalle.clone(),
            ApiError::ErrorDelServidor(detalle) => detalle.clone(),
        }
    }

    fn codigo_http(&self) -> StatusCode {
        match self {
            ApiError::LoginIncorrecto => StatusCode::UNAUTHORIZED,
            ApiError::NoAutorizado => StatusCode::UNAUTHORIZED,
            ApiError::Prohibido => StatusCode::FORBIDDEN,
            ApiError::NoEncontrado => StatusCode::NOT_FOUND,
            ApiError::UsuarioYaExiste => StatusCode::CONFLICT,
            ApiError::SolicitudInvalida(_) => StatusCode::BAD_REQUEST,
            ApiError::ErrorDelServidor(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
            AuthError::Forbidden => ApiError::Prohibido,
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

impl From<UtensilioError> for ApiError {
    fn from(err: UtensilioError) -> Self {
        match err {
            UtensilioError::Database(e) => ApiError::ErrorDelServidor(e.to_string()),
        }
    }
}

impl From<RecetaError> for ApiError {
    fn from(err: RecetaError) -> Self {
        match err {
            RecetaError::InvalidRequest(msg) => ApiError::SolicitudInvalida(msg),
            RecetaError::Forbidden => ApiError::Prohibido,
            RecetaError::NotFound => ApiError::NoEncontrado,
            RecetaError::Database(e) => ApiError::ErrorDelServidor(e.to_string()),
        }
    }
}

impl From<FavoritoError> for ApiError {
    fn from(err: FavoritoError) -> Self {
        match err {
            FavoritoError::NotFound => ApiError::NoEncontrado,
            FavoritoError::InvalidRequest(msg) => ApiError::SolicitudInvalida(msg),
            FavoritoError::Database(e) => ApiError::ErrorDelServidor(e.to_string()),
        }
    }
}

impl From<PuntuacionError> for ApiError {
    fn from(err: PuntuacionError) -> Self {
        match err {
            PuntuacionError::NotFound => ApiError::NoEncontrado,
            PuntuacionError::InvalidRequest(msg) => ApiError::SolicitudInvalida(msg),
            PuntuacionError::Database(e) => ApiError::ErrorDelServidor(e.to_string()),
        }
    }
}

impl From<GrupoError> for ApiError {
    fn from(err: GrupoError) -> Self {
        match err {
            GrupoError::NotFound => ApiError::NoEncontrado,
            GrupoError::Forbidden => ApiError::Prohibido,
            GrupoError::InvalidRequest(msg) => ApiError::SolicitudInvalida(msg),
            GrupoError::Database(e) => ApiError::ErrorDelServidor(e.to_string()),
        }
    }
}

impl From<LogroError> for ApiError {
    fn from(err: LogroError) -> Self {
        match err {
            LogroError::Database(e) => ApiError::ErrorDelServidor(e.to_string()),
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
