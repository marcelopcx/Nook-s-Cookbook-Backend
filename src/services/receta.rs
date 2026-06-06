use sqlx::PgPool;

use crate::models::receta::{
    CreateRecetaRequest, IngredienteRecetaResponse, PasoResponse, RecetaDetalleResponse,
    RecetaListItem, RecetaResponse, UpdateRecetaRequest, UtensilioRecetaResponse,
};
use crate::services::logro;

#[derive(Debug, thiserror::Error)]
pub enum RecetaError {
    #[error("solicitud inválida: {0}")]
    InvalidRequest(String),

    #[error("no autorizado")]
    Forbidden,

    #[error("recurso no encontrado")]
    NotFound,

    #[error("error de base de datos")]
    Database(#[from] sqlx::Error),
}

pub async fn listar(pool: &PgPool) -> Result<Vec<RecetaListItem>, RecetaError> {
    let recetas = sqlx::query_as!(
        RecetaListItem,
        r#"
        SELECT
            id,
            nombre,
            descripcion,
            raciones,
            tiempo,
            promedio_puntuacion::float8 AS "promedio_puntuacion?: f64",
            dificultad,
            imagen,
            id_usuario_creador
        FROM receta
        ORDER BY LOWER(nombre)
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(recetas)
}

pub async fn listar_por_usuario(
    pool: &PgPool,
    user_id: i32,
) -> Result<Vec<RecetaListItem>, RecetaError> {
    let recetas = sqlx::query_as!(
        RecetaListItem,
        r#"
        SELECT
            id,
            nombre,
            descripcion,
            raciones,
            tiempo,
            promedio_puntuacion::float8 AS "promedio_puntuacion?: f64",
            dificultad,
            imagen,
            id_usuario_creador
        FROM receta
        WHERE id_usuario_creador = $1
        ORDER BY LOWER(nombre)
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(recetas)
}

pub async fn crear(
    pool: &PgPool,
    user_id: i32,
    body: &CreateRecetaRequest,
) -> Result<RecetaResponse, RecetaError> {
    validar_solicitud(body)?;

    let mut tx = pool.begin().await?;

    let receta_id = sqlx::query_scalar!(
        r#"
        INSERT INTO receta (
            nombre,
            descripcion,
            raciones,
            tiempo,
            dificultad,
            imagen,
            id_usuario_creador
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id
        "#,
        body.nombre.trim(),
        body.descripcion,
        body.raciones,
        body.tiempo,
        body.dificultad,
        body.imagen,
        user_id
    )
    .fetch_one(&mut *tx)
    .await?;

    for paso in &body.pasos {
        sqlx::query!(
            r#"
            INSERT INTO paso_receta (id_receta, numero_paso, instruccion)
            VALUES ($1, $2, $3)
            "#,
            receta_id,
            paso.numero_paso,
            paso.instruccion.trim()
        )
        .execute(&mut *tx)
        .await?;
    }

    for ing in &body.ingredientes {
        sqlx::query!(
            r#"
            INSERT INTO ingrediente_receta (id_receta, id_ingrediente, cantidad)
            VALUES ($1, $2, $3)
            "#,
            receta_id,
            ing.id_ingrediente,
            ing.cantidad
        )
        .execute(&mut *tx)
        .await
        .map_err(|err| conflicto_referencia(err, "ingrediente"))?;
    }

    for ut in &body.utensilios {
        sqlx::query!(
            r#"
            INSERT INTO utensilio_receta (id_receta, id_utensilio, cantidad)
            VALUES ($1, $2, $3)
            "#,
            receta_id,
            ut.id_utensilio,
            ut.cantidad
        )
        .execute(&mut *tx)
        .await
        .map_err(|err| conflicto_referencia(err, "utensilio"))?;
    }

    tx.commit().await?;

    logro::verificar_recetas_creadas(pool, user_id).await?;
    logro::verificar_gran_banquete(
        pool,
        user_id,
        receta_id,
        body.ingredientes.len(),
        body.pasos.len(),
        body.tiempo.as_deref(),
    )
    .await?;

    obtener_por_id(pool, receta_id)
        .await?
        .ok_or(RecetaError::NotFound)
}

pub async fn obtener_detalle(
    pool: &PgPool,
    id: i32,
) -> Result<RecetaDetalleResponse, RecetaError> {
    let receta = sqlx::query!(
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
            r.id_usuario_creador,
            u.username AS creador_username
        FROM receta r
        INNER JOIN usuario u ON u.id = r.id_usuario_creador
        WHERE r.id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(RecetaError::NotFound)?;

    let pasos = sqlx::query_as!(
        PasoResponse,
        r#"
        SELECT numero_paso, instruccion
        FROM paso_receta
        WHERE id_receta = $1
        ORDER BY numero_paso ASC
        "#,
        id
    )
    .fetch_all(pool)
    .await?;

    let ingredientes = sqlx::query_as!(
        IngredienteRecetaResponse,
        r#"
        SELECT
            ir.id_ingrediente,
            i.nombre,
            ir.cantidad,
            ti.nombre AS tipo_nombre
        FROM ingrediente_receta ir
        INNER JOIN ingrediente i ON i.id = ir.id_ingrediente
        LEFT JOIN tipo_ingrediente ti ON ti.id = i.id_tipo_ingrediente
        WHERE ir.id_receta = $1
        ORDER BY LOWER(i.nombre)
        "#,
        id
    )
    .fetch_all(pool)
    .await?;

    let utensilios = sqlx::query_as!(
        UtensilioRecetaResponse,
        r#"
        SELECT
            ur.id_utensilio,
            u.nombre,
            ur.cantidad,
            tu.nombre AS tipo_nombre
        FROM utensilio_receta ur
        INNER JOIN utensilio u ON u.id = ur.id_utensilio
        LEFT JOIN tipo_utensilio tu ON tu.id = u.id_tipo_utensilio
        WHERE ur.id_receta = $1
        ORDER BY LOWER(u.nombre)
        "#,
        id
    )
    .fetch_all(pool)
    .await?;

    Ok(RecetaDetalleResponse {
        id: receta.id,
        nombre: receta.nombre,
        descripcion: receta.descripcion,
        raciones: receta.raciones,
        tiempo: receta.tiempo,
        promedio_puntuacion: receta.promedio_puntuacion,
        dificultad: receta.dificultad,
        imagen: receta.imagen,
        id_usuario_creador: receta.id_usuario_creador,
        creador_username: receta.creador_username,
        pasos,
        ingredientes,
        utensilios,
    })
}

pub async fn actualizar(
    pool: &PgPool,
    user_id: i32,
    receta_id: i32,
    body: &UpdateRecetaRequest,
) -> Result<RecetaResponse, RecetaError> {
    validar_solicitud_actualizacion(body)?;
    verificar_propietario(pool, user_id, receta_id).await?;

    let mut tx = pool.begin().await?;

    sqlx::query!(
        r#"
        UPDATE receta
        SET
            nombre = $1,
            descripcion = $2,
            raciones = $3,
            tiempo = $4,
            dificultad = $5,
            imagen = $6
        WHERE id = $7
        "#,
        body.nombre.trim(),
        body.descripcion,
        body.raciones,
        body.tiempo,
        body.dificultad,
        body.imagen,
        receta_id
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query!(
        r#"DELETE FROM paso_receta WHERE id_receta = $1"#,
        receta_id
    )
    .execute(&mut *tx)
    .await?;

    for paso in &body.pasos {
        sqlx::query!(
            r#"
            INSERT INTO paso_receta (id_receta, numero_paso, instruccion)
            VALUES ($1, $2, $3)
            "#,
            receta_id,
            paso.numero_paso,
            paso.instruccion.trim()
        )
        .execute(&mut *tx)
        .await?;
    }

    sqlx::query!(
        r#"DELETE FROM ingrediente_receta WHERE id_receta = $1"#,
        receta_id
    )
    .execute(&mut *tx)
    .await?;

    for ing in &body.ingredientes {
        sqlx::query!(
            r#"
            INSERT INTO ingrediente_receta (id_receta, id_ingrediente, cantidad)
            VALUES ($1, $2, $3)
            "#,
            receta_id,
            ing.id_ingrediente,
            ing.cantidad
        )
        .execute(&mut *tx)
        .await
        .map_err(|err| conflicto_referencia(err, "ingrediente"))?;
    }

    sqlx::query!(
        r#"DELETE FROM utensilio_receta WHERE id_receta = $1"#,
        receta_id
    )
    .execute(&mut *tx)
    .await?;

    for ut in &body.utensilios {
        sqlx::query!(
            r#"
            INSERT INTO utensilio_receta (id_receta, id_utensilio, cantidad)
            VALUES ($1, $2, $3)
            "#,
            receta_id,
            ut.id_utensilio,
            ut.cantidad
        )
        .execute(&mut *tx)
        .await
        .map_err(|err| conflicto_referencia(err, "utensilio"))?;
    }

    tx.commit().await?;

    logro::verificar_gran_banquete(
        pool,
        user_id,
        receta_id,
        body.ingredientes.len(),
        body.pasos.len(),
        body.tiempo.as_deref(),
    )
    .await?;

    obtener_por_id(pool, receta_id)
        .await?
        .ok_or(RecetaError::NotFound)
}

pub async fn eliminar(pool: &PgPool, user_id: i32, receta_id: i32) -> Result<(), RecetaError> {
    verificar_propietario(pool, user_id, receta_id).await?;

    let result = sqlx::query!(r#"DELETE FROM receta WHERE id = $1"#, receta_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(RecetaError::NotFound);
    }

    Ok(())
}

async fn obtener_por_id(pool: &PgPool, id: i32) -> Result<Option<RecetaResponse>, RecetaError> {
    let receta = sqlx::query_as!(
        RecetaResponse,
        r#"
        SELECT
            id,
            nombre,
            descripcion,
            raciones,
            tiempo,
            promedio_puntuacion::float8 AS "promedio_puntuacion?: f64",
            dificultad,
            imagen,
            id_usuario_creador
        FROM receta
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(receta)
}

fn validar_solicitud(body: &CreateRecetaRequest) -> Result<(), RecetaError> {
    if body.nombre.trim().is_empty() {
        return Err(RecetaError::InvalidRequest(
            "el nombre es obligatorio".into(),
        ));
    }

    if body.pasos.is_empty() {
        return Err(RecetaError::InvalidRequest(
            "la receta debe tener al menos un paso".into(),
        ));
    }

    for paso in &body.pasos {
        if paso.numero_paso <= 0 {
            return Err(RecetaError::InvalidRequest(
                "cada paso debe tener un número mayor a cero".into(),
            ));
        }
        if paso.instruccion.trim().is_empty() {
            return Err(RecetaError::InvalidRequest(
                "cada paso debe incluir una instrucción".into(),
            ));
        }
    }

    if let Some(raciones) = body.raciones {
        if raciones <= 0 {
            return Err(RecetaError::InvalidRequest(
                "las raciones deben ser mayores a cero".into(),
            ));
        }
    }

    Ok(())
}

fn validar_solicitud_actualizacion(body: &UpdateRecetaRequest) -> Result<(), RecetaError> {
    if body.nombre.trim().is_empty() {
        return Err(RecetaError::InvalidRequest(
            "el nombre es obligatorio".into(),
        ));
    }

    if body.pasos.is_empty() {
        return Err(RecetaError::InvalidRequest(
            "la receta debe tener al menos un paso".into(),
        ));
    }

    for paso in &body.pasos {
        if paso.numero_paso <= 0 {
            return Err(RecetaError::InvalidRequest(
                "cada paso debe tener un número mayor a cero".into(),
            ));
        }
        if paso.instruccion.trim().is_empty() {
            return Err(RecetaError::InvalidRequest(
                "cada paso debe incluir una instrucción".into(),
            ));
        }
    }

    if let Some(raciones) = body.raciones {
        if raciones <= 0 {
            return Err(RecetaError::InvalidRequest(
                "las raciones deben ser mayores a cero".into(),
            ));
        }
    }

    Ok(())
}

async fn verificar_propietario(
    pool: &PgPool,
    user_id: i32,
    receta_id: i32,
) -> Result<(), RecetaError> {
    let creador = sqlx::query_scalar!(
        r#"SELECT id_usuario_creador FROM receta WHERE id = $1"#,
        receta_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(RecetaError::NotFound)?;

    if creador != user_id {
        return Err(RecetaError::Forbidden);
    }

    Ok(())
}

fn conflicto_referencia(err: sqlx::Error, recurso: &str) -> RecetaError {
    if let sqlx::Error::Database(db) = &err {
        if db.constraint().is_some() {
            return RecetaError::InvalidRequest(format!("{recurso} no válido"));
        }
    }
    RecetaError::Database(err)
}
