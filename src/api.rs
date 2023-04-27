
use actix_web::middleware::{ErrorHandlerResponse};
use actix_web::{dev, error, get, post, web, HttpResponse, Responder, Result};

use sqlx::MySqlPool;

use crate::{db, model};

// ルートパス
#[get("/")]
pub async fn index(pool: web::Data<MySqlPool>) -> Result<impl Responder> {
    let tasks = db::get_all_tasks(&pool)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(tasks))
}

// postされたTaskをDBに格納する
#[post("/todo/insert/")]
pub async fn create_todo(
    pool: web::Data<MySqlPool>,
    new_task: web::Json<model::NewTask>,
) -> Result<impl Responder> {
    if new_task.title.is_empty() {
        Ok(HttpResponse::Ok().body("title is empty"))
        // Ok(web::Redirect::to("/").using_status_code(StatusCode::FOUND))
    } else {
        db::create_task(&pool, new_task.into_inner())
            .await
            .map_err(error::ErrorInternalServerError)?;
        Ok(HttpResponse::Ok().body("ok todo insert completed!"))
        // Ok(web::Redirect::to("/").using_status_code(StatusCode::FOUND))
    }
}

// 指定されたタスクの状態を切り替える
#[get("/todo/toggle/{id}")]
pub async fn update(pool: web::Data<MySqlPool>, id: web::Path<u64>) -> Result<impl Responder> {
    db::toggle_task_with_id(&pool, *id)
        .await
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(format!("todo_Update! {}", id)))
}

// 指定されたタスクを削除する
#[get("/todo/delete/{id}")]
pub async fn delete(pool: web::Data<MySqlPool>, id: web::Path<u64>) -> Result<impl Responder> {
    db::delete_task_with_id(&pool, *id)
        .await
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(format!("todo_Delete! {}", id)))
}

// デフォルトのエラーハンドラー
pub fn default_error_handle<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let error_message = "DEFAULT ERROR";

    let (req, res) = res.into_parts();

    let les = res.set_body(error_message).map_into_boxed_body();

    let res = dev::ServiceResponse::new(req, les).map_into_right_body();
    Ok(ErrorHandlerResponse::Response(res))
}

// 500エラーのエラーハンドラー
pub fn internal_server_error<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let error_message = "INTERNAL SERVER ERROR";

    let (req, res) = res.into_parts();

    let les = res.set_body(error_message).map_into_boxed_body();

    let res = dev::ServiceResponse::new(req, les).map_into_right_body();
    Ok(ErrorHandlerResponse::Response(res))
}

// 404エラーのエラーハンドラー
pub fn not_found<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let error_message = "NOT FOUND";

    let (req, res) = res.into_parts();

    let les = res.set_body(error_message).map_into_boxed_body();

    let res = dev::ServiceResponse::new(req, les).map_into_right_body();
    Ok(ErrorHandlerResponse::Response(res))
}

// 400エラーのエラーハンドラー
pub fn bad_reqest<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let error_message = "BAD REQUEST";

    let (req, res) = res.into_parts();

    let les = res.set_body(error_message).map_into_boxed_body();

    let res = dev::ServiceResponse::new(req, les).map_into_right_body();
    Ok(ErrorHandlerResponse::Response(res))
}

/*
// すべてのTodoをjsonで返す関数
#[get("/todos")]
pub async fn get_todos() -> impl Responder {
    // 例としてのデータ
    let todos = vec![
        Todo {
            id: 1,
            title: "title1".to_string(),
            description: "description1".to_string(),
            completed: false,
        },
        Todo {
            id: 2,
            title: "title2".to_string(),
            description: "description2".to_string(),
            completed: true,
        },
    ];
    HttpResponse::Ok().json(todos)
}

// 受け取ったTodoをjsonで返す関数
// DBに保存する処理に変更する
#[post("/todos")]
pub async fn post_todos(todo: web::Json<Todo>) -> impl Responder {
    HttpResponse::Ok().json(todo.0)
}

// 完了したタスクのIDリストを配列で受け取り、そのタスクを完了にする関数
// DBを更新する処理に変更する
#[post("/todos/complete")]
pub async fn complete_todos(ids: web::Json<Vec<i32>>) -> impl Responder {
    HttpResponse::Ok().json(ids.0)
}

// 未完了したタスクのIDリストを配列で受け取り、そのタスクを未完了にする関数
// DBを更新する処理に変更する
#[post("/todos/uncomplete")]
pub async fn uncomplete_todos(ids: web::Json<Vec<i32>>) -> impl Responder {
    HttpResponse::Ok().json(ids.0)
}

// 削除するタスクのIDリストを配列で受け取り、そのタスクを削除する関数
// DBを更新する処理に変更する
#[post("/todos/delete")]
pub async fn delete_todos(ids: web::Json<Vec<i32>>) -> impl Responder {
    HttpResponse::Ok().json(ids.0)
}
*/
