use actix_web::{get, post, web, HttpResponse};
use sqlx::PgPool;

use crate::auth::AuthenticatedUser;
use crate::error::ApiError;
use crate::models::logro::ReclamarLogroRequest;
use crate::services::logro;

#[get("/logros")]
pub async fn listar_logros(pool: web::Data<PgPool>) -> Result<HttpResponse, ApiError> {
    let logros = logro::listar(pool.get_ref()).await?;
    Ok(HttpResponse::Ok().json(logros))
}

#[get("/auth/me/logros")]
pub async fn listar_mis_logros(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, ApiError> {
    let logros = logro::listar_por_usuario(pool.get_ref(), user.user_id).await?;
    Ok(HttpResponse::Ok().json(logros))
}

#[post("/auth/me/logros/reclamar")]
pub async fn reclamar_logro(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    body: web::Json<ReclamarLogroRequest>,
) -> Result<HttpResponse, ApiError> {
    let logro = logro::reclamar_desde_cliente(pool.get_ref(), user.user_id, &body.nombre).await?;
    Ok(HttpResponse::Created().json(logro))
}
