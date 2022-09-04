use std::time::Duration;

use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result};
use agql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Object, Schema, SimpleObject, InputObject,
};
use async_graphql as agql;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use chrono::NaiveDateTime;
use sqlx::{postgres::PgPoolOptions, PgPool};

type WikiSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

struct QueryRoot;

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

#[derive(SimpleObject)]
struct Page {
    id: i32,
    title: String,
    body_html: String,
}

#[derive(InputObject)]
struct CreatePageInput {
    title: String,
    source: String,
}

#[allow(dead_code)]
#[derive(Debug)]
struct PageRecord {
    id: i32,
    title: String,
    source: String,
    create_time: NaiveDateTime,
    update_time: NaiveDateTime,
}

struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_page(&self, ctx: &agql::Context<'_>, input: CreatePageInput) -> Result<Page, agql::Error> {
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
        tx.commit().await?;
        let gql_page = Page {
            id: page_record.id,
            title: page_record.title,
            body_html: page_record.source,
        };
        Ok(gql_page)
    }
}

async fn index(schema: web::Data<WikiSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/graphql"))))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let pg_pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(5))
        .connect("postgres://postgres:password@localhost:5432/wiki")
        .await
        .unwrap();
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pg_pool)
        .finish();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/graphql").guard(guard::Post()).to(index))
            .service(
                web::resource("/graphql")
                    .guard(guard::Get())
                    .to(index_graphiql),
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
