use sqlx::PgPool;

use crate::models::utensilio::{TipoUtensilioResponse, UtensilioResponse};

#[derive(Debug, thiserror::Error)]
pub enum UtensilioError {
    #[error("error de base de datos")]
    Database(#[from] sqlx::Error),
}

pub async fn listar(pool: &PgPool) -> Result<Vec<UtensilioResponse>, UtensilioError> {
    let utensilios = sqlx::query_as!(
        UtensilioResponse,
        r#"
        SELECT
            u.id,
            u.nombre,
            u.id_tipo_utensilio,
            tu.nombre AS tipo_nombre
        FROM utensilio u
        LEFT JOIN tipo_utensilio tu ON tu.id = u.id_tipo_utensilio
        ORDER BY LOWER(u.nombre)
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(utensilios)
}

pub async fn listar_tipos(pool: &PgPool) -> Result<Vec<TipoUtensilioResponse>, UtensilioError> {
    let tipos = sqlx::query_as!(
        TipoUtensilioResponse,
        r#"
        SELECT id, nombre
        FROM tipo_utensilio
        ORDER BY LOWER(nombre)
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(tipos)
}
