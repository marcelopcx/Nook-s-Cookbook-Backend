use sqlx::PgPool;

use crate::models::ingrediente::{IngredienteResponse, TipoIngredienteResponse};

#[derive(Debug, thiserror::Error)]
pub enum IngredienteError {
    #[error("error de base de datos")]
    Database(#[from] sqlx::Error),
}

pub async fn listar(pool: &PgPool) -> Result<Vec<IngredienteResponse>, IngredienteError> {
    let ingredientes = sqlx::query_as!(
        IngredienteResponse,
        r#"
        SELECT
            i.id,
            i.nombre,
            i.id_tipo_ingrediente,
            ti.nombre AS tipo_nombre
        FROM ingrediente i
        LEFT JOIN tipo_ingrediente ti ON ti.id = i.id_tipo_ingrediente
        ORDER BY LOWER(i.nombre)
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(ingredientes)
}

pub async fn listar_tipos(pool: &PgPool) -> Result<Vec<TipoIngredienteResponse>, IngredienteError> {
    let tipos = sqlx::query_as!(
        TipoIngredienteResponse,
        r#"
        SELECT id, nombre
        FROM tipo_ingrediente
        ORDER BY LOWER(nombre)
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(tipos)
}
