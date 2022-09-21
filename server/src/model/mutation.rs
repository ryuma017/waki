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
        let page_record = sqlx::query_as!(
            PageRecord,
            r#"
        insert into pages (
            title, source, create_time, update_time
        ) values (
            $1, $2, current_timestamp, current_timestamp
        ) returning
            id, title, source, create_time, update_time
        ;
            "#,
            input.title,
            input.source,
        )
        .fetch_one(&mut tx)
        .await?;

        let gql_page = Page {
            id: page_record.id,
            title: page_record.title,
            body_html: page_record.source,
        };

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

        Ok(gql_page)
    }
}
