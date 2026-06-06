-- =============================================================================
-- Logros iniciales — Nook's Cookbook
-- Ejecutar DESPUÉS de nooks_cookbook.sql
-- =============================================================================

BEGIN;

SET search_path TO nooks_cookbook;

DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM logro LIMIT 1) THEN
        RAISE NOTICE 'Los logros ya están cargados. Seed omitido.';
        RETURN;
    END IF;

    INSERT INTO logro (nombre, descripcion) VALUES
        (
            'Primeros Pasos',
            'Crear la primera receta propia en la aplicación.'
        ),
        (
            'Aprendiz de Cocina',
            'Crear un total de 10 recetas propias.'
        ),
        (
            'Maestro Culinario',
            'Crear un total de 100 recetas propias.'
        ),
        (
            'Miembro Fundador',
            'Crear el primer grupo o comunidad culinaria.'
        ),
        (
            'El Gran Banquete',
            'Guardar una receta que tenga simultáneamente más de 15 ingredientes, más de 20 pasos de preparación y un tiempo de elaboración mayor o igual a 120 minutos.'
        ),
        (
            'Guardado en Memoria',
            'Añadir cualquier receta a la sección de "Mis Recetas" (Favoritos) por primera vez.'
        ),
        (
            '¿La niña esta triste 🏳️‍🌈?',
            'Desactivar la música de fondo o los efectos de sonido en la configuración de la aplicación.'
        ),
        (
            'Calabaza, Calabaza...',
            'Iniciar el proceso para eliminar permanentemente la cuenta de usuario.'
        );

    RAISE NOTICE 'Seed completado: % logros.', (SELECT COUNT(*) FROM logro);
END $$;

COMMIT;

SELECT id, nombre, descripcion FROM logro ORDER BY id;
