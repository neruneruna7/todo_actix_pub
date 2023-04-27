/// Todoアプリのバックエンド部分です
use actix_web::{
    http::StatusCode,
    middleware::{ErrorHandlers, Logger},
    web, App, HttpServer,
};

// api.rs, db.rs, model.rsを展開します
mod api;
mod db;
mod model;

// 本プログラムのエントリーポイントです
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ログ出力の初期化を行います
    // デフォルトの環境変数を使用してフィルターを読み込みます
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // ログ出力を行います
    log::info!("starting HTTP server at http://localhost:8080");

    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url).await.unwrap();

    HttpServer::new(move || {
        let error_handler = ErrorHandlers::new()
            .default_handler(api::default_error_handle)
            .handler(
                StatusCode::INTERNAL_SERVER_ERROR,
                api::internal_server_error,
            )
            .handler(StatusCode::NOT_FOUND, api::not_found)
            .handler(StatusCode::BAD_REQUEST, api::bad_reqest);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .wrap(error_handler)
            .service(api::index)
            .service(api::create_todo)
            .service(api::update)
            .service(api::delete)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
