//! Cliente para subir imágenes a Cloudinary usando el método
//! de subidas no firmadas (Unsigned Uploads).
//!
//! Endpoint: `https://api.cloudinary.com/v1_1/{cloud_name}/image/upload`
//!
//! La función `subir_imagen` recibe los bytes ya leídos (por ejemplo, desde
//! un campo `multipart` de Actix) y el nombre original del archivo, envía
//! el formulario al endpoint público de Cloudinary y devuelve la
//! `secure_url` para que el handler la persista en la base de datos.

use reqwest::multipart::{Form, Part};
use serde::Deserialize;

use crate::config::CloudinaryConfig;

#[derive(Debug, thiserror::Error)]
pub enum CloudinaryError {
    #[error("error de red al comunicarse con Cloudinary")]
    Http(#[from] reqwest::Error),

    #[error("Cloudinary respondió con error {status}: {body}")]
    Upload { status: u16, body: String },

    #[error("respuesta inválida de Cloudinary: falta `secure_url`")]
    MissingSecureUrl,
}

/// Forma mínima de la respuesta de Cloudinary que nos interesa.
/// Cloudinary devuelve muchos más campos (public_id, format, width, etc.),
/// pero para persistir solo necesitamos la URL HTTPS final del recurso.
#[derive(Debug, Deserialize)]
struct CloudinaryUploadResponse {
    secure_url: Option<String>,
}

/// Sube una imagen a Cloudinary mediante Unsigned Upload.
///
/// # Parámetros
/// - `config`: credenciales y carpeta destino (`cloud_name`, `upload_preset`, `folder`).
/// - `bytes`: contenido binario completo de la imagen.
/// - `filename`: nombre original del archivo (Cloudinary lo usa como hint
///   para la extensión y nombre por defecto del recurso).
///
/// # Retorno
/// La `secure_url` del recurso recién subido, lista para guardar en
/// `Receta.imagen`.
pub async fn subir_imagen(
    config: &CloudinaryConfig,
    bytes: Vec<u8>,
    filename: String,
) -> Result<String, CloudinaryError> {
    let endpoint = format!(
        "https://api.cloudinary.com/v1_1/{}/image/upload",
        config.cloud_name
    );

    let archivo = Part::bytes(bytes).file_name(filename);

    let formulario = Form::new()
        .part("file", archivo)
        .text("upload_preset", config.upload_preset.clone())
        .text("folder", config.folder.clone());

    let respuesta = reqwest::Client::new()
        .post(&endpoint)
        .multipart(formulario)
        .send()
        .await?;

    let estado = respuesta.status();
    if !estado.is_success() {
        let cuerpo = respuesta.text().await.unwrap_or_default();
        return Err(CloudinaryError::Upload {
            status: estado.as_u16(),
            body: cuerpo,
        });
    }

    let datos: CloudinaryUploadResponse = respuesta.json().await?;
    datos.secure_url.ok_or(CloudinaryError::MissingSecureUrl)
}
