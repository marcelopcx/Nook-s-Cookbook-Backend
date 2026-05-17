//! Carga `.env` al compilar para que `sqlx::query!` / `query_as!` vean `DATABASE_URL`
//! (p. ej. Cargo Check del IDE). Con `.sqlx/` generado (`cargo sqlx prepare`), activa offline.

fn main() {
    dotenvy::dotenv().ok();

    let sqlx_dir = std::path::Path::new(".sqlx");
    let cache_ready = sqlx_dir.is_dir()
        && std::fs::read_dir(sqlx_dir)
            .map(|mut entries| entries.next().is_some())
            .unwrap_or(false);
    if cache_ready {
        println!("cargo:rustc-env=SQLX_OFFLINE=true");
    }
}
