use crate::types::*;
use sqlx::{Pool, Postgres, query, query_as, query_scalar};

#[derive(Debug)]
pub struct DB {
    pub pool: Pool<Postgres>,
}

impl DB {
    pub async fn read_theme(&self, ulid: &Ulid) -> Result<Option<ReadData>, SqlxError> {
        query_as("SELECT name, schemes, owner, description FROM themes WHERE ulid = $1")
            .bind(ulid)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn read_theme_owner(&self, theme_ulid: &Ulid) -> Result<Option<Ulid>, SqlxError> {
        query_scalar("SELECT owner FROM themes WHERE ulid = $1")
            .bind(theme_ulid)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn create_theme(
        &self,
        create_data: &CreateData,
        owner: &Ulid,
    ) -> Result<Ulid, SqlxError> {
        query_scalar("INSERT INTO themes (ulid, name, schemes, owner, description) VALUES ($1, $2, $3, $4, $5) RETURNING ulid")
            .bind(Ulid(PrimitiveUlid::new()))
            .bind(&create_data.name)
            .bind(SqlxJson(&create_data.schemes))
            .bind(owner)
            .bind(match &create_data.description {
                // if description is empty, make it None
                Some(d) => if d.len() == 0 {None} else {Some(d)},
                None=>None
            })
            .fetch_one(&self.pool)
            .await
    }

    pub async fn list_themes(&self, page: i64, per_page: i64) -> Result<Vec<ListData>, SqlxError> {
        query_as(
            "SELECT ulid, name, schemes, owner, description FROM themes ORDER BY ulid LIMIT $1 OFFSET $2",
        )
        .bind(per_page)
        .bind((page - 1) * per_page)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn update_theme(
        &self,
        ulid: &Ulid,
        update_data: &UpdateData,
    ) -> Result<(), SqlxError> {
        query("UPDATE themes SET name = $1, schemes = $2, description = $3 WHERE ulid = $4")
            .bind(&update_data.name)
            .bind(SqlxJson(&update_data.schemes))
            .bind(&update_data.description)
            .bind(ulid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn delete_theme(&self, ulid: &Ulid) -> Result<(), SqlxError> {
        let _ = query("DELETE FROM themes WHERE ulid = $1")
            .bind(ulid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn create_user(&self, email: &str) -> Result<Ulid, SqlxError> {
        query_scalar("INSERT INTO users (ulid, email, username) VALUES ($1, $2, $3) RETURNING ulid")
            .bind(Ulid(PrimitiveUlid::new()))
            .bind(email)
            .bind(PrimitiveUlid::new().to_string())
            .fetch_one(&self.pool)
            .await
    }

    pub async fn read_username(&self, user_ulid: &Ulid) -> Result<Option<String>, SqlxError> {
        query_scalar("SELECT username FROM users WHERE ulid = $1")
            .bind(user_ulid)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn update_username(
        &self,
        user_ulid: &Ulid,
        new_username: &str,
    ) -> Result<(), SqlxError> {
        query("UPDATE users SET username = $2 WHERE ulid = $1")
            .bind(user_ulid)
            .bind(new_username)
            .execute(&self.pool)
            .await
            .map(|_| ())
    }

    pub async fn read_user(&self, email: &str) -> Result<Option<Ulid>, SqlxError> {
        query_scalar("SELECT ulid FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn check_user_exists(&self, user_ulid: &Ulid) -> Result<bool, SqlxError> {
        query("SELECT NULL FROM users WHERE ulid = $1")
            .bind(user_ulid)
            .fetch_optional(&self.pool)
            .await
            .map(|row| row.is_some())
    }

    pub async fn read_theme_count(&self) -> Result<i64, SqlxError> {
        query_scalar("SELECT theme_count FROM theme_count")
            .fetch_one(&self.pool)
            .await
    }
}
