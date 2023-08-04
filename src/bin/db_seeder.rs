use nodeless_api::helpers::crypto::sha256_hmac;
use sqlx::PgPool;
use uuid::Uuid;

#[tokio::main]
pub async fn main() {
    let pool = PgPool::connect(dotenvy::var("DATABASE_URL").unwrap().as_str())
        .await
        .unwrap();
    run(&pool).await.unwrap();
}

pub async fn run(pool: &PgPool) -> Result<(), sqlx::Error> {
    seed_users(pool).await?;
    Ok(())
}

async fn seed_users(pool: &PgPool) -> Result<(), sqlx::Error> {
    println!("Seeding users...");

    // Seed an example user
    let uuid = Uuid::new_v4().to_string();
    let hmac = dotenvy::var("APP_KEY").unwrap();
    let result = sqlx::query!(
        r#"
        INSERT INTO users (uuid, email, password)
        VALUES ($1, $2, $3)
        "#,
        uuid, // UUID generation using the `uuid` crate
        "admin@nodeless.io",
        sha256_hmac("password", &hmac), // In a real-world scenario, hash the password!
    )
    .execute(pool)
    .await?;

    println!("User seeded: {:?}", result);

    Ok(())
}

// Add other seed functions as needed, like:
// async fn seed_products(pool: &PgPool) -> Result<(), sqlx::Error> { ... }
// async fn seed_orders(pool: &PgPool) -> Result<(), sqlx::Error> { ... }
// ...
