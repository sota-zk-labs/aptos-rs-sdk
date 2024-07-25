use reqwest::{Client};
use crate::aptos_client_config::AptosClientConfig;
use crate::queries::get_events::get_events::{EventsBoolExp, EventsOrderBy, GetEventsEvents, ResponseData, StringComparisonExp, Variables};
use crate::queries::get_events::GetEvents;
use graphql_client::{GraphQLQuery, Response};
use crate::errors::AptosClientError;
use crate::pagination_filter::{extract_pagination, PaginationFilter};

#[derive(Debug)]
pub struct AptosClient {
    pub aptos_client_config: AptosClientConfig,
    client: Client,
}

impl AptosClient {
    pub fn new(aptos_client_config: AptosClientConfig) -> Self {
        let client = Client::new();
        Self::new_with_client(aptos_client_config, client)
    }

    pub fn new_with_client(aptos_client_config: AptosClientConfig, client: Client) -> Self {
        Self {
            aptos_client_config,
            client,
        }
    }
}

impl AptosClient {
    pub async fn get_account_events_by_event_type(&self, account_address: String, indexed_type: String, option: Option<PaginationFilter<Vec<EventsOrderBy>>>) -> Result<Vec<GetEventsEvents>, AptosClientError> {
        let (offset, limit, order_by) = extract_pagination(option);
        let variables = Variables {
            where_condition: Some(EventsBoolExp {
                and: Box::new(None),
                not: Box::new(None),
                or: Box::new(None),
                account_address: Self::compare_equal_string(account_address),
                creation_number: None,
                data: None,
                event_index: None,
                indexed_type: Self::compare_equal_string(indexed_type),
                sequence_number: None,
                transaction_block_height: None,
                transaction_version: None,
                type_: None,
            }),
            offset,
            limit,
            order_by,
        };
        let request_body = GetEvents::build_query(variables);

        let res = self.client.post(&self.aptos_client_config.graphql_url).json(&request_body).send().await?;
        res.json::<Response<ResponseData>>().await?.data.map(|d| { d.events }).ok_or(AptosClientError::NotFound)
    }

    fn compare_equal_string(input: String) -> Option<StringComparisonExp> {
        Some(StringComparisonExp {
            eq: Some(input),
            gt: None,
            gte: None,
            ilike: None,
            in_: None,
            iregex: None,
            is_null: None,
            like: None,
            lt: None,
            lte: None,
            neq: None,
            nilike: None,
            nin: None,
            niregex: None,
            nlike: None,
            nregex: None,
            nsimilar: None,
            regex: None,
            similar: None,
        })
    }
}