use sqlx::PgPool;

use crate::models::puntuacion::{
    CreatePuntuacionRequest, PuntuacionResponse, UpdatePuntuacionRequest,
};

#[derive(Debug, thiserror::Error)]
pub enum PuntuacionError {
    #[error("recurso no encontrado")]
    NotFound,

    #[error("solicitud inválida: {0}")]
    InvalidRequest(String),

    #[error("error de base de datos")]
    Database(#[from] sqlx::Error),
}

pub async fn listar_por_receta(
    pool: &PgPool,
    receta_id: i32,
) -> Result<Vec<PuntuacionResponse>, PuntuacionError> {
    let existe = sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM receta WHERE id = $1) AS "exists!""#,
        receta_id
    )
    .fetch_one(pool)
    .await?;

    if !existe {
        return Err(PuntuacionError::NotFound);
    }

    let puntuaciones = sqlx::query_as!(
        PuntuacionResponse,
        r#"
        SELECT
            p.id,
            p.puntuacion,
            p.comentario,
            p.id_usuario,
            u.username,
            p.id_receta
        FROM puntuacion_receta p
        INNER JOIN usuario u ON u.id = p.id_usuario
        WHERE p.id_receta = $1
        ORDER BY p.id DESC
        "#,
        receta_id
    )
    .fetch_all(pool)
    .await?;

    Ok(puntuaciones)
}

pub async fn crear(
    pool: &PgPool,
    user_id: i32,
    receta_id: i32,
    body: &CreatePuntuacionRequest,
) -> Result<PuntuacionResponse, PuntuacionError> {
    validar_puntuacion(body.puntuacion)?;

    let existe = sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM receta WHERE id = $1) AS "exists!""#,
        receta_id
    )
    .fetch_one(pool)
    .await?;

    if !existe {
        return Err(PuntuacionError::NotFound);
    }

    let id = sqlx::query_scalar!(
        r#"
        INSERT INTO puntuacion_receta (puntuacion, comentario, id_usuario, id_receta)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
        body.puntuacion,
        body.comentario,
        user_id,
        receta_id
    )
    .fetch_one(pool)
    .await
    .map_err(|err| conflicto_duplicado(err))?;

    obtener_por_id(pool, id)
        .await?
        .ok_or(PuntuacionError::NotFound)
}

pub async fn actualizar(
    pool: &PgPool,
    user_id: i32,
    receta_id: i32,
    body: &UpdatePuntuacionRequest,
) -> Result<PuntuacionResponse, PuntuacionError> {
    if body.puntuacion.is_none() && body.comentario.is_none() {
        return Err(PuntuacionError::InvalidRequest(
            "debe enviar al menos un campo para actualizar".into(),
        ));
    }

    if let Some(p) = body.puntuacion {
        validar_puntuacion(p)?;
    }

    let id = sqlx::query_scalar!(
        r#"
        UPDATE puntuacion_receta
        SET
            puntuacion = COALESCE($1, puntuacion),
            comentario = COALESCE($2, comentario)
        WHERE id_usuario = $3 AND id_receta = $4
        RETURNING id
        "#,
        body.puntuacion,
        body.comentario,
        user_id,
        receta_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(PuntuacionError::NotFound)?;

    obtener_por_id(pool, id)
        .await?
        .ok_or(PuntuacionError::NotFound)
}

pub async fn eliminar(pool: &PgPool, user_id: i32, receta_id: i32) -> Result<(), PuntuacionError> {
    let result = sqlx::query!(
        r#"
        DELETE FROM puntuacion_receta
        WHERE id_usuario = $1 AND id_receta = $2
        "#,
        user_id,
        receta_id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(PuntuacionError::NotFound);
    }

    Ok(())
}

async fn obtener_por_id(
    pool: &PgPool,
    id: i32,
) -> Result<Option<PuntuacionResponse>, PuntuacionError> {
    let puntuacion = sqlx::query_as!(
        PuntuacionResponse,
        r#"
        SELECT
            p.id,
            p.puntuacion,
            p.comentario,
            p.id_usuario,
            u.username,
            p.id_receta
        FROM puntuacion_receta p
        INNER JOIN usuario u ON u.id = p.id_usuario
        WHERE p.id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(puntuacion)
}

fn validar_puntuacion(puntuacion: i32) -> Result<(), PuntuacionError> {
    if !(1..=5).contains(&puntuacion) {
        return Err(PuntuacionError::InvalidRequest(
            "la puntuación debe estar entre 1 y 5".into(),
        ));
    }
    Ok(())
}

fn conflicto_duplicado(err: sqlx::Error) -> PuntuacionError {
    if let sqlx::Error::Database(db) = &err {
        if db.constraint().is_some() {
            return PuntuacionError::InvalidRequest(
                "ya has puntuado esta receta".into(),
            );
        }
    }
    PuntuacionError::Database(err)
}
