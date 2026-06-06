use actix_web::{delete, get, patch, post, web, HttpResponse};
use sqlx::PgPool;

use crate::auth::AuthenticatedUser;
use crate::error::ApiError;
use crate::models::receta::{CreateRecetaRequest, UpdateRecetaRequest};
use crate::services::{ingrediente, receta, utensilio};

#[get("/ingredientes")]
pub async fn listar_ingredientes(pool: web::Data<PgPool>) -> Result<HttpResponse, ApiError> {
    let ingredientes = ingrediente::listar(pool.get_ref()).await?;
    Ok(HttpResponse::Ok().json(ingredientes))
}

#[get("/tipos-ingrediente")]
pub async fn listar_tipos_ingrediente(pool: web::Data<PgPool>) -> Result<HttpResponse, ApiError> {
    let tipos = ingrediente::listar_tipos(pool.get_ref()).await?;
    Ok(HttpResponse::Ok().json(tipos))
}

#[get("/utensilios")]
pub async fn listar_utensilios(pool: web::Data<PgPool>) -> Result<HttpResponse, ApiError> {
    let utensilios = utensilio::listar(pool.get_ref()).await?;
    Ok(HttpResponse::Ok().json(utensilios))
}

#[get("/tipos-utensilio")]
pub async fn listar_tipos_utensilio(pool: web::Data<PgPool>) -> Result<HttpResponse, ApiError> {
    let tipos = utensilio::listar_tipos(pool.get_ref()).await?;
    Ok(HttpResponse::Ok().json(tipos))
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

#[patch("/recetas/{id}")]
pub async fn actualizar_receta(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path: web::Path<i32>,
    body: web::Json<UpdateRecetaRequest>,
) -> Result<HttpResponse, ApiError> {
    let receta = receta::actualizar(pool.get_ref(), user.user_id, path.into_inner(), &body).await?;
    Ok(HttpResponse::Ok().json(receta))
}

#[delete("/recetas/{id}")]
pub async fn eliminar_receta(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    receta::eliminar(pool.get_ref(), user.user_id, path.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
