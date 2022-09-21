pub mod mutation;
pub mod query;

use async_graphql::{EmptySubscription, Schema, SimpleObject};

use crate::model::{mutation::MutationRoot, query::QueryRoot};

pub type WikiSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(SimpleObject)]
pub struct Page {
    id: i32,
    title: String,
    body_html: String,
}
