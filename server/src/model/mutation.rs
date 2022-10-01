use agql::{InputObject, Object};
use async_graphql as agql;
use sqlx::PgPool;

use crate::db::PageRecord;
use crate::model::Page;

#[derive(InputObject)]
pub struct CreatePageInput {
    title: String,
    source: String,
}

#[derive(InputObject)]
pub struct UpdatePageInput {
    id: i32,
    title: Option<String>,
    source: Option<String>,
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_page(
        &self,
        ctx: &agql::Context<'_>,
        input: CreatePageInput,
    ) -> Result<Page, agql::Error> {
        let pool = ctx.data::<PgPool>()?;
        let mut tx = pool.begin().await?;
        let now = chrono::Utc::now().naive_utc();

        let page_record = sqlx::query_as!(
            PageRecord,
            r#"
        insert into pages (
            title, source, create_time, update_time
        ) values (
            $1, $2, $3, $4
        ) returning
            id, title, source, create_time, update_time
        ;
            "#,
            input.title,
            input.source,
            now,
            now,
        )
        .fetch_one(&mut tx)
        .await?;

        sqlx::query!(
            r#"
        insert into page_revisions (
            page_id, source, author, create_time
        ) values (
            $1, $2, $3, $4
        );
            "#,
            page_record.id,
            input.source,
            "anonymous@example.com", // FIXME: 認証作ったときに直す
            page_record.update_time,
        )
        .execute(&mut tx)
        .await?;

        tx.commit().await?;

        let gql_page = page_record.into();

        Ok(gql_page)
    }

    async fn update_page(
        &self,
        ctx: &agql::Context<'_>,
        input: UpdatePageInput,
    ) -> Result<Option<Page>, agql::Error> {
        let pool = ctx.data::<PgPool>()?;
        let mut tx = pool.begin().await?;

        let now = chrono::Utc::now().naive_utc();

        if let Some(title) = input.title {
            let id = sqlx::query_scalar!(
                r#"
            update pages
            set
                title = $1,
                update_time = $2
            where
                id = $3
            returning
                id
            ;
                "#,
                title,
                now,
                input.id,
            )
            .fetch_optional(&mut tx)
            .await?;

            if id.is_none() {
                return Ok(None);
            }
        }

        if let Some(source) = input.source {
            let id = sqlx::query_scalar!(
                r#"
            update pages
            set
                source = $1,
                update_time = $2
            where
                id = $3
            returning
                id
            ;
                "#,
                source,
                now,
                input.id,
            )
            .fetch_optional(&mut tx)
            .await?;

            if id.is_none() {
                return Ok(None);
            }

            sqlx::query!(
                r#"
            insert into page_revisions (
                page_id, source, author, create_time
            ) values (
                $1, $2, $3, $4
            );
                "#,
                input.id,
                source,
                "anonymous@exampl.com",
                now,
            )
            .execute(&mut tx)
            .await?;
        }

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
            input.id,
        )
        .fetch_optional(&mut tx)
        .await?;

        tx.commit().await?;

        let page = page_record.map(Into::into);

        Ok(page)
    }
}
