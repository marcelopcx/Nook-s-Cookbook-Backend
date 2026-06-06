use actix_web::{delete, get, patch, post, web, HttpResponse};
use sqlx::PgPool;

use crate::auth::AuthenticatedUser;
use crate::error::ApiError;
use crate::models::puntuacion::{CreatePuntuacionRequest, UpdatePuntuacionRequest};
use crate::services::{favorito, puntuacion};

#[get("/auth/me/favoritos")]
pub async fn listar_favoritos(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, ApiError> {
    let favoritos = favorito::listar(pool.get_ref(), user.user_id).await?;
    Ok(HttpResponse::Ok().json(favoritos))
}

#[post("/recetas/{id}/favorito")]
pub async fn agregar_favorito(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    favorito::agregar(pool.get_ref(), user.user_id, path.into_inner()).await?;
    Ok(HttpResponse::Created().finish())
}

#[delete("/recetas/{id}/favorito")]
pub async fn quitar_favorito(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    favorito::quitar(pool.get_ref(), user.user_id, path.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[get("/recetas/{id}/puntuaciones")]
pub async fn listar_puntuaciones(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let puntuaciones = puntuacion::listar_por_receta(pool.get_ref(), path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(puntuaciones))
}

#[post("/recetas/{id}/puntuaciones")]
pub async fn crear_puntuacion(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path: web::Path<i32>,
    body: web::Json<CreatePuntuacionRequest>,
) -> Result<HttpResponse, ApiError> {
    let puntuacion =
        puntuacion::crear(pool.get_ref(), user.user_id, path.into_inner(), &body).await?;
    Ok(HttpResponse::Created().json(puntuacion))
}

#[patch("/recetas/{id}/puntuaciones")]
pub async fn actualizar_puntuacion(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path: web::Path<i32>,
    body: web::Json<UpdatePuntuacionRequest>,
) -> Result<HttpResponse, ApiError> {
    let puntuacion =
        puntuacion::actualizar(pool.get_ref(), user.user_id, path.into_inner(), &body).await?;
    Ok(HttpResponse::Ok().json(puntuacion))
}

#[delete("/recetas/{id}/puntuaciones")]
pub async fn eliminar_puntuacion(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    puntuacion::eliminar(pool.get_ref(), user.user_id, path.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
