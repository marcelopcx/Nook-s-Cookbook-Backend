use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct LogroResponse {
    pub id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UsuarioLogroResponse {
    pub id_logro: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub fecha_obtenido: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct ReclamarLogroRequest {
    pub nombre: String,
}
