use sqlx::PgPool;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}
