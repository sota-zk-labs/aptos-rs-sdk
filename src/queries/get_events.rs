use graphql_client::GraphQLQuery;
use super::typ::Jsonb;
use super::typ::Bigint;
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/get_events.graphql",
    response_derives = "Debug",
    normalization = "rust",
    skip_serializing_none
)]
pub struct GetEvents;

