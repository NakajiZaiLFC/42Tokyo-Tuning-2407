-- このファイルに記述されたSQLコマンドが、マイグレーション時に実行されます。
-- ファイル名: 20240728_add_indexes_to_auth_tables.sql
-- 説明: auth関連のテーブルにインデックスを追加します。
-- このファイルに記述されたSQLコマンドが、マイグレーション時に実行されます。

-- users テーブルのインデックス
CREATE INDEX idx_users_id ON users (id);
CREATE UNIQUE INDEX idx_users_username ON users (username);

-- sessions テーブルのインデックス
CREATE UNIQUE INDEX idx_sessions_session_token ON sessions (session_token);
CREATE INDEX idx_sessions_user_id ON sessions (user_id);

-- dispatchers テーブルのインデックス
CREATE INDEX idx_dispatchers_id ON dispatchers (id);
CREATE UNIQUE INDEX idx_dispatchers_user_id ON dispatchers (user_id);
CREATE INDEX idx_dispatchers_area_id ON dispatchers (area_id);

-- 注意: 既存のインデックスがある場合は、以下のようにIF NOT EXISTSを使用することができます
-- CREATE INDEX IF NOT EXISTS idx_users_id ON users (id);