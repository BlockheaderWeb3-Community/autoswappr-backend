use axum::{
    body::Body,
    http::{Request, Response},
    Router,
};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::sync::Once;
use tower::ServiceExt;
use uuid::Uuid;

use autoswappr_backend::{router, telemetry, Configuration, Db};

static TRACING: Once = Once::new();

pub struct TestApp {
    pub router: Router,
    pub db: Db,
}

impl TestApp {
    pub async fn new() -> Self {
        dotenvy::dotenv().ok();
        std::env::set_var("PORT", "0");
        TRACING.call_once(telemetry::setup_tracing);
        let config = Configuration::new();
        let db_str = create_test_db(&config.db_str).await;
        let db = Db::new(&db_str, config.db_pool_max_size)
            .await
            .expect("Failed to Initialize DB.");
        tracing::debug!("Running migrations");
        db.migrate().await.expect("Failed to run migrations");
        let router = router(config, db.clone());
        Self { db, router }
    }

    pub async fn request(&self, req: Request<Body>) -> Response<Body> {
        self.router.clone().oneshot(req).await.unwrap()
    }
}

pub async fn create_test_db(db_str: &str) -> String {
    let db_name =
        std::env::var("DATABASE_NAME").expect("DATABASE_NAME environment variable not specified.");
    let db_str = db_str
        .strip_suffix(&db_name)
        .expect("Failed to remove DB name from connection string");
    let random_db_name = Uuid::now_v7().to_string();
    let mut conn = PgConnection::connect(&db_str)
        .await
        .expect("Failed to connect to Postgres.");
    conn.execute(format!(r#"CREATE DATABASE "{}";"#, random_db_name).as_str())
        .await
        .expect("Failed to create test DB.");
    db_str.to_owned()
}

pub async fn clean_database(pool: &PgPool) {
    let _ = sqlx::query!("SELECT COUNT(*) FROM swap_subscription")
        .fetch_one(pool)
        .await
        .unwrap_or_else(|_| panic!("Database tables not ready"));

    sqlx::query!("DELETE FROM swap_subscription_from_token")
        .execute(pool)
        .await
        .unwrap();
    sqlx::query!("DELETE FROM swap_subscription")
        .execute(pool)
        .await
        .unwrap();

    let count = sqlx::query!("SELECT COUNT(*) as count FROM swap_subscription")
        .fetch_one(pool)
        .await
        .unwrap();

    println!("Database cleaned. Subscription count: {:?}", count.count);
}
