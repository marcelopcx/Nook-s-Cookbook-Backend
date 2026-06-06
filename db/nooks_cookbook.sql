-- =============================================================================
-- Nook's Cookbook — Esquema de base de datos (PostgreSQL)
-- =============================================================================
-- Reglas de negocio reflejadas en el DDL:
--   • Correo y username únicos (no se registran emails repetidos).
--   • Recetas listadas en orden alfabético (índices sobre LOWER(nombre)).
--   • Una receta puede pertenecer a varios grupos (Receta_Grupo N:M).
--   • Quitar una receta de un grupo solo elimina la fila en Receta_Grupo.
--   • Al borrar un grupo se eliminan solo las recetas del dueño del grupo
--     que estaban vinculadas a ese grupo (trigger).
--   • Al borrar un usuario se eliminan sus recetas, grupos creados y su Persona.
-- =============================================================================

CREATE SCHEMA IF NOT EXISTS nooks_cookbook;
SET search_path TO nooks_cookbook;

-- -----------------------------------------------------------------------------
-- 1. Entidades de tipos (maestras)
-- -----------------------------------------------------------------------------
CREATE TABLE Tipo_Ingrediente (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL UNIQUE
);

CREATE TABLE Tipo_Utensilio (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL UNIQUE
);

-- -----------------------------------------------------------------------------
-- 2. Entidades de sujetos
-- -----------------------------------------------------------------------------
CREATE TABLE Persona (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    apellido VARCHAR(100),
    correo VARCHAR(150) NOT NULL UNIQUE,
    telefono VARCHAR(20)
);

CREATE TABLE Usuario (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    contrasena VARCHAR(255) NOT NULL,
    public BOOLEAN NOT NULL DEFAULT TRUE,
    id_persona INTEGER NOT NULL UNIQUE REFERENCES Persona(id) ON DELETE RESTRICT
);

CREATE TABLE Grupo (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    descripcion TEXT,
    publico BOOLEAN NOT NULL DEFAULT TRUE,
    fecha_creacion TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    id_usuario_creador INTEGER NOT NULL REFERENCES Usuario(id) ON DELETE CASCADE
);

-- -----------------------------------------------------------------------------
-- 3. Catálogo de objetos
-- -----------------------------------------------------------------------------
CREATE TABLE Ingrediente (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    id_tipo_ingrediente INTEGER REFERENCES Tipo_Ingrediente(id) ON DELETE SET NULL
);

CREATE TABLE Utensilio (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    id_tipo_utensilio INTEGER REFERENCES Tipo_Utensilio(id) ON DELETE SET NULL
);

-- -----------------------------------------------------------------------------
-- 4. Entidad principal
-- -----------------------------------------------------------------------------
CREATE TABLE Receta (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(150) NOT NULL,
    descripcion TEXT,
    raciones INTEGER CHECK (raciones IS NULL OR raciones > 0),
    tiempo VARCHAR(50),
    promedio_puntuacion DECIMAL(3,2) CHECK (
        promedio_puntuacion IS NULL
        OR (promedio_puntuacion >= 1 AND promedio_puntuacion <= 5)
    ),
    dificultad VARCHAR(50),
    imagen TEXT,
    id_usuario_creador INTEGER NOT NULL REFERENCES Usuario(id) ON DELETE CASCADE
);

-- -----------------------------------------------------------------------------
-- 5. Relaciones intermedias
-- -----------------------------------------------------------------------------
CREATE TABLE Paso_Receta (
    id SERIAL PRIMARY KEY,
    id_receta INTEGER NOT NULL REFERENCES Receta(id) ON DELETE CASCADE,
    numero_paso INTEGER NOT NULL CHECK (numero_paso > 0),
    instruccion TEXT NOT NULL,
    UNIQUE (id_receta, numero_paso)
);

CREATE TABLE Favoritos_Usuario (
    id_usuario INTEGER NOT NULL REFERENCES Usuario(id) ON DELETE CASCADE,
    id_receta INTEGER NOT NULL REFERENCES Receta(id) ON DELETE CASCADE,
    fecha_agregado TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id_usuario, id_receta)
);

-- N:M receta ↔ grupo (quitar de un grupo = DELETE aquí; borrar grupo = trigger + CASCADE)
CREATE TABLE Receta_Grupo (
    id_grupo INTEGER NOT NULL REFERENCES Grupo(id) ON DELETE CASCADE,
    id_receta INTEGER NOT NULL REFERENCES Receta(id) ON DELETE CASCADE,
    fecha_guardado TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id_grupo, id_receta)
);

CREATE TABLE Seguidor_Grupo (
    id_usuario INTEGER NOT NULL REFERENCES Usuario(id) ON DELETE CASCADE,
    id_grupo INTEGER NOT NULL REFERENCES Grupo(id) ON DELETE CASCADE,
    fecha_seguido TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id_usuario, id_grupo)
);

CREATE TABLE Ingrediente_Receta (
    id SERIAL PRIMARY KEY,
    id_receta INTEGER NOT NULL REFERENCES Receta(id) ON DELETE CASCADE,
    id_ingrediente INTEGER NOT NULL REFERENCES Ingrediente(id) ON DELETE RESTRICT,
    cantidad VARCHAR(50),
    UNIQUE (id_receta, id_ingrediente)
);

CREATE TABLE Utensilio_Receta (
    id SERIAL PRIMARY KEY,
    id_receta INTEGER NOT NULL REFERENCES Receta(id) ON DELETE CASCADE,
    id_utensilio INTEGER NOT NULL REFERENCES Utensilio(id) ON DELETE RESTRICT,
    cantidad VARCHAR(50),
    UNIQUE (id_receta, id_utensilio)
);

CREATE TABLE Puntuacion_Receta (
    id SERIAL PRIMARY KEY,
    puntuacion INTEGER NOT NULL CHECK (puntuacion >= 1 AND puntuacion <= 5),
    comentario TEXT,
    id_usuario INTEGER NOT NULL REFERENCES Usuario(id) ON DELETE CASCADE,
    id_receta INTEGER NOT NULL REFERENCES Receta(id) ON DELETE CASCADE,
    UNIQUE (id_usuario, id_receta)
);

CREATE TABLE Logro (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    descripcion TEXT
);

CREATE TABLE Usuario_Logro (
    id_usuario INTEGER NOT NULL REFERENCES Usuario(id) ON DELETE CASCADE,
    id_logro INTEGER NOT NULL REFERENCES Logro(id) ON DELETE CASCADE,
    fecha_obtenido TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id_usuario, id_logro)
);

-- -----------------------------------------------------------------------------
-- 6. Índices (listados alfabéticos y búsquedas frecuentes)
-- -----------------------------------------------------------------------------
CREATE INDEX idx_persona_correo_lower ON Persona (LOWER(correo));
CREATE INDEX idx_usuario_username_lower ON Usuario (LOWER(username));
CREATE INDEX idx_receta_nombre_lower ON Receta (LOWER(nombre));
CREATE INDEX idx_receta_usuario_creador ON Receta (id_usuario_creador);
CREATE INDEX idx_grupo_nombre_lower ON Grupo (LOWER(nombre));
CREATE INDEX idx_grupo_usuario_creador ON Grupo (id_usuario_creador);
CREATE INDEX idx_receta_grupo_grupo ON Receta_Grupo (id_grupo);
CREATE INDEX idx_receta_grupo_receta ON Receta_Grupo (id_receta);
CREATE INDEX idx_ingrediente_nombre_lower ON Ingrediente (LOWER(nombre));
CREATE INDEX idx_utensilio_nombre_lower ON Utensilio (LOWER(nombre));
CREATE INDEX idx_logro_nombre_lower ON Logro (LOWER(nombre));
CREATE INDEX idx_usuario_logro_usuario ON Usuario_Logro (id_usuario);
CREATE INDEX idx_usuario_logro_logro ON Usuario_Logro (id_logro);

-- -----------------------------------------------------------------------------
-- 7. Funciones y triggers
-- -----------------------------------------------------------------------------

-- Al borrar un grupo: eliminar solo las recetas del dueño del grupo vinculadas en Receta_Grupo.
CREATE OR REPLACE FUNCTION fn_borrar_recetas_al_eliminar_grupo()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
BEGIN
    DELETE FROM Receta
    WHERE id IN (
        SELECT rg.id_receta
        FROM Receta_Grupo rg
        INNER JOIN Receta r ON r.id = rg.id_receta
        WHERE rg.id_grupo = OLD.id
          AND r.id_usuario_creador = OLD.id_usuario_creador
    );
    RETURN OLD;
END;
$$;

CREATE TRIGGER trg_grupo_borra_recetas_afiliadas
    BEFORE DELETE ON Grupo
    FOR EACH ROW
    EXECUTE PROCEDURE fn_borrar_recetas_al_eliminar_grupo();

-- Al borrar un usuario: eliminar la persona asociada (1:1 usuario–persona).
CREATE OR REPLACE FUNCTION fn_borrar_persona_al_eliminar_usuario()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
BEGIN
    DELETE FROM Persona WHERE id = OLD.id_persona;
    RETURN OLD;
END;
$$;

CREATE TRIGGER trg_usuario_borra_persona
    AFTER DELETE ON Usuario
    FOR EACH ROW
    EXECUTE PROCEDURE fn_borrar_persona_al_eliminar_usuario();

-- Mantener promedio_puntuacion de la receta al insertar/actualizar/borrar puntuaciones.
CREATE OR REPLACE FUNCTION fn_actualizar_promedio_receta()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
DECLARE
    v_id_receta INTEGER;
BEGIN
    v_id_receta := COALESCE(NEW.id_receta, OLD.id_receta);

    UPDATE Receta
    SET promedio_puntuacion = (
        SELECT ROUND(AVG(puntuacion)::numeric, 2)
        FROM Puntuacion_Receta
        WHERE id_receta = v_id_receta
    )
    WHERE id = v_id_receta;

    RETURN COALESCE(NEW, OLD);
END;
$$;

CREATE TRIGGER trg_puntuacion_actualiza_promedio
    AFTER INSERT OR UPDATE OR DELETE ON Puntuacion_Receta
    FOR EACH ROW
    EXECUTE PROCEDURE fn_actualizar_promedio_receta();
