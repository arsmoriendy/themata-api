use crate::types::*;
use sqlx::{Pool, Postgres, QueryBuilder, query, query_as, query_scalar};

#[derive(Debug)]
pub struct DB {
    pub pool: Pool<Postgres>,
}

#[derive(Debug)]
pub enum ListFilter<'a> {
    Search(&'a str),
    Owner(Ulid),
    LikedBy(Ulid),
}

#[derive(Debug)]
pub enum SortList {
    Views,
    Likes,
    Created,
}

impl Default for SortList {
    fn default() -> Self {
        Self::Created
    }
}

#[derive(Debug)]
pub enum SortOrder {
    Ascending,
    Descending,
}

impl Default for SortOrder {
    fn default() -> Self {
        Self::Descending
    }
}

impl DB {
    pub async fn read_theme(&self, ulid: &Ulid) -> Result<Option<ReadData>, SqlxError> {
        query_as("SELECT name, schemes, owner, description, count(likes) AS like_count, views, created_at FROM themes LEFT JOIN likes ON themes.ulid = likes.theme_ulid GROUP BY themes.ulid HAVING ulid = $1")
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

    pub async fn list_themes<'a>(
        &self,
        page: i64,
        per_page: i64,
        filters: &[ListFilter<'a>],
        sort_by: SortList,
        sort_order: SortOrder,
    ) -> Result<Vec<ListData>, SqlxError> {
        let mut q = QueryBuilder::<Postgres>::new(
            "SELECT ulid, name, schemes, owner, description, count(likes) AS like_count, views, created_at FROM themes LEFT JOIN likes ON themes.ulid = likes.theme_ulid",
        );

        if !filters.is_empty() {
            q.push(" WHERE ");
            for (i, f) in filters.iter().enumerate() {
                match f {
                    ListFilter::Search(s) => q
                        .push("LOWER(name) LIKE '%' || LOWER(")
                        .push_bind(s)
                        .push(") || '%'"),
                    ListFilter::Owner(o) => q.push("owner = ").push_bind(o),
                    ListFilter::LikedBy(u) => q.push("likes.user_ulid = ").push_bind(u),
                };
                if i != filters.len() - 1 {
                    q.push(" AND ");
                };
            }
        }

        q.push(" GROUP BY themes.ulid ORDER BY ")
            .push(match sort_by {
                SortList::Views => "views",
                SortList::Created => "created_at",
                SortList::Likes => "like_count",
            })
            .push(" ")
            .push(match sort_order {
                SortOrder::Ascending => "ASC",
                SortOrder::Descending => "DESC",
            });

        q.push(" LIMIT ")
            .push_bind(per_page)
            .push(" OFFSET ")
            .push_bind((page - 1) * per_page);
        q.build_query_as().fetch_all(&self.pool).await
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
        query("DELETE FROM themes WHERE ulid = $1")
            .bind(ulid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn create_user(&self, email: &str) -> Result<Ulid, SqlxError> {
        query_scalar("INSERT INTO users (ulid, email) VALUES ($1, $2) RETURNING ulid")
            .bind(Ulid(PrimitiveUlid::new()))
            .bind(email)
            .fetch_one(&self.pool)
            .await
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

    pub async fn like(&self, theme: &Ulid, user: &Ulid) -> Result<(), SqlxError> {
        query("INSERT INTO likes (user_ulid, theme_ulid) VALUES ($1, $2)")
            .bind(user)
            .bind(theme)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn unlike(&self, theme: &Ulid, user: &Ulid) -> Result<(), SqlxError> {
        query("DELETE FROM likes WHERE user_ulid = $1 AND theme_ulid = $2")
            .bind(user)
            .bind(theme)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn liked(&self, theme: &Ulid, user: &Ulid) -> Result<bool, SqlxError> {
        let liked: bool =
            query_scalar("SELECT true FROM likes WHERE user_ulid = $1 AND theme_ulid = $2")
                .bind(user)
                .bind(theme)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| {
                    tracing::error!("{e}");
                    e
                })?
                .unwrap_or(false);
        Ok(liked)
    }

    pub async fn increment_views_by(&self, theme: &Ulid, inc: i64) -> Result<(), SqlxError> {
        query("UPDATE themes SET views = views + $2 WHERE ulid = $1")
            .bind(theme)
            .bind(inc)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
