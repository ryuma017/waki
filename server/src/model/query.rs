use agql::Object;
use async_graphql as agql;
use sqlx::PgPool;

use crate::db::PageRecord;

use super::Page;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn msg(&self, ctx: &agql::Context<'_>) -> Result<String, agql::Error> {
        let pool = ctx.data::<PgPool>()?;

        let msg: String = sqlx::query_scalar_unchecked!(r#"select 'Hello, GraphQL!';"#)
            .fetch_one(pool)
            .await?;

        Ok(msg)
    }

    async fn page(&self, ctx: &agql::Context<'_>, id: i32) -> Result<Option<Page>, agql::Error> {
        let pool = ctx.data::<PgPool>()?;

        let page_record = sqlx::query_as!(
            PageRecord,
            r#"
        select
            id, title, source, create_time, update_time
        from
            pages
        where
            id = $1
        ;
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        let page = page_record.map(Into::into);

        Ok(page)
    }

    async fn page_by_title(
        &self,
        ctx: &agql::Context<'_>,
        title: String,
    ) -> Result<Option<Page>, agql::Error> {
        let pool = ctx.data::<PgPool>()?;

        let page_record = sqlx::query_as!(
            PageRecord,
            r#"
        select
            id, title, source, create_time, update_time
        from
            pages
        where
            title = $1
        ;
            "#,
            title
        )
        .fetch_optional(pool)
        .await?;

        let page = page_record.map(Into::into);

        Ok(page)
    }
}
