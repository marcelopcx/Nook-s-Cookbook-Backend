use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FavoritoResponse {
    pub id_receta: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub raciones: Option<i32>,
    pub tiempo: Option<String>,
    pub promedio_puntuacion: Option<f64>,
    pub dificultad: Option<String>,
    pub imagen: Option<String>,
    pub id_usuario_creador: i32,
    pub creador_username: String,
    pub fecha_agregado: NaiveDateTime,
}
