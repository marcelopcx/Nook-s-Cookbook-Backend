use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct RecetaListItem {
    pub id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub raciones: Option<i32>,
    pub tiempo: Option<String>,
    pub promedio_puntuacion: Option<f64>,
    pub dificultad: Option<String>,
    pub imagen: Option<String>,
    pub id_usuario_creador: i32,
}

#[derive(Debug, Deserialize)]
pub struct PasoInput {
    pub numero_paso: i32,
    pub instruccion: String,
}

#[derive(Debug, Deserialize)]
pub struct IngredienteRecetaInput {
    pub id_ingrediente: i32,
    pub cantidad: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UtensilioRecetaInput {
    pub id_utensilio: i32,
    pub cantidad: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRecetaRequest {
    pub nombre: String,
    pub descripcion: Option<String>,
    pub raciones: Option<i32>,
    pub tiempo: Option<String>,
    pub dificultad: Option<String>,
    pub imagen: Option<String>,
    pub pasos: Vec<PasoInput>,
    pub ingredientes: Vec<IngredienteRecetaInput>,
    pub utensilios: Vec<UtensilioRecetaInput>,
}

#[derive(Debug, Serialize)]
pub struct RecetaResponse {
    pub id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub raciones: Option<i32>,
    pub tiempo: Option<String>,
    pub promedio_puntuacion: Option<f64>,
    pub dificultad: Option<String>,
    pub imagen: Option<String>,
    pub id_usuario_creador: i32,
}

#[derive(Debug, Serialize)]
pub struct PasoResponse {
    pub numero_paso: i32,
    pub instruccion: String,
}

#[derive(Debug, Serialize)]
pub struct IngredienteRecetaResponse {
    pub id_ingrediente: i32,
    pub nombre: String,
    pub cantidad: Option<String>,
    pub tipo_nombre: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UtensilioRecetaResponse {
    pub id_utensilio: i32,
    pub nombre: String,
    pub cantidad: Option<String>,
    pub tipo_nombre: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RecetaDetalleResponse {
    pub id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub raciones: Option<i32>,
    pub tiempo: Option<String>,
    pub promedio_puntuacion: Option<f64>,
    pub dificultad: Option<String>,
    pub imagen: Option<String>,
    pub id_usuario_creador: i32,
    pub creador_username: String,
    pub pasos: Vec<PasoResponse>,
    pub ingredientes: Vec<IngredienteRecetaResponse>,
    pub utensilios: Vec<UtensilioRecetaResponse>,
}
