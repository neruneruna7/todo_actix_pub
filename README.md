# Todo_actix

actix-web と sqlx を使った TODO アプリケーションです．  
命名はフォルダ構造など，actixのexampleをおおいに参考にしています  
公開するには.env に問題を抱えていたので，公開用のリポジトリを作り直しました．

<br>

# 前提
Rustの環境が整っていること．

<br>

## 使い方
### Mysqlの準備
まずMysqlをインストール，DBサーバーを起動してください．

次に，Mysqlのユーザを作成，権限設定し，.envファイルの内容を書き換えてください．

.envファイル
```
DATABASE_URL=mysql://{ユーザ名}:{パスワード}@localhost:3306/todo_actix
```

<br>

### sqlx-cliのインストール
sqlx-cliをインストールしてください．
```
cargo install sqlx-cli
```
インストールの際にオプション指定ができたりもしますが，それは公式のドキュメントを参照してください．  
[sqlx-cli のcrate-io](https://crates.io/crates/sqlx-cli)  

<br>

### マイグレーション
既に`migration`フォルダにマイグレーションファイルがあるので，それを実行することでマイグレーションを行います．
```bash
sqlx migrate run
```

これで，DBの準備は完了です．
コードをコンパイル可能になっているでしょう.

<br>

## デバッグビルド実行
```bash
cargo run
```

<br>

## 操作
本Todoアプリにはクライアントは存在しません．
よって，何かしらのHTTPクライアントを用いて操作してください．

このリポジトリには，test.http というファイルがあります．
これは，VsCode拡張機能の REST Client で使用することができるファイルです．
その拡張機能を導入済みであれば，このファイルを開いて，`Send Request`を押すことで，リクエストを送信することができます．

<br>

## 操作方法
### Task読み取り
GET http://localhost:8080/

タスクの一覧を取得します．  

<br>

### Task作成
POST  http://localhost:8080/todo/insert/ HTTP/1.1
Content-Type: application/json

{
    "title": "test",
    "description": "test"
}

タスクを作成します.
json形式で，`title`と`description`を指定します．  

<br>

### Taskの状態更新
GET  http://localhost:8080/todo/toggle/{id}

完了したタスクであるか否かを示す`completed`を切り替えます．
指定したidのみを更新します．

<br>

### Task削除
GET  http://localhost:8080/todo/delete/{id}

指定したidのタスクを削除します．



