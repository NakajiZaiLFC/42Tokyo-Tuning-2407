use sqlx::MySqlPool;
use crate::{
    domains::map_service::MapRepository,
    models::graph::{Edge, Node},
};

#[derive(Debug)]
pub struct MapRepositoryImpl {
    pool: MySqlPool,
}

impl MapRepositoryImpl {
    pub fn new(pool: MySqlPool) -> Self {
        MapRepositoryImpl { pool }
    }

    async fn fetch_nodes(&self, sql: &str, area_id: Option<i32>) -> Result<Vec<Node>, sqlx::Error> {
        match area_id {
            Some(area_id) => {
                sqlx::query_as::<, Node>(sql)
                    .bind(areaid)
                    .fetch_all(&self.pool)
                    .await
            }
            None => {
                sqlx::query_as::<, Node>(sql)
                    .fetchall(&self.pool)
                    .await
            }
        }
    }

    async fn fetch_edges(&self, sql: &str, area_id: Option<i32>) -> Result<Vec<Edge>, sqlx::Error> {
        match area_id {
            Some(area_id) => {
                sqlx::query_as::<, Edge>(sql)
                    .bind(areaid)
                    .fetch_all(&self.pool)
                    .await
            }
            None => {
                sqlx::query_as::<, Edge>(sql)
                    .fetch_all(&self.pool)
                    .await
            }
        }
    }
}

impl MapRepository for MapRepositoryImpl {
    async fn get_all_nodes(&self, area_id: Option<i32>) -> Result<Vec<Node>, sqlx::Error> {
        let where_clause = if area_id.is_some() {
            "WHERE area_id = ?"
        } else {
            ""
        };

		let sql = format!(
            "SELECT * FROM nodes {} ORDER BY id",
            where_clause
        );

        self.fetch_nodes(&sql, area_id).await
    }

    async fn get_all_edges(&self, area_id: Option<i32>) -> Result<Vec<Edge>, sqlx::Error> {
        let where_clause = if area_id.is_some() {
            "JOIN nodes n ON e.node_a_id = n.id WHERE n.area_id = ?"
        } else {
            ""
        };

        let sql = format!(
            "SELECT e.node_a_id, e.node_b_id, e.weight FROM edges e {}",
            where_clause
        );

        self.fetch_edges(&sql, area_id).await
    }

    async fn get_area_id_by_node_id(&self, node_id: i32) -> Result<i32, sqlx::Error> {
        let area_id = sqlx::query_scalar("SELECT area_id FROM nodes WHERE id = ?")
            .bind(node_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(area_id)
    }

    async fn update_edge(
        &self,
        node_a_id: i32,
        node_b_id: i32,
        weight: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE edges SET weight = ? WHERE (node_a_id = ? AND node_b_id = ?) OR (node_a_id = ? AND node_b_id = ?)")
            .bind(weight)
            .bind(node_a_id)
            .bind(node_b_id)
            .bind(node_b_id)
            .bind(node_a_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}