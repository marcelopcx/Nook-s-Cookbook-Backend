use sqlx::PgPool;

use crate::models::grupo::{
    CreateGrupoRequest, GrupoDetalleResponse, GrupoListItem, GrupoResponse, SeguidorResponse,
    UpdateGrupoRequest,
};
use crate::models::receta::RecetaListItem;
use crate::services::logro;

#[derive(Debug, thiserror::Error)]
pub enum GrupoError {
    #[error("recurso no encontrado")]
    NotFound,

    #[error("no autorizado")]
    Forbidden,

    #[error("solicitud inválida: {0}")]
    InvalidRequest(String),

    #[error("error de base de datos")]
    Database(#[from] sqlx::Error),
}

pub async fn listar(pool: &PgPool) -> Result<Vec<GrupoListItem>, GrupoError> {
    let grupos = sqlx::query_as!(
        GrupoListItem,
        r#"
        SELECT
            g.id,
            g.nombre,
            g.descripcion,
            g.publico,
            g.fecha_creacion,
            g.id_usuario_creador,
            u.username AS creador_username,
            (SELECT COUNT(*) FROM seguidor_grupo sg WHERE sg.id_grupo = g.id) AS "num_seguidores!",
            (SELECT COUNT(*) FROM receta_grupo rg WHERE rg.id_grupo = g.id) AS "num_recetas!"
        FROM grupo g
        INNER JOIN usuario u ON u.id = g.id_usuario_creador
        ORDER BY LOWER(g.nombre)
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(grupos)
}

pub async fn obtener_detalle(
    pool: &PgPool,
    grupo_id: i32,
    user_id: Option<i32>,
) -> Result<GrupoDetalleResponse, GrupoError> {
    let grupo = sqlx::query!(
        r#"
        SELECT
            g.id,
            g.nombre,
            g.descripcion,
            g.publico,
            g.fecha_creacion,
            g.id_usuario_creador,
            u.username AS creador_username,
            (SELECT COUNT(*) FROM seguidor_grupo sg WHERE sg.id_grupo = g.id) AS "num_seguidores!",
            (SELECT COUNT(*) FROM receta_grupo rg WHERE rg.id_grupo = g.id) AS "num_recetas!"
        FROM grupo g
        INNER JOIN usuario u ON u.id = g.id_usuario_creador
        WHERE g.id = $1
        "#,
        grupo_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(GrupoError::NotFound)?;

    let sigue = if let Some(uid) = user_id {
        sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM seguidor_grupo
                WHERE id_usuario = $1 AND id_grupo = $2
            ) AS "exists!"
            "#,
            uid,
            grupo_id
        )
        .fetch_one(pool)
        .await?
    } else {
        false
    };

    Ok(GrupoDetalleResponse {
        id: grupo.id,
        nombre: grupo.nombre,
        descripcion: grupo.descripcion,
        publico: grupo.publico,
        fecha_creacion: grupo.fecha_creacion,
        id_usuario_creador: grupo.id_usuario_creador,
        creador_username: grupo.creador_username,
        num_seguidores: grupo.num_seguidores,
        num_recetas: grupo.num_recetas,
        sigue,
    })
}

pub async fn crear(
    pool: &PgPool,
    user_id: i32,
    body: &CreateGrupoRequest,
) -> Result<GrupoResponse, GrupoError> {
    if body.nombre.trim().is_empty() {
        return Err(GrupoError::InvalidRequest(
            "el nombre es obligatorio".into(),
        ));
    }

    let grupo = sqlx::query_as!(
        GrupoResponse,
        r#"
        INSERT INTO grupo (nombre, descripcion, publico, id_usuario_creador)
        VALUES ($1, $2, COALESCE($3, TRUE), $4)
        RETURNING id, nombre, descripcion, publico, fecha_creacion, id_usuario_creador
        "#,
        body.nombre.trim(),
        body.descripcion,
        body.publico,
        user_id
    )
    .fetch_one(pool)
    .await?;

    logro::verificar_primer_grupo(pool, user_id).await?;

    Ok(grupo)
}

pub async fn actualizar(
    pool: &PgPool,
    user_id: i32,
    grupo_id: i32,
    body: &UpdateGrupoRequest,
) -> Result<GrupoResponse, GrupoError> {
    verificar_creador(pool, user_id, grupo_id).await?;

    if body.nombre.as_ref().is_some_and(|n| n.trim().is_empty()) {
        return Err(GrupoError::InvalidRequest(
            "el nombre no puede estar vacío".into(),
        ));
    }

    let grupo = sqlx::query_as!(
        GrupoResponse,
        r#"
        UPDATE grupo
        SET
            nombre = COALESCE($1, nombre),
            descripcion = COALESCE($2, descripcion),
            publico = COALESCE($3, publico)
        WHERE id = $4
        RETURNING id, nombre, descripcion, publico, fecha_creacion, id_usuario_creador
        "#,
        body.nombre.as_ref().map(|n| n.trim()),
        body.descripcion,
        body.publico,
        grupo_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(GrupoError::NotFound)?;

    Ok(grupo)
}

pub async fn eliminar(pool: &PgPool, user_id: i32, grupo_id: i32) -> Result<(), GrupoError> {
    verificar_creador(pool, user_id, grupo_id).await?;

    let result = sqlx::query!(r#"DELETE FROM grupo WHERE id = $1"#, grupo_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(GrupoError::NotFound);
    }

    Ok(())
}

pub async fn listar_recetas(
    pool: &PgPool,
    grupo_id: i32,
) -> Result<Vec<RecetaListItem>, GrupoError> {
    let existe = sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM grupo WHERE id = $1) AS "exists!""#,
        grupo_id
    )
    .fetch_one(pool)
    .await?;

    if !existe {
        return Err(GrupoError::NotFound);
    }

    let recetas = sqlx::query_as!(
        RecetaListItem,
        r#"
        SELECT
            r.id,
            r.nombre,
            r.descripcion,
            r.raciones,
            r.tiempo,
            r.promedio_puntuacion::float8 AS "promedio_puntuacion?: f64",
            r.dificultad,
            r.imagen,
            r.id_usuario_creador
        FROM receta r
        INNER JOIN receta_grupo rg ON rg.id_receta = r.id
        WHERE rg.id_grupo = $1
        ORDER BY LOWER(r.nombre)
        "#,
        grupo_id
    )
    .fetch_all(pool)
    .await?;

    Ok(recetas)
}

pub async fn agregar_receta(
    pool: &PgPool,
    grupo_id: i32,
    receta_id: i32,
) -> Result<(), GrupoError> {
    let grupo_existe = sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM grupo WHERE id = $1) AS "exists!""#,
        grupo_id
    )
    .fetch_one(pool)
    .await?;

    if !grupo_existe {
        return Err(GrupoError::NotFound);
    }

    sqlx::query!(
        r#"
        INSERT INTO receta_grupo (id_grupo, id_receta)
        VALUES ($1, $2)
        ON CONFLICT (id_grupo, id_receta) DO NOTHING
        "#,
        grupo_id,
        receta_id
    )
    .execute(pool)
    .await
    .map_err(|err| conflicto_referencia(err))?;

    Ok(())
}

pub async fn quitar_receta(
    pool: &PgPool,
    grupo_id: i32,
    receta_id: i32,
) -> Result<(), GrupoError> {
    let result = sqlx::query!(
        r#"
        DELETE FROM receta_grupo
        WHERE id_grupo = $1 AND id_receta = $2
        "#,
        grupo_id,
        receta_id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(GrupoError::NotFound);
    }

    Ok(())
}

pub async fn seguir(pool: &PgPool, user_id: i32, grupo_id: i32) -> Result<(), GrupoError> {
    let existe = sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM grupo WHERE id = $1) AS "exists!""#,
        grupo_id
    )
    .fetch_one(pool)
    .await?;

    if !existe {
        return Err(GrupoError::NotFound);
    }

    sqlx::query!(
        r#"
        INSERT INTO seguidor_grupo (id_usuario, id_grupo)
        VALUES ($1, $2)
        ON CONFLICT (id_usuario, id_grupo) DO NOTHING
        "#,
        user_id,
        grupo_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn dejar_de_seguir(
    pool: &PgPool,
    user_id: i32,
    grupo_id: i32,
) -> Result<(), GrupoError> {
    let result = sqlx::query!(
        r#"
        DELETE FROM seguidor_grupo
        WHERE id_usuario = $1 AND id_grupo = $2
        "#,
        user_id,
        grupo_id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(GrupoError::NotFound);
    }

    Ok(())
}

pub async fn listar_seguidores(
    pool: &PgPool,
    grupo_id: i32,
) -> Result<Vec<SeguidorResponse>, GrupoError> {
    let existe = sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM grupo WHERE id = $1) AS "exists!""#,
        grupo_id
    )
    .fetch_one(pool)
    .await?;

    if !existe {
        return Err(GrupoError::NotFound);
    }

    let seguidores = sqlx::query_as!(
        SeguidorResponse,
        r#"
        SELECT
            u.id,
            u.username,
            sg.fecha_seguido
        FROM seguidor_grupo sg
        INNER JOIN usuario u ON u.id = sg.id_usuario
        WHERE sg.id_grupo = $1
        ORDER BY sg.fecha_seguido DESC
        "#,
        grupo_id
    )
    .fetch_all(pool)
    .await?;

    Ok(seguidores)
}

async fn verificar_creador(pool: &PgPool, user_id: i32, grupo_id: i32) -> Result<(), GrupoError> {
    let creador = sqlx::query_scalar!(
        r#"SELECT id_usuario_creador FROM grupo WHERE id = $1"#,
        grupo_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(GrupoError::NotFound)?;

    if creador != user_id {
        return Err(GrupoError::Forbidden);
    }

    Ok(())
}

fn conflicto_referencia(err: sqlx::Error) -> GrupoError {
    if let sqlx::Error::Database(db) = &err {
        if db.constraint().is_some() {
            return GrupoError::InvalidRequest("receta no válida".into());
        }
    }
    GrupoError::Database(err)
}
