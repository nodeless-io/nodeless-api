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
    let result = sqlx::query!(
        r#"
        INSERT INTO users (uuid, email, password)
        VALUES ($1, $2, $3)
        "#,
        uuid,
        "admin@nodeless.io",
        "87e7bd0a19e98040f9668b619e3c2c33da19caa475a1928ff19a113102214e8c", // password
    )
    .execute(pool)
    .await?;

    println!("User seeded: {:?}", result);

    Ok(())
}
