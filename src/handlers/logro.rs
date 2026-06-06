use actix_web::{get, web, HttpResponse};
use sqlx::PgPool;

use crate::auth::AuthenticatedUser;
use crate::error::ApiError;
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
