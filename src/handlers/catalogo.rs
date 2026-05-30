use actix_web::{get, post, web, HttpResponse};
use sqlx::PgPool;

use crate::auth::AuthenticatedUser;
use crate::error::ApiError;
use crate::models::receta::CreateRecetaRequest;
use crate::services::{ingrediente, receta};

#[get("/ingredientes")]
pub async fn listar_ingredientes(pool: web::Data<PgPool>) -> Result<HttpResponse, ApiError> {
    let ingredientes = ingrediente::listar(pool.get_ref()).await?;
    Ok(HttpResponse::Ok().json(ingredientes))
}

#[get("/recetas")]
pub async fn listar_recetas(pool: web::Data<PgPool>) -> Result<HttpResponse, ApiError> {
    let recetas = receta::listar(pool.get_ref()).await?;
    Ok(HttpResponse::Ok().json(recetas))
}

#[get("/recetas/{id}")]
pub async fn obtener_receta(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let detalle = receta::obtener_detalle(pool.get_ref(), path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(detalle))
}

#[post("/recetas")]
pub async fn crear_receta(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    body: web::Json<CreateRecetaRequest>,
) -> Result<HttpResponse, ApiError> {
    let receta = receta::crear(pool.get_ref(), user.user_id, &body).await?;
    Ok(HttpResponse::Created().json(receta))
}
