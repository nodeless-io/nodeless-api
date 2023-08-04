use actix_web::{get, post, web::{self, Data}, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use handlers::frontend::fe_auth_handlers::configure_routes;
use repositories::user_repository::UserRepository;
use sqlx::PgPool;

pub mod handlers;
pub mod models;
pub mod repositories;
pub mod services;
pub mod middleware;
pub mod helpers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = PgPool::connect(dotenvy::var("DATABASE_URL").unwrap().as_str()).await.unwrap();
    let user_repo = UserRepository::new(pool);

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(user_repo.clone()))
            .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(handlers::frontend::fe_auth_handlers::configure_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}