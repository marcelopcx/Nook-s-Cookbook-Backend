-- Migración: corregir borrado de recetas al eliminar grupo + tablas Logro y Usuario_Logro
-- Aplicar en bases de datos que ya tienen el esquema inicial.

SET search_path TO nooks_cookbook;

-- -----------------------------------------------------------------------------
-- 1. Corregir trigger: solo borrar recetas del dueño del grupo
-- -----------------------------------------------------------------------------
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

-- -----------------------------------------------------------------------------
-- 2. Nuevas entidades: Logro y Usuario_Logro
-- -----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS Logro (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    descripcion TEXT
);

CREATE TABLE IF NOT EXISTS Usuario_Logro (
    id_usuario INTEGER NOT NULL REFERENCES Usuario(id) ON DELETE CASCADE,
    id_logro INTEGER NOT NULL REFERENCES Logro(id) ON DELETE CASCADE,
    fecha_obtenido TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id_usuario, id_logro)
);

CREATE INDEX IF NOT EXISTS idx_logro_nombre_lower ON Logro (LOWER(nombre));
CREATE INDEX IF NOT EXISTS idx_usuario_logro_usuario ON Usuario_Logro (id_usuario);
CREATE INDEX IF NOT EXISTS idx_usuario_logro_logro ON Usuario_Logro (id_logro);
