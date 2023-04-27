use anyhow::anyhow;
/// 使用する型をまとめて定義するモジュール
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
// データベース操作のエラーを扱うための型
type TaskResult<T> = Result<T, anyhow::Error>;

// タスク書き込みのデータを渡す構造体
#[derive(Debug, Deserialize)]
pub struct NewTask {
    pub title: String,
    pub description: String,
}

impl NewTask {
    pub fn new(title: String, description: String) -> Self {
        NewTask { title, description }
    }
}

// MysqlのBool型の違いを吸収するための構造体
// mysqlのboolはi8で表現される
// その違いを吸収するために，bool型とi8型を相互変換する処理を実装する
#[derive(Debug, Serialize, Deserialize)]
struct MysqlTask {
    id: u64,
    title: String,
    description: String,
    completed: i8,
}

impl MysqlTask {
    pub fn new(id: u64, title: String, description: String, completed: i8) -> Self {
        MysqlTask {
            id,
            title,
            description,
            completed,
        }
    }
}

// タスク読み出しのデータを受ける構造体
#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    // mysqlのidはu64
    pub id: u64,
    pub title: String,
    pub description: String,
    // mysqlのboolはi8
    pub completed: bool,
}

// 以下，Fromトレイトは実装しているが，IntoではなくTryIntoトレイトを実装している
// これは，変換元のi8型が0か1でない場合にエラーを返すためである
impl From<Task> for MysqlTask {
    fn from(value: Task) -> Self {
        let mysql_bool = if value.completed { 1 } else { 0 };

        MysqlTask {
            id: value.id,
            title: value.title,
            description: value.description,
            completed: mysql_bool,
        }
    }
}

impl TryInto<Task> for MysqlTask {
    type Error = &'static str;
    fn try_into(self) -> Result<Task, Self::Error> {
        let std_bool = match self.completed {
            1 => true,
            0 => false,
            _ => return Err("field [completed] is can not convert boolean"),
        };

        Ok(Task {
            id: self.id,
            title: self.title,
            description: self.description,
            completed: std_bool,
        })
    }
}

// CRUDを操作のメソッドたち
impl Task {
    // mysqlとのコネクションを引数にして，DBからすべてのタスクを取得する関数
    #[allow(dead_code)]
    pub async fn get_all(connection: &MySqlPool) -> TaskResult<Vec<Task>> {
        let mysql_tasks = sqlx::query_as!(MysqlTask, r#"SELECT * FROM todo;"#)
            .fetch_all(connection)
            .await?;

        let mut tasks = Vec::new();

        for i in mysql_tasks.into_iter() {
            let j = match i.try_into() {
                Ok(v) => v,
                Err(e) => return Err(anyhow!("{}", e)),
            };
            tasks.push(j);
        }

        Ok(tasks)
    }

    // データ挿入
    #[allow(dead_code)]
    pub async fn insert(connection: &MySqlPool, todo: NewTask) -> TaskResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO todo (title, description)
            VALUE(?,?);
            "#,
            todo.title,
            todo.description
        )
        .execute(connection)
        .await?;

        Ok(())
    }

    // データ更新
    #[allow(dead_code)]
    pub async fn toggle_with_id(connection: &MySqlPool, id: u64) -> TaskResult<()> {
        sqlx::query!(
            r#"
            UPDATE todo
            SET completed = NOT completed
            WHERE id = ?;
            "#,
            id
        )
        .execute(connection)
        .await?;

        Ok(())
    }

    // データ削除
    #[allow(dead_code)]
    pub async fn delete_with_id(connection: &MySqlPool, id: u64) -> TaskResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM todo
            WHERE id = ?;
            "#,
            id
        )
        .execute(connection)
        .await?;

        Ok(())
    }
}
