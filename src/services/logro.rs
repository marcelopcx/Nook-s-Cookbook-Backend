use sqlx::PgPool;

use crate::models::logro::{LogroResponse, UsuarioLogroResponse};

#[derive(Debug, thiserror::Error)]
pub enum LogroError {
    #[error("error de base de datos")]
    Database(#[from] sqlx::Error),
}

pub async fn listar(pool: &PgPool) -> Result<Vec<LogroResponse>, LogroError> {
    let logros = sqlx::query_as!(
        LogroResponse,
        r#"
        SELECT id, nombre, descripcion
        FROM logro
        ORDER BY LOWER(nombre)
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(logros)
}

pub async fn listar_por_usuario(
    pool: &PgPool,
    user_id: i32,
) -> Result<Vec<UsuarioLogroResponse>, LogroError> {
    let logros = sqlx::query_as!(
        UsuarioLogroResponse,
        r#"
        SELECT
            ul.id_logro,
            l.nombre,
            l.descripcion,
            ul.fecha_obtenido
        FROM usuario_logro ul
        INNER JOIN logro l ON l.id = ul.id_logro
        WHERE ul.id_usuario = $1
        ORDER BY ul.fecha_obtenido DESC
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(logros)
}

pub async fn otorgar_por_nombre(
    pool: &PgPool,
    user_id: i32,
    nombre_logro: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO usuario_logro (id_usuario, id_logro)
        SELECT $1, l.id
        FROM logro l
        WHERE LOWER(l.nombre) = LOWER($2)
          AND NOT EXISTS (
              SELECT 1 FROM usuario_logro ul
              WHERE ul.id_usuario = $1 AND ul.id_logro = l.id
          )
        "#,
        user_id,
        nombre_logro
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn verificar_recetas_creadas(pool: &PgPool, user_id: i32) -> Result<(), sqlx::Error> {
    let count = sqlx::query_scalar!(
        r#"SELECT COUNT(*)::bigint FROM receta WHERE id_usuario_creador = $1"#,
        user_id
    )
    .fetch_one(pool)
    .await?
    .unwrap_or(0);

    if count >= 1 {
        otorgar_por_nombre(pool, user_id, "Primeros Pasos").await?;
    }
    if count >= 10 {
        otorgar_por_nombre(pool, user_id, "Aprendiz de Cocina").await?;
    }
    if count >= 100 {
        otorgar_por_nombre(pool, user_id, "Maestro Culinario").await?;
    }

    Ok(())
}

pub async fn verificar_gran_banquete(
    pool: &PgPool,
    user_id: i32,
    receta_id: i32,
    num_ingredientes: usize,
    num_pasos: usize,
    tiempo: Option<&str>,
) -> Result<(), sqlx::Error> {
    let minutos = tiempo.and_then(parse_minutos).unwrap_or(0);

    if num_ingredientes > 15 && num_pasos > 20 && minutos >= 120 {
        otorgar_por_nombre(pool, user_id, "El Gran Banquete").await?;
        let _ = receta_id;
    }

    Ok(())
}

pub async fn verificar_primer_grupo(pool: &PgPool, user_id: i32) -> Result<(), sqlx::Error> {
    otorgar_por_nombre(pool, user_id, "Miembro Fundador").await
}

pub async fn verificar_primer_favorito(pool: &PgPool, user_id: i32) -> Result<(), sqlx::Error> {
    otorgar_por_nombre(pool, user_id, "Guardado en Memoria").await
}

pub async fn verificar_eliminacion_cuenta(pool: &PgPool, user_id: i32) -> Result<(), sqlx::Error> {
    otorgar_por_nombre(pool, user_id, "Calabaza, Calabaza...").await
}

fn parse_minutos(tiempo: &str) -> Option<i32> {
    let digits: String = tiempo.chars().filter(|c| c.is_ascii_digit()).collect();
    digits.parse().ok()
}
