use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct Usuario {
    pub id: i32,
    pub username: String,
    pub public: bool,
    pub id_persona: i32,
}

pub struct UsuarioPassword {
    pub id: i32,
    pub username: String,
    pub public: bool,
    pub id_persona: i32,
    pub contrasena: String,
}

pub(crate) struct PerfilRow {
    pub(crate) id: i32,
    pub(crate) username: String,
    pub(crate) public: bool,
    pub(crate) nombre: String,
    pub(crate) apellido: Option<String>,
    pub(crate) correo: String,
    pub(crate) telefono: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: Usuario,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub nombre: String,
    pub apellido: Option<String>,
    pub correo: String,
    pub telefono: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub user: Usuario,
}

#[derive(Debug, Serialize)]
pub struct PerfilResponse {
    pub id: i32,
    pub username: String,
    pub public: bool,
    pub nombre: String,
    pub apellido: Option<String>,
    pub correo: String,
    pub telefono: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PerfilPublicoResponse {
    pub id: i32,
    pub username: String,
    pub nombre: String,
    pub apellido: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMeRequest {
    pub username: Option<String>,
    pub public: Option<bool>,
    pub password: Option<String>,
    pub nombre: Option<String>,
    pub apellido: Option<String>,
    pub correo: Option<String>,
    pub telefono: Option<String>,
}