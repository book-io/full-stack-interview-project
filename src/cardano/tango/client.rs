use std::{collections::HashSet, future::Future};

use serde::de::DeserializeOwned;

use crate::cardano::tango::endpoints::{add_cursor_to_url, generate_get_all_addresses_endpoint};
use crate::cardano::tango::lib::TANGO_API_KEY_HEADER;
use crate::cardano::tango::model::Address;
use crate::cardano::{api::CardanoApi, model::Asset};

use super::model::ApiListRes;

// feel free to add/remove properties as needed.
pub struct TangoClient {
    base_url: String,
    app_id: String,
    api_key: String,
}

impl TangoClient {
    pub fn new(base_url: String, app_id: String, api_key: String) -> anyhow::Result<Self> {
        Ok(TangoClient {
            base_url,
            app_id,
            api_key,
        })
    }
}

/**
 * A helper method that abstracts iterating over an API response
 * that returns a cursor when there are more results available.
 *
 * Example:
 *
 * async fn get_the_things(id: &str, cursor: Option<String>) -> anyhow::Result<ApiListRes<TheThing>>;
 *
 * let id = "something";
 * let res = get_all(|cursor| get_the_things(&id, cursor)).await?;
 */
pub async fn get_all<F, Fut, T>(f: F) -> anyhow::Result<Vec<T>>
where
    F: Fn(Option<String>) -> Fut,
    Fut: Future<Output = anyhow::Result<ApiListRes<T>>>,
    T: DeserializeOwned,
    T: Clone,
{
    let mut data = Vec::new();
    let mut cursor = None;
    loop {
        let res = f(cursor.clone()).await?;

        data.append(&mut res.data.clone());
        cursor = res.cursor;

        match cursor {
            None => break,
            _ => {}
        }
    }

    Ok(data)
}

/// Helper method that abstracts calls to Tango Crypto for an arbitrary url,
/// which can optionally have a cursor appended
async fn get_collection_from_tango<T>(
    url: &str,
    api_key: &str,
    cursor: Option<String>,
) -> anyhow::Result<ApiListRes<T>>
where
    T: DeserializeOwned,
{
    let full_url = if cursor.is_some() {
        add_cursor_to_url(url, &cursor.unwrap())
    } else {
        url.to_string()
    };

    let res = reqwest::Client::new()
        .get(full_url)
        .header(TANGO_API_KEY_HEADER, api_key)
        .send()
        .await?
        .json()
        .await?;

    Ok(res)
}

#[async_trait::async_trait]
impl CardanoApi for TangoClient {
    // recommended api:
    // https://www.tangocrypto.com/api-reference/#/operations/list-stake_address-addresses
    async fn get_all_addresses(&self, stake_address: &str) -> anyhow::Result<Vec<String>> {
        let url = generate_get_all_addresses_endpoint(&self.base_url, &self.app_id, stake_address);
        let api_key = &self.api_key;

        let response: Vec<Address> =
            get_all(|cursor| get_collection_from_tango(&url, api_key, cursor)).await?;

        let addresses: Vec<String> = response
            .into_iter()
            .map(|address| address.address)
            .collect();

        Ok(addresses)
    }

    // recommended api:
    // https://www.tangocrypto.com/api-reference/#/operations/list-address-assets
    async fn get_address_assets(&self, address: &str) -> anyhow::Result<Vec<Asset>> {
        todo!()
    }

    // recommended api:
    // https://www.tangocrypto.com/api-reference/#/operations/list-asset-addresses
    async fn get_asset_addresses(&self, asset_id: &str) -> anyhow::Result<HashSet<String>> {
        todo!()
    }
}
