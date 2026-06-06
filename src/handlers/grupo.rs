use actix_web::{delete, get, patch, post, web, HttpResponse};
use sqlx::PgPool;

use crate::auth::{AuthenticatedUser, OptionalAuthenticatedUser};
use crate::error::ApiError;
use crate::models::grupo::{CreateGrupoRequest, UpdateGrupoRequest};
use crate::services::grupo;

#[get("/grupos")]
pub async fn listar_grupos(pool: web::Data<PgPool>) -> Result<HttpResponse, ApiError> {
    let grupos = grupo::listar(pool.get_ref()).await?;
    Ok(HttpResponse::Ok().json(grupos))
}

#[get("/grupos/{id}")]
pub async fn obtener_grupo(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    user: OptionalAuthenticatedUser,
) -> Result<HttpResponse, ApiError> {
    let detalle = grupo::obtener_detalle(pool.get_ref(), path.into_inner(), user.user_id).await?;
    Ok(HttpResponse::Ok().json(detalle))
}

#[post("/grupos")]
pub async fn crear_grupo(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    body: web::Json<CreateGrupoRequest>,
) -> Result<HttpResponse, ApiError> {
    let grupo = grupo::crear(pool.get_ref(), user.user_id, &body).await?;
    Ok(HttpResponse::Created().json(grupo))
}

#[patch("/grupos/{id}")]
pub async fn actualizar_grupo(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path: web::Path<i32>,
    body: web::Json<UpdateGrupoRequest>,
) -> Result<HttpResponse, ApiError> {
    let grupo = grupo::actualizar(pool.get_ref(), user.user_id, path.into_inner(), &body).await?;
    Ok(HttpResponse::Ok().json(grupo))
}

#[delete("/grupos/{id}")]
pub async fn eliminar_grupo(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    grupo::eliminar(pool.get_ref(), user.user_id, path.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[get("/grupos/{id}/recetas")]
pub async fn listar_recetas_grupo(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let recetas = grupo::listar_recetas(pool.get_ref(), path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(recetas))
}

#[post("/grupos/{grupo_id}/recetas/{receta_id}")]
pub async fn agregar_receta_grupo(
    pool: web::Data<PgPool>,
    _user: AuthenticatedUser,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, ApiError> {
    let (grupo_id, receta_id) = path.into_inner();
    grupo::agregar_receta(pool.get_ref(), grupo_id, receta_id).await?;
    Ok(HttpResponse::Created().finish())
}

#[delete("/grupos/{grupo_id}/recetas/{receta_id}")]
pub async fn quitar_receta_grupo(
    pool: web::Data<PgPool>,
    _user: AuthenticatedUser,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, ApiError> {
    let (grupo_id, receta_id) = path.into_inner();
    grupo::quitar_receta(pool.get_ref(), grupo_id, receta_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[post("/grupos/{id}/seguir")]
pub async fn seguir_grupo(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    grupo::seguir(pool.get_ref(), user.user_id, path.into_inner()).await?;
    Ok(HttpResponse::Created().finish())
}

#[delete("/grupos/{id}/seguir")]
pub async fn dejar_seguir_grupo(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    grupo::dejar_de_seguir(pool.get_ref(), user.user_id, path.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[get("/grupos/{id}/seguidores")]
pub async fn listar_seguidores_grupo(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let seguidores = grupo::listar_seguidores(pool.get_ref(), path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(seguidores))
}
