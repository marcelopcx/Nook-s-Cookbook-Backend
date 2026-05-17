use bcrypt::BcryptError;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::models::usuario::{
    PerfilResponse, PerfilRow, RegisterRequest, UpdateMeRequest, Usuario, UsuarioPassword,
};

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("credenciales inválidas")]
    InvalidCredentials,

    #[error("no autorizado")]
    Unauthorized,

    #[error("usuario no encontrado")]
    NotFound,

    #[error("conflicto: usuario o correo ya registrado")]
    Conflict,

    #[error("error de base de datos")]
    Database(#[from] sqlx::Error),

    #[error("error al verificar contraseña")]
    PasswordHash(#[from] BcryptError),

    #[error("error al generar token")]
    Token(#[from] jsonwebtoken::errors::Error),
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: i32,
    exp: usize,
}

pub async fn login(
    pool: &PgPool,
    jwt_secret: &str,
    jwt_expiration_hours: i64,
    username: &str,
    password: &str,
) -> Result<(String, Usuario), AuthError> {
    let user = sqlx::query_as!(
        UsuarioPassword,
        r#"
        SELECT
            id,
            username,
            public,
            id_persona,
            contrasena AS "contrasena!"
        FROM usuario
        WHERE LOWER(username) = LOWER($1)
        "#,
        username
    )
        .fetch_optional(pool)
        .await?
        .ok_or(AuthError::InvalidCredentials)?;

    let valid = bcrypt::verify(password, &user.contrasena)?;
    if !valid {
        return Err(AuthError::InvalidCredentials);
    }

    let token = create_jwt(user.id, jwt_secret, jwt_expiration_hours)?;
    let public_user = Usuario {
        id: user.id,
        username: user.username,
        public: user.public,
        id_persona: user.id_persona,
    };

    Ok((token, public_user))
}

fn conflicto_duplicado(err: sqlx::Error) -> AuthError {
    if let sqlx::Error::Database(db) = &err {
        if db.constraint().is_some() {
            return AuthError::Conflict;
        }
    }
    AuthError::Database(err)
}

pub async fn register(pool: &PgPool, body: &RegisterRequest) -> Result<Usuario, AuthError> {
    let hash = bcrypt::hash(&body.password, bcrypt::DEFAULT_COST)?;

    let mut tx = pool.begin().await?;

    let persona_id = sqlx::query_scalar!(
        r#"
        INSERT INTO persona (nombre, apellido, correo, telefono)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
        body.nombre,
        body.apellido,
        body.correo,
        body.telefono
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(conflicto_duplicado)?;

    let user = sqlx::query_as!(
        Usuario,
        r#"
        INSERT INTO usuario (username, contrasena, public, id_persona)
        VALUES ($1, $2, TRUE, $3)
        RETURNING id, username, public, id_persona
        "#,
        body.username,
        hash,
        persona_id
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(conflicto_duplicado)?;

    tx.commit().await?;

    Ok(user)
}

pub fn user_id_from_token(token: &str, secret: &str) -> Result<i32, AuthError> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AuthError::Unauthorized)?;

    Ok(data.claims.sub)
}

pub async fn get_profile(pool: &PgPool, user_id: i32) -> Result<PerfilResponse, AuthError> {
    let row = sqlx::query_as!(
        PerfilRow,
        r#"
        SELECT
            u.id,
            u.username,
            u.public,
            p.nombre,
            p.apellido,
            p.correo,
            p.telefono
        FROM usuario u
        INNER JOIN persona p ON p.id = u.id_persona
        WHERE u.id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AuthError::NotFound)?;

    Ok(PerfilResponse {
        id: row.id,
        username: row.username,
        public: row.public,
        nombre: row.nombre,
        apellido: row.apellido,
        correo: row.correo,
        telefono: row.telefono,
    })
}

pub async fn update_profile(
    pool: &PgPool,
    user_id: i32,
    body: &UpdateMeRequest,
) -> Result<PerfilResponse, AuthError> {
    let id_persona = sqlx::query_scalar!(
        r#"SELECT id_persona FROM usuario WHERE id = $1"#,
        user_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AuthError::NotFound)?;

    let mut tx = pool.begin().await?;

    if let Some(username) = &body.username {
        sqlx::query!(
            r#"UPDATE usuario SET username = $1 WHERE id = $2"#,
            username,
            user_id
        )
            .execute(&mut *tx)
            .await
            .map_err(conflicto_duplicado)?;
    }

    if let Some(public) = body.public {
        sqlx::query!(
            r#"UPDATE usuario SET public = $1 WHERE id = $2"#,
            public,
            user_id
        )
            .execute(&mut *tx)
            .await?;
    }

    if let Some(password) = &body.password {
        let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
        sqlx::query!(
            r#"UPDATE usuario SET contrasena = $1 WHERE id = $2"#,
            hash,
            user_id
        )
            .execute(&mut *tx)
            .await?;
    }

    if body.nombre.is_some() || body.apellido.is_some() || body.correo.is_some() || body.telefono.is_some()
    {
        sqlx::query!(
            r#"
            UPDATE persona
            SET
                nombre = COALESCE($1, nombre),
                apellido = COALESCE($2, apellido),
                correo = COALESCE($3, correo),
                telefono = COALESCE($4, telefono)
            WHERE id = $5
            "#,
            body.nombre,
            body.apellido,
            body.correo,
            body.telefono,
            id_persona
        )
            .execute(&mut *tx)
            .await
            .map_err(conflicto_duplicado)?;
    }

    tx.commit().await?;

    get_profile(pool, user_id).await
}

fn create_jwt(user_id: i32, secret: &str, expiration_hours: i64) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = (Utc::now() + Duration::hours(expiration_hours)).timestamp() as usize;
    let claims = Claims { sub: user_id, exp };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}