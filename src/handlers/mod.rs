pub mod auth;
pub mod catalogo;
pub mod favorito;
pub mod grupo;
pub mod health;
pub mod imagen;
pub mod logro;

pub use auth::{
    delete_me, get_me, get_me_recetas, get_usuario_publico, login, patch_me, register,
};
pub use catalogo::{
    actualizar_receta, crear_receta, eliminar_receta, listar_ingredientes, listar_recetas,
    listar_tipos_ingrediente, listar_tipos_utensilio, listar_utensilios, obtener_receta,
};
pub use favorito::{
    actualizar_puntuacion, agregar_favorito, crear_puntuacion, eliminar_puntuacion,
    listar_favoritos, listar_puntuaciones, quitar_favorito,
};
pub use grupo::{
    actualizar_grupo, agregar_receta_grupo, crear_grupo, dejar_seguir_grupo, eliminar_grupo,
    listar_grupos, listar_recetas_grupo, listar_seguidores_grupo, obtener_grupo,
    quitar_receta_grupo, seguir_grupo,
};
pub use health::health_check;
pub use imagen::subir_imagen_receta;
pub use logro::{listar_logros, listar_mis_logros};
