# 🍳 Nook's Cookbook (Backend)

¡Bienvenido a **Nook's Cookbook**! Este proyecto es la API REST del ecosistema de recetas inspirada y tematizada con el universo de *Animal Crossing*. El desarrollo forma parte de una asignación práctica para el curso de **Desarrollo de Aplicaciones Móviles** en la **Universidad Rafael Urdaneta (URU)**.

La aplicación cuenta con una arquitectura moderna desacoplada:
* Desarrollado en **Rust** con **[Actix Web](https://actix.rs/)** como servidor HTTP.
* Persistencia en **PostgreSQL** mediante **[SQLx](https://github.com/launchbadge/sqlx)** (consultas verificadas en tiempo de compilación).
* Autenticación con **JWT** (Bearer) y contraseñas hasheadas con **bcrypt**.
* El cliente móvil se encuentra en el repositorio del frontend: **[Nook's Cookbook — Frontend](https://github.com/marcelopcx/Nook-s-Cookbook-Frontend)** (React Native + Expo).

---

## 🚀 Guía de Inicialización del Proyecto

Sigue estos pasos para clonar, configurar y ejecutar el backend localmente en tu entorno de desarrollo.

### 📋 Prerrequisitos

Asegúrate de tener instalado lo siguiente en tu sistema:

1.  **[Rust](https://www.rust-lang.org/tools/install)** (toolchain *stable*).
2.  **[Docker](https://www.docker.com/)** y **Docker Compose** (para PostgreSQL).
---

## 🦀 Configuración del Backend (Rust + Actix + PostgreSQL)

1.  **Navega al directorio del backend:**
    ```bash
    cd backend
    ```

2.  **Da permisos de ejecución a los scripts** *(solo la primera vez)*:
    ```bash
    chmod +x scripts/*.sh
    ```

3.  **Inicializa el entorno completo** (`.env`, Docker, esquema y datos semilla):
    ```bash
    make setup
    ```
    Este comando realiza automáticamente:
    * Crea `.env` desde `.env.example` si no existe.
    * Levanta PostgreSQL con Docker Compose.
    * Aplica el esquema `nooks_cookbook` (tablas, índices y triggers).
    * Carga el catálogo inicial (`db/seed_data.sql`: tipos, ~100 ingredientes, ~55 utensilios).

4.  **Configura las Variables de Entorno (`.env`):**
    Si necesitás ajustar credenciales, editá el archivo `.env` en la raíz de `backend/`. Los valores por defecto están alineados con `docker-compose.yml`:
    ```env
    DATABASE_URL=postgres://nooks:secret123@127.0.0.1:5432/nooks_cookbook?options=-csearch_path%3Dnooks_cookbook
    JWT_SECRET=un_secreto_largo_minimo_32_caracteres_cambiar_en_produccion
    JWT_EXPIRATION_HOURS=24
    HOST=0.0.0.0
    PORT=8080
    ```
    *Nota: No subas `.env` a git. Usá `.env.example` como plantilla.*

5.  **Inicia el servidor de la API:**
    ```bash
    cargo run
    ```
    El servidor quedará disponible en **`http://TU_IP_LOCAL:8080`** y también aceptará conexiones desde otros dispositivos de la red porque escucha en **`0.0.0.0`**.

6.  **Comprueba que todo funciona:**
    ```bash
    curl http://TU_IP_LOCAL:8080/health
    ```
    Deberías recibir un mensaje temático de Animal Crossing confirmando que los servicios están en línea.

### Alternativa sin Make

```bash
./scripts/setup.sh
cargo run
```

---


## 📂 Arquitectura de Carpetas

El backend está organizado en capas para separar rutas, lógica de negocio y acceso a datos:

* `src/main.rs` — Punto de entrada: pool de conexiones, configuración y arranque de **HttpServer**.
* `src/lib.rs` — Módulos públicos del crate (`auth`, `config`, `db`, `handlers`, `models`, `routes`, `services`, `error`).
* `src/config/` — Carga de variables de entorno (`AppConfig`).
* `src/db/` — Creación del pool de PostgreSQL.
* `src/auth/` — Extractor `AuthenticatedUser` (validación JWT en rutas protegidas).
* `src/handlers/` — Controladores HTTP (`health`, `auth`).
* `src/services/` — Lógica de negocio y consultas SQLx (`auth`: login, registro, perfil).
* `src/models/` — Structs de request/response y filas de base de datos.
* `src/routes/` — Registro de servicios Actix (`configure`).
* `src/error/` — Errores de API unificados (`ApiError`).
* `db/` — Esquema SQL y datos semilla.
* `scripts/` — `setup.sh` y `reset-db.sh` para automatizar la BD.
* `api/` — Colección [Bruno](https://www.usebruno.com/) para probar los endpoints.
* `docker-compose.yml` — PostgreSQL 16 en contenedor.
* `build.rs` — Carga `.env` al compilar; activa **SQLx offline** si existe `.sqlx/`.

---

## 📱 Conexión con el Frontend (Expo)

Repositorio del cliente: **[Nook-s-Cookbook-Frontend](https://github.com/marcelopcx/Nook-s-Cookbook-Frontend)**

Clona y configura el frontend siguiendo su README. Luego, en el `.env` del proyecto Expo, apunta la URL de la API a tu máquina usando la **IP local** (no `localhost`) para que el dispositivo móvil pueda alcanzar este backend. Como el backend escucha en `0.0.0.0`, cualquier dispositivo en la misma red podrá conectarse usando esa IP:

```env
EXPO_PUBLIC_API_URL=http://TU_IP_LOCAL:8080
```

*Nota: El backend escucha en el puerto **8080** por defecto (`PORT` en `.env`). Asegurate de que coincida con la variable del frontend.*
