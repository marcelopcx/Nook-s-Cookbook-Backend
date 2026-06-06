use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PuntuacionResponse {
    pub id: i32,
    pub puntuacion: i32,
    pub comentario: Option<String>,
    pub id_usuario: i32,
    pub username: String,
    pub id_receta: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreatePuntuacionRequest {
    pub puntuacion: i32,
    pub comentario: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePuntuacionRequest {
    pub puntuacion: Option<i32>,
    pub comentario: Option<String>,
}
