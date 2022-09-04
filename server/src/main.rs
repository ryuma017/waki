use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result};
use async_graphql::{http::{playground_source, GraphQLPlaygroundConfig}, EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

type WikiSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[derive(async_graphql::SimpleObject)]
struct QueryRoot {
    msg: String,
}

async fn index(schema: web::Data<WikiSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            playground_source(GraphQLPlaygroundConfig::new("/graphql")),
        ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let query_root = QueryRoot { msg: "Hello, GraphQL!".into() };
    let schema = Schema::build(query_root, EmptyMutation, EmptySubscription)
        .finish();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/graphql").guard(guard::Post()).to(index))
            .service(web::resource("/graphql").guard(guard::Get()).to(index_graphiql))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
