use crate::errors::AppError;
use crate::models::user::{Dispatcher, User};
use crate::{domains::auth_service::AuthRepository, models::user::Session};
use sqlx::mysql::MySqlPool;

#[derive(Debug)]
pub struct AuthRepositoryImpl {
    pool: MySqlPool,
}

impl AuthRepositoryImpl {
    pub fn new(pool: MySqlPool) -> Self {
        AuthRepositoryImpl { pool }
    }
}

impl AuthRepository for AuthRepositoryImpl {
    // async fn find_user_by_id(&self, id: i32) -> Result<Option<User>, AppError> {
    //     let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
    //         .bind(id)
    //         .fetch_optional(&self.pool)
    //         .await?;

    //     Ok(user)
    // }

    async fn find_user_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
            .bind(username)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

	async fn find_user_and_profile_image_by_id(&self, user_id: i32) -> Result<(Option<User>, Option<String>), AppError> {
		let result = sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", user_id)
			.fetch_optional(&self.pool)
			.await?;
	
		let profile_image = result.as_ref().map(|user| user.profile_image.clone());
		Ok((result, profile_image))
	} 

    // async fn find_profile_image_name_by_user_id(
    //     &self,
    //     user_id: i32,
    // ) -> Result<Option<String>, AppError> {
    //     let profile_image_name = sqlx::query_scalar("SELECT profile_image FROM users WHERE id = ?")
    //         .bind(user_id)
    //         .fetch_optional(&self.pool)
    //         .await?;

    //     Ok(profile_image_name)
    // }

    async fn authenticate_user(&self, username: &str, password: &str) -> Result<User, AppError> {
        let user =
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ? AND password = ?")
                .bind(username)
                .bind(password)
                .fetch_one(&self.pool)
                .await?;

        Ok(user)
    }

    async fn create_user(
        &self,
        username: &str,
        password: &str,
        role: &str,
    ) -> Result<(), AppError> {
        sqlx::query("INSERT INTO users (username, password, role) VALUES (?, ?, ?)")
            .bind(username)
            .bind(password)
            .bind(role)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn create_session(&self, user_id: i32, session_token: &str) -> Result<(), AppError> {
        sqlx::query("INSERT INTO sessions (user_id, session_token) VALUES (?, ?)")
            .bind(user_id)
            .bind(session_token)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn delete_session(&self, session_token: &str) -> Result<(), AppError> {
        sqlx::query("DELETE FROM sessions WHERE session_token = ?")
            .bind(session_token)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn find_session_by_session_token(
        &self,
        session_token: &str,
    ) -> Result<Session, AppError> {
        let session =
            sqlx::query_as::<_, Session>("SELECT * FROM sessions WHERE session_token = ?")
                .bind(session_token)
                .fetch_one(&self.pool)
                .await?;

        Ok(session)
    }

    // async fn find_dispatcher_by_id(&self, id: i32) -> Result<Option<Dispatcher>, AppError> {
    //     let dispatcher = sqlx::query_as::<_, Dispatcher>("SELECT * FROM dispatchers WHERE id = ?")
    //         .bind(id)
    //         .fetch_optional(&self.pool)
    //         .await?;

    //     Ok(dispatcher)
    // }

    // async fn find_dispatcher_by_user_id(
    //     &self,
    //     user_id: i32,
    // ) -> Result<Option<Dispatcher>, AppError> {
    //     let dispatcher =
    //         sqlx::query_as::<_, Dispatcher>("SELECT * FROM dispatchers WHERE user_id = ?")
    //             .bind(user_id)
    //             .fetch_optional(&self.pool)
    //             .await?;

    //     Ok(dispatcher)
    // }

	async fn finddispatcher(&self, id: Option<i32>, user_id: Option<i32>) -> Result<Option<Dispatcher>, AppError> {
		let mut query = "SELECT * FROM dispatchers".to_string();
		if let Some(id) = id {
			query.push_str(" WHERE id = ?");
			let dispatcher = sqlx::query_as::<, Dispatcher>(&query)
				.bind(id)
				.fetchoptional(&self.pool)
				.await?;
			return Ok(dispatcher);
		}
		if let Some(user_id) = user_id {
			query.push_str(" WHERE user_id = ?");
			let dispatcher = sqlx::query_as::<, Dispatcher>(&query)
				.bind(user_id)
				.fetch_optional(&self.pool)
				.await?;
			return Ok(dispatcher);
		}
		Ok(None)
	}

    async fn create_dispatcher(&self, user_id: i32, area_id: i32) -> Result<(), AppError> {
        sqlx::query("INSERT INTO dispatchers (user_id, area_id) VALUES (?, ?)")
            .bind(user_id)
            .bind(area_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
