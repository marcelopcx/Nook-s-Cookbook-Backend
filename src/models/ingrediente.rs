use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct IngredienteResponse {
    pub id: i32,
    pub nombre: String,
    pub id_tipo_ingrediente: Option<i32>,
    pub tipo_nombre: Option<String>,
}
