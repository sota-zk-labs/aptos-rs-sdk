use aptos_rs_sdk::aptos_client::AptosClient;
use aptos_rs_sdk::aptos_client_config::AptosClientConfig;
use aptos_rs_sdk::pagination_filter::PaginationFilter;
use aptos_rs_sdk::queries::get_events::get_events::EventsOrderBy;
use aptos_rs_sdk::queries::get_events::get_events::OrderBy::{Asc, Desc};
use aptos_rs_sdk::r#const::TESTNET;

#[tokio::main]
async fn main() {
    let aptos_client_config = AptosClientConfig {
        graphql_url: TESTNET.to_string(),
    };

    let aptos_client = AptosClient::new(aptos_client_config);

    let events = aptos_client.get_account_events_by_event_type(
        "0x3f63f9a175fe45adcb3bbfae689f4e814e4fd955c3db7bea89735b1b5d904a5b".to_string(),
        "0xd86ef4ed97b15752707c861650d084d2ad181357b7f968a2e08a931c63187c7::fri_statement::FriCtx".to_string(),
        Some(PaginationFilter {
            offset: None,
            limit: Some(1),
            order_by: Some(
                vec![EventsOrderBy {
                    creation_number: Some(Desc),
                    account_address: None,
                    data: None,
                    event_index: None,
                    indexed_type: None,
                    sequence_number: None,
                    transaction_block_height: None,
                    transaction_version: None,
                    type_: None,
                }]),
        }),
    ).await.unwrap();

    eprintln!("events = {:#?}", events);
}
