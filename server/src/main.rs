use std::time::Duration;

use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result};
use agql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql as agql;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use sqlx::postgres::PgPoolOptions;

use wiki_api::model::{mutation::MutationRoot, query::QueryRoot, WikiSchema};

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
