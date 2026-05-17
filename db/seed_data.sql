-- =============================================================================
-- Datos iniciales — Nook's Cookbook
-- Ejecutar DESPUÉS de nooks_cookbook.sql
-- =============================================================================
-- Solo para poblar catálogo base
--   • Tipos maestros (tipo_ingrediente, tipo_utensilio)
--   • Ingredientes y utensilios en volumen
-- =============================================================================

BEGIN;

SET search_path TO nooks_cookbook;

DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM ingrediente LIMIT 1) THEN
        RAISE NOTICE 'El catálogo de ingredientes ya tiene datos. Seed omitido.';
        RETURN;
    END IF;

    -- -------------------------------------------------------------------------
    -- Tipos maestros — ingredientes
    -- -------------------------------------------------------------------------
    INSERT INTO tipo_ingrediente (nombre) VALUES
        ('Vegetal'),
        ('Fruta'),
        ('Proteína'),
        ('Lácteo'),
        ('Marisco'),
        ('Pescado'),
        ('Cereal'),
        ('Legumbre'),
        ('Especia y hierba'),
        ('Endulzante'),
        ('Aceite y grasa'),
        ('Bebida'),
        ('Otro')
    ON CONFLICT (nombre) DO NOTHING;

    -- -------------------------------------------------------------------------
    -- Tipos maestros — utensilios
    -- -------------------------------------------------------------------------
    INSERT INTO tipo_utensilio (nombre) VALUES
        ('Cocina'),
        ('Horno'),
        ('Repostería'),
        ('Preparación'),
        ('Medición'),
        ('Vajilla y servicio'),
        ('Corte'),
        ('Otro')
    ON CONFLICT (nombre) DO NOTHING;

    -- -------------------------------------------------------------------------
    -- Ingredientes (catálogo amplio — temática isla / cocina)
    -- -------------------------------------------------------------------------
    INSERT INTO ingrediente (nombre, id_tipo_ingrediente)
    SELECT v.nombre, t.id
    FROM (
        VALUES
            -- Vegetales
            ('Champiñón plano', 'Vegetal'),
            ('Champiñón redondo', 'Vegetal'),
            ('Champiñón raro', 'Vegetal'),
            ('Bambú tierno', 'Vegetal'),
            ('Brote de bambú', 'Vegetal'),
            ('Zanahoria', 'Vegetal'),
            ('Patata', 'Vegetal'),
            ('Tomate', 'Vegetal'),
            ('Calabaza', 'Vegetal'),
            ('Calabaza verde', 'Vegetal'),
            ('Calabaza amarilla', 'Vegetal'),
            ('Calabaza naranja', 'Vegetal'),
            ('Calabaza blanca', 'Vegetal'),
            ('Puerro', 'Vegetal'),
            ('Champiñón elegante', 'Vegetal'),
            ('Hongo común', 'Vegetal'),
            ('Apio', 'Vegetal'),
            ('Lechuga', 'Vegetal'),
            ('Cebolla', 'Vegetal'),
            ('Pimiento', 'Vegetal'),
            ('Maíz', 'Vegetal'),
            ('Papa', 'Vegetal'),
            ('Remolacha', 'Vegetal'),
            ('Col', 'Vegetal'),
            ('Berenjena', 'Vegetal'),
            ('Pepino', 'Vegetal'),
            -- Frutas
            ('Manzana', 'Fruta'),
            ('Naranja', 'Fruta'),
            ('Cereza', 'Fruta'),
            ('Durazno', 'Fruta'),
            ('Pera', 'Fruta'),
            ('Coco', 'Fruta'),
            ('Plátano', 'Fruta'),
            ('Sandía', 'Fruta'),
            ('Melón', 'Fruta'),
            ('Uva', 'Fruta'),
            ('Mandarina', 'Fruta'),
            ('Limón', 'Fruta'),
            ('Frambuesa', 'Fruta'),
            ('Ciruela', 'Fruta'),
            ('Kiwi', 'Fruta'),
            ('Mango', 'Fruta'),
            -- Proteínas
            ('Huevo', 'Proteína'),
            ('Carne cruda', 'Proteína'),
            ('Pollo crudo', 'Proteína'),
            ('Jamón', 'Proteína'),
            ('Tocino', 'Proteína'),
            ('Carne de caza', 'Proteína'),
            ('Muslo de pollo', 'Proteína'),
            ('Ala de pollo', 'Proteína'),
            -- Lácteos
            ('Leche', 'Lácteo'),
            ('Queso', 'Lácteo'),
            ('Mantequilla', 'Lácteo'),
            ('Yogur', 'Lácteo'),
            ('Nata', 'Lácteo'),
            ('Crema de leche', 'Lácteo'),
            -- Mariscos y pescados
            ('Mejillón', 'Marisco'),
            ('Almeja', 'Marisco'),
            ('Ostra', 'Marisco'),
            ('Cangrejo', 'Marisco'),
            ('Langosta', 'Marisco'),
            ('Camarón', 'Marisco'),
            ('Vieira', 'Marisco'),
            ('Pulpo', 'Marisco'),
            ('Sardina', 'Pescado'),
            ('Caballa', 'Pescado'),
            ('Dorada', 'Pescado'),
            ('Lubina', 'Pescado'),
            ('Salmón', 'Pescado'),
            ('Atún', 'Pescado'),
            ('Pez lucio', 'Pescado'),
            ('Carpa', 'Pescado'),
            ('Perca', 'Pescado'),
            ('Pez payaso', 'Pescado'),
            -- Cereales y legumbres
            ('Trigo', 'Cereal'),
            ('Harina de trigo', 'Cereal'),
            ('Arroz', 'Cereal'),
            ('Pan', 'Cereal'),
            ('Fideos', 'Cereal'),
            ('Azúcar moreno', 'Cereal'),
            ('Harina', 'Cereal'),
            ('Maicena', 'Cereal'),
            ('Frijol', 'Legumbre'),
            ('Guisante', 'Legumbre'),
            ('Soja', 'Legumbre'),
            -- Especias, endulzantes, otros
            ('Trufa', 'Otro'),
            ('Azúcar', 'Endulzante'),
            ('Miel', 'Endulzante'),
            ('Sal', 'Especia y hierba'),
            ('Pimienta', 'Especia y hierba'),
            ('Albahaca', 'Especia y hierba'),
            ('Menta', 'Especia y hierba'),
            ('Canela', 'Especia y hierba'),
            ('Vainilla', 'Especia y hierba'),
            ('Orégano', 'Especia y hierba'),
            ('Aceite de oliva', 'Aceite y grasa'),
            ('Aceite vegetal', 'Aceite y grasa'),
            ('Vinagre', 'Bebida'),
            ('Agua', 'Bebida'),
            ('Café en grano', 'Bebida'),
            ('Té', 'Bebida'),
            ('Chocolate', 'Otro'),
            ('Cacao en polvo', 'Otro'),
            ('Patata dulce', 'Vegetal'),
            ('Seta', 'Vegetal'),
            ('Algas', 'Otro'),
            ('Algas marinas', 'Otro'),
            ('Harina de arroz', 'Cereal'),
            ('Levadura', 'Otro'),
            ('Gelatina', 'Otro')
    ) AS v(nombre, tipo)
    INNER JOIN tipo_ingrediente t ON t.nombre = v.tipo;

    -- -------------------------------------------------------------------------
    -- Utensilios (catálogo amplio)
    -- -------------------------------------------------------------------------
    INSERT INTO utensilio (nombre, id_tipo_utensilio)
    SELECT v.nombre, t.id
    FROM (
        VALUES
            -- Cocina
            ('Sartén de cocina', 'Cocina'),
            ('Olla de cocina', 'Cocina'),
            ('Cacerola', 'Cocina'),
            ('Cazo', 'Cocina'),
            ('Wok', 'Cocina'),
            ('Plancha', 'Cocina'),
            ('Paellera', 'Cocina'),
            ('Vaporera', 'Cocina'),
            ('Sartén antiadherente', 'Cocina'),
            ('Olla a presión', 'Cocina'),
            -- Horno
            ('Bandeja de horno', 'Horno'),
            ('Molde para hornear', 'Horno'),
            ('Molde para pan', 'Horno'),
            ('Molde para muffins', 'Horno'),
            ('Molde para tarta', 'Horno'),
            ('Piedra para pizza', 'Horno'),
            ('Brocheta de horno', 'Horno'),
            -- Repostería
            ('Batidora', 'Repostería'),
            ('Rodillo', 'Repostería'),
            ('Manga pastelera', 'Repostería'),
            ('Espátula de repostería', 'Repostería'),
            ('Molde para galletas', 'Repostería'),
            ('Tamiz', 'Repostería'),
            ('Cortador de galletas', 'Repostería'),
            -- Preparación
            ('Tabla de cortar', 'Preparación'),
            ('Bol de mezclar', 'Preparación'),
            ('Colador', 'Preparación'),
            ('Espumadera', 'Preparación'),
            ('Cuchara de madera', 'Preparación'),
            ('Rallador', 'Preparación'),
            ('Pelador', 'Preparación'),
            ('Mortero', 'Preparación'),
            ('Batidor de varillas', 'Preparación'),
            ('Abrelatas', 'Preparación'),
            ('Exprimidor', 'Preparación'),
            ('Embudo', 'Preparación'),
            ('Pinzas de cocina', 'Preparación'),
            ('Espátula plana', 'Preparación'),
            ('Cuchillo de chef', 'Preparación'),
            -- Medición
            ('Taza medidora', 'Medición'),
            ('Cuchara medidora', 'Medición'),
            ('Balanza de cocina', 'Medición'),
            ('Jarra medidora', 'Medición'),
            -- Corte
            ('Cuchillo de pan', 'Corte'),
            ('Cuchillo para filetear', 'Corte'),
            ('Tijeras de cocina', 'Corte'),
            ('Mandolina', 'Corte'),
            -- Vajilla y servicio
            ('Plato hondo', 'Vajilla y servicio'),
            ('Plato llano', 'Vajilla y servicio'),
            ('Cuenco', 'Vajilla y servicio'),
            ('Taza', 'Vajilla y servicio'),
            ('Vaso', 'Vajilla y servicio'),
            ('Fuente para servir', 'Vajilla y servicio'),
            ('Cuchara de servir', 'Vajilla y servicio'),
            ('Tenaza para servir', 'Vajilla y servicio'),
            -- Otro
            ('Termómetro de cocina', 'Otro'),
            ('Guantes de horno', 'Otro'),
            ('Mante individual', 'Otro'),
            ('Papel de horno', 'Otro'),
            ('Film transparente', 'Otro')
    ) AS v(nombre, tipo)
    INNER JOIN tipo_utensilio t ON t.nombre = v.tipo;

    RAISE NOTICE 'Seed completado: tipos maestros, % ingredientes, % utensilios.',
        (SELECT COUNT(*) FROM ingrediente),
        (SELECT COUNT(*) FROM utensilio);
END $$;

COMMIT;

-- Comprobar
SELECT 'tipo_ingrediente' AS tabla, COUNT(*) AS total FROM tipo_ingrediente
UNION ALL SELECT 'tipo_utensilio', COUNT(*) FROM tipo_utensilio
UNION ALL SELECT 'ingrediente', COUNT(*) FROM ingrediente
UNION ALL SELECT 'utensilio', COUNT(*) FROM utensilio
ORDER BY tabla;
