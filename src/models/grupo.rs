use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::models::receta::RecetaListItem;

#[derive(Debug, Serialize)]
pub struct GrupoListItem {
    pub id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub publico: bool,
    pub fecha_creacion: NaiveDateTime,
    pub id_usuario_creador: i32,
    pub creador_username: String,
    pub num_seguidores: i64,
    pub num_recetas: i64,
}

#[derive(Debug, Serialize)]
pub struct GrupoDetalleResponse {
    pub id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub publico: bool,
    pub fecha_creacion: NaiveDateTime,
    pub id_usuario_creador: i32,
    pub creador_username: String,
    pub num_seguidores: i64,
    pub num_recetas: i64,
    pub sigue: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateGrupoRequest {
    pub nombre: String,
    pub descripcion: Option<String>,
    pub publico: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateGrupoRequest {
    pub nombre: Option<String>,
    pub descripcion: Option<String>,
    pub publico: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct GrupoResponse {
    pub id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub publico: bool,
    pub fecha_creacion: NaiveDateTime,
    pub id_usuario_creador: i32,
}

#[derive(Debug, Serialize)]
pub struct SeguidorResponse {
    pub id: i32,
    pub username: String,
    pub fecha_seguido: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct GrupoRecetasResponse {
    pub recetas: Vec<RecetaListItem>,
}
