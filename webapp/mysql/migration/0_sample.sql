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

-- areas テーブルのインデックス
CREATE INDEX idx_areas_id ON areas (id);

-- orders テーブルのインデックス
CREATE INDEX idx_orders_id ON orders (id);
CREATE INDEX idx_orders_dispatcher_id ON orders (dispatcher_id);
CREATE INDEX idx_orders_area_id ON orders (area_id);
CREATE INDEX idx_orders_status ON orders (status);

-- order_details テーブルのインデックス
CREATE INDEX idx_order_details_id ON order_details (id);
CREATE INDEX idx_order_details_order_id ON order_details (order_id);

-- order_histories テーブルのインデックス
CREATE INDEX idx_order_histories_id ON order_histories (id);
CREATE INDEX idx_order_histories_order_id ON order_histories (order_id);

-- order_details_histories テーブルのインデックス
CREATE INDEX idx_order_details_histories_id ON order_details_histories (id);
CREATE INDEX idx_order_details_histories_order_detail_id ON order_details_histories (order_detail_id);


-- 以下のように、複数のカラムにインデックスを追加することもできます
-- CREATE INDEX idx_users_id_username ON users (id, username);

-- 注意: 既存のインデックスがある場合は、以下のようにIF NOT EXISTSを使用することができます
-- CREATE INDEX IF NOT EXISTS idx_users_id ON users (id);