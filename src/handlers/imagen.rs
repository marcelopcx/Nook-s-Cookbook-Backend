//! Endpoint para subir imágenes de recetas a Cloudinary.
//!
//! El cliente (Expo) envía un `multipart/form-data` con un campo `file`
//! que contiene la imagen. El servidor reenvía esos bytes a Cloudinary
//! usando Unsigned Upload y devuelve la `secure_url` resultante para
//! que el cliente la persista luego en `Receta.imagen`.

use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse};
use futures_util::StreamExt;
use serde::Serialize;

use crate::auth::AuthenticatedUser;
use crate::config::AppConfig;
use crate::error::ApiError;
use crate::services::cloudinary;

/// Tamaño máximo aceptado por archivo (10 MB).
/// Cloudinary acepta hasta 10 MB para imágenes en cuentas gratuitas.
const MAX_BYTES: usize = 10 * 1024 * 1024;

#[derive(Serialize)]
struct ImagenSubidaResponse {
    secure_url: String,
}

#[post("/recetas/imagen")]
pub async fn subir_imagen_receta(
    config: web::Data<AppConfig>,
    _user: AuthenticatedUser,
    mut payload: Multipart,
) -> Result<HttpResponse, ApiError> {
    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|e| {
            ApiError::SolicitudInvalida(format!("multipart inválido: {e}"))
        })?;

        if field.name() != Some("file") {
            continue;
        }

        let filename = field
            .content_disposition()
            .and_then(|cd| cd.get_filename())
            .map(|name| name.to_string())
            .unwrap_or_else(|| "imagen.jpg".to_string());

        let mut bytes: Vec<u8> = Vec::new();
        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(|e| {
                ApiError::ErrorDelServidor(format!("error leyendo archivo: {e}"))
            })?;

            if bytes.len() + data.len() > MAX_BYTES {
                return Err(ApiError::SolicitudInvalida(
                    "la imagen supera el tamaño máximo permitido (10 MB)".into(),
                ));
            }

            bytes.extend_from_slice(&data);
        }

        if bytes.is_empty() {
            return Err(ApiError::SolicitudInvalida(
                "el archivo está vacío".into(),
            ));
        }

        let secure_url =
            cloudinary::subir_imagen(&config.cloudinary, bytes, filename).await?;

        return Ok(HttpResponse::Ok().json(ImagenSubidaResponse { secure_url }));
    }

    Err(ApiError::SolicitudInvalida(
        "no se envió ningún archivo en el campo `file`".into(),
    ))
}
