-- Add migration script here

CREATE TABLE
    IF NOT EXISTS todo (
        id SERIAL PRIMARY KEY,
        title VARCHAR(255) NOT NULL,
        description VARCHAR(255) NOT NULL,
        completed BOOLEAN NOT NULL DEFAULT FALSE
    ) ENGINE = InnoDB;

-- q:migration 20230421010827 is partially applied; fix and remove row from `_sqlx_migrations` table

-- これはどういう意味ですか？

-- a: このエラーは、migrations テーブルに記録されているマイグレーションの状態と、実際の DB の状態が一致しないことを示しています。

--    この場合、migrations テーブルには 20230421010827 というマイグレーションが適用されていると記録されていますが、実際の DB には適用されていません。

--    このため、このマイグレーションを再度適用する必要があります。

--    このマイグレーションを再度適用するには、migrations テーブルから該当のマイグレーションを削除する必要があります。