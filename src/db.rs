/// mysqlとの接続を司るAPIを提供します
use anyhow::anyhow;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

use crate::model::{NewTask, Task};

// このモジュール内で使うリザルト型
// anyhow::Resultのエイリアス
type DbResult<T> = anyhow::Result<T>;
// DBとのコネクションを確立
#[allow(dead_code)]
pub async fn init_pool(database_url: &str) -> DbResult<MySqlPool> {
    MySqlPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(1))
        .connect(database_url)
        .await
        .map_err(|e| anyhow!(e))
}

// すべてのタスクを読み込む
#[allow(dead_code)]
pub async fn get_all_tasks(pool: &MySqlPool) -> DbResult<Vec<Task>> {
    Task::get_all(pool)
        .await
        .map_err(|_| anyhow!("failed to get all tasks タスクの取得に失敗しました"))
}

// タスクを挿入する
#[allow(dead_code)]
pub async fn create_task(pool: &MySqlPool, new_task: NewTask) -> DbResult<()> {
    // let new_task = NewTask::new(title, descreption);
    Task::insert(pool, new_task)
        .await
        .map_err(|_| anyhow!("failed to insert task データの挿入に失敗しました"))
}

// タスクの状態を切り替える
#[allow(dead_code)]
pub async fn toggle_task_with_id(pool: &MySqlPool, id: u64) -> DbResult<()> {
    Task::toggle_with_id(pool, id).await.map_err(|_| {
        anyhow!(
            "failed to toggle task with id {} 指定されたIDのタスクの状態を切り替えることができませんでした",
            id
        )
    })
}

// タスクを削除する
#[allow(dead_code)]
pub async fn delete_task_with_id(pool: &MySqlPool, id: u64) -> DbResult<()> {
    Task::delete_with_id(pool, id).await.map_err(|_| {
        anyhow!(
            "failed to delete task with id {} 指定されたIDのタスクを削除できませんでした",
            id
        )
    })
}
