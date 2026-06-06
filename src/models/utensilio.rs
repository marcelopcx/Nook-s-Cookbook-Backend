use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UtensilioResponse {
    pub id: i32,
    pub nombre: String,
    pub id_tipo_utensilio: Option<i32>,
    pub tipo_nombre: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TipoUtensilioResponse {
    pub id: i32,
    pub nombre: String,
}
