use agql::Object;
use async_graphql as agql;
use sqlx::PgPool;

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
}
