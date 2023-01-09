use std::{collections::HashSet, future::Future, time::Duration};
use serde::de::DeserializeOwned;
use crate::cardano::{api::CardanoApi, model::Asset, tango::model::Address};
use super::model::ApiListRes;

// feel free to add/remove properties as needed.
#[derive(Clone)]
pub struct TangoClient {
    base_url: String,
    app_id: String,
    api_key: String,
    client: reqwest::Client,
}

impl TangoClient {
    pub fn new(base_url: String, app_id: String, api_key: String) -> anyhow::Result<Self> {
        Ok(TangoClient {
            base_url,
            app_id,
            api_key,
            client: reqwest::Client::new(),
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
        // rate limit work arround
        tokio::time::sleep(Duration::from_millis(100)).await;
        let res = f(cursor.clone()).await?;

        data.append(&mut res.data.clone());
        cursor = res.cursor;

        match cursor {
            None => break,
            _ => {}
        }

        match cursor {
            None => break,
            _ => {}
        }
    }

    Ok(data)
}


#[async_trait::async_trait]
impl CardanoApi for TangoClient {
    // recommended api:
    // https://www.tangocrypto.com/api-reference/#/operations/list-stake_address-addresses
    //   https://cardano-mainnet.tangocrypto.com/f95871car1b0412bbe3750df46f9540e/v1/wallets/stake_address/addresses \
    async fn get_all_addresses(&self, stake_address: &str) -> anyhow::Result<Vec<String>> {

        let response = get_all(|cursor| async move {
            let query = if let Some(cur) = cursor {
                ("cursor", cur)
            } 
            else {
                ("cursor","".to_string())
            };

            let req = self.client
            .request(reqwest::Method::GET, format!("{}/{}/v1/wallets/{}/addresses", self.base_url.clone(),self.app_id.clone(), stake_address.to_string()))
            .header("x-api-key", self.api_key.clone())
            .query(&[query])
            .build()?;
            let api_response = self.client.execute(req).await;

            let addresses = match api_response {
                Ok(res) => {
                    let value = res.json::<ApiListRes<Address>>().await?;
                    value
                },
                Err(err) => {
                    anyhow::bail!(err)
                }
            };

            Ok(addresses)
        }).await?;

        let addrs: Vec<String> = response.into_iter().map(|addr| addr.address.clone()).collect();
        Ok(addrs)
    }

    // recommended api:
    // https://www.tangocrypto.com/api-reference/#/operations/list-address-assets
    async fn get_address_assets(&self, address: &str) -> anyhow::Result<Vec<Asset>> {
        let response = get_all(|cursor| async move {
            // rate limit work arround
            tokio::time::sleep(Duration::from_millis(50)).await;
            let cursor_query = if let Some(cur) = cursor {
                ("cursor", cur)
            } 
            else {
                ("cursor","".to_string())
            };

            let req = self.client
            .request(reqwest::Method::GET, format!("{}/{}/v1/addresses/{}/assets", self.base_url.clone(),self.app_id.clone(), address.to_string()))
            .header("x-api-key", self.api_key.clone())
            .query(&[cursor_query])
            .build()?;
            //let api_text = self.client.execute(req.try_clone().unwrap()).await?.text().await?;
            let api_response = self.client.execute(req).await;
            let assets = match api_response {
                Ok(res) => {
                    let value = res.json::<ApiListRes<Asset>>().await?;
                    value
                },
                Err(err) => {
                    anyhow::bail!(err)
                }
            };
            Ok(assets)

        }).await;

        response
    }

    // recommended api:
    // https://www.tangocrypto.com/api-reference/#/operations/list-asset-addresses
    async fn get_asset_addresses(&self, asset_id: &str) -> anyhow::Result<HashSet<String>> {

        let response = get_all(|cursor| async move {
            // rate limit work arround
            tokio::time::sleep(Duration::from_millis(50)).await;
            let query = if let Some(cur) = cursor {
                ("cursor", cur)
            } 
            else {
                ("cursor","".to_string())
            };

            let req = self.client
            .request(reqwest::Method::GET, format!("{}/{}/v1/assets/{}/addresses", self.base_url.clone(),self.app_id.clone(), asset_id.to_string()))
            .header("x-api-key", self.api_key.clone())
            .query(&[query])
            .build()?;
            let api_response = self.client.execute(req).await?.json::<ApiListRes<serde_json::Value>>().await?;
            Ok(api_response)
        }).await?;

        let mut addrs: HashSet<String> = HashSet::new();

        let filtered = response
            .into_iter()
            .filter(|addr| addr.is_object()).collect::<Vec<serde_json::Value>>();

        for val in filtered.into_iter() {
            addrs.insert(val.get("address").unwrap().as_str().unwrap().to_string());
        }
        
        Ok(addrs)
    }
}
