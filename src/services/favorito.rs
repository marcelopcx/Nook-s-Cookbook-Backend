use sqlx::PgPool;

use crate::models::favorito::FavoritoResponse;
use crate::services::logro;

#[derive(Debug, thiserror::Error)]
pub enum FavoritoError {
    #[error("recurso no encontrado")]
    NotFound,

    #[error("solicitud inválida: {0}")]
    InvalidRequest(String),

    #[error("error de base de datos")]
    Database(#[from] sqlx::Error),
}

pub async fn listar(pool: &PgPool, user_id: i32) -> Result<Vec<FavoritoResponse>, FavoritoError> {
    let favoritos = sqlx::query_as!(
        FavoritoResponse,
        r#"
        SELECT
            r.id AS id_receta,
            r.nombre,
            r.descripcion,
            r.raciones,
            r.tiempo,
            r.promedio_puntuacion::float8 AS "promedio_puntuacion?: f64",
            r.dificultad,
            r.imagen,
            r.id_usuario_creador,
            u.username AS creador_username,
            f.fecha_agregado
        FROM favoritos_usuario f
        INNER JOIN receta r ON r.id = f.id_receta
        INNER JOIN usuario u ON u.id = r.id_usuario_creador
        WHERE f.id_usuario = $1
        ORDER BY f.fecha_agregado DESC
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(favoritos)
}

pub async fn agregar(
    pool: &PgPool,
    user_id: i32,
    receta_id: i32,
) -> Result<(), FavoritoError> {
    let existe = sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM receta WHERE id = $1) AS "exists!""#,
        receta_id
    )
    .fetch_one(pool)
    .await?;

    if !existe {
        return Err(FavoritoError::NotFound);
    }

    sqlx::query!(
        r#"
        INSERT INTO favoritos_usuario (id_usuario, id_receta)
        VALUES ($1, $2)
        ON CONFLICT (id_usuario, id_receta) DO NOTHING
        "#,
        user_id,
        receta_id
    )
    .execute(pool)
    .await?;

    logro::verificar_primer_favorito(pool, user_id).await?;

    Ok(())
}

pub async fn quitar(
    pool: &PgPool,
    user_id: i32,
    receta_id: i32,
) -> Result<(), FavoritoError> {
    let result = sqlx::query!(
        r#"
        DELETE FROM favoritos_usuario
        WHERE id_usuario = $1 AND id_receta = $2
        "#,
        user_id,
        receta_id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(FavoritoError::NotFound);
    }

    Ok(())
}
