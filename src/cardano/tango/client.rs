use tokio_native_tls::TlsConnector as TlsConnector_Async;
use tokio_native_tls::native_tls::TlsConnector;
use tokio_native_tls::TlsStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::TcpStream;
use std::io::Write;
use std::io::Read;
use std::io::ErrorKind;
use std::net::*;
use std::vec::Vec;
use serde_json::Value as SerdeValue;
use anyhow::Context;

use std::{collections::HashSet, future::Future};

use serde::de::DeserializeOwned;

use crate::cardano::{api::CardanoApi, model::Asset, request_helper::*};

use super::model::ApiListRes;
use super::model::*;

// feel free to add/remove properties as needed.
pub struct TangoClient 
{
  host: String,
  base_url: String,
  app_id: String,
  api_key: String
}

impl TangoClient 
{
  pub fn new(base_url: String, app_id: String, api_key: String) -> anyhow::Result<Self> 
  {
    let host: &str = base_url.as_str();

    let host: &str = if host.starts_with("https://")
    {
      &host["https://".len()..]
    }
    else
    {
      &host
    };
    let host: &str = if host.ends_with("/")
    {
      &host[..host.len() - 1]
    }
    else
    {
      &host
    };

    let host = host.to_owned();

    Ok(TangoClient {
        host,
        base_url,
        app_id,
        api_key
    })
  }

  async fn get_addresses(&self, stake_address: &str, cursor: Option<String>) -> anyhow::Result<ApiListRes<Address>>
  {
    let mut path = format!("/{}/v1/wallets/{}/addresses", self.app_id, stake_address);
    self.make_get_request::<Address>(path, cursor).await
  }

  async fn get_address_assets(&self, address: &str, cursor: Option<String>) -> anyhow::Result<ApiListRes<AddressAsset>> 
  {
    let mut path = format!("/{}/v1/addresses/{}/assets", self.app_id, address);
    self.make_get_request::<AddressAsset>(path, cursor).await
  }

  async fn get_asset_addresses(&self, asset_id: &str, cursor: Option<String>) -> anyhow::Result<ApiListRes<AssetAddress>> 
  {
    let mut path = format!("/{}/v1/assets/{}/addresses", self.app_id, asset_id);
    self.make_get_request::<AssetAddress>(path, cursor).await
  }

  async fn make_get_request<T>(&self, mut path: String, cursor: Option<String>) -> anyhow::Result<ApiListRes<T>>
  where
    T: DeserializeOwned
  {
    if let Some(x) = cursor
    {
      path.push_str("?cursor=");
      path.push_str(x.as_str());
    }

    let req = HttpGetRequest::new(
      self.host.as_str(),
      path.as_str(),
      vec![
        ("Accept", "application/json"),
        ("x-api-key", self.api_key.as_str())
      ]
    );
  
    let mut res = http_s_get(req).await?;
  
    let statusCode = parse_httpStatusCode(&res);
  
    if statusCode == 200
    {
      let body = parse_extractJsonStringFromBody(&res)?;
      let result: ApiListRes<T> = parse_apiBody(&body)?;
      Ok(result)
    }
    else
    {
      eprintln!("{}", res);
      Err(anyhow::anyhow!("Recieved {} status code", statusCode))
    }
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
  loop 
  {
    let res = f(cursor.clone()).await?;

    data.append(&mut res.data.clone());
    cursor = res.cursor;

    match cursor 
    {
      None => break,
      _ => {}
    }
  }

  Ok(data)
}

#[async_trait::async_trait]
impl CardanoApi for TangoClient 
{
  // recommended api:
  // https://www.tangocrypto.com/api-reference/#/operations/list-stake_address-addresses
  async fn get_all_addresses(&self, stake_address: &str) -> anyhow::Result<Vec<String>> {
    let result = get_all(|cursor| self.get_addresses(stake_address, cursor)).await?;
    Ok(result.into_iter().map(|x| x.address).collect())
  }

  // recommended api:
  // https://www.tangocrypto.com/api-reference/#/operations/list-address-assets
  async fn get_address_assets(&self, address: &str) -> anyhow::Result<Vec<Asset>> {
    let result = get_all(|cursor| self.get_address_assets(address, cursor)).await?;
    Ok(result.into_iter().map(|x| Asset {
      policy_id: x.policy_id,
      asset_name: x.asset_name,
      quantity: x.quantity
    }).collect())    
  }

  // recommended api:
  // https://www.tangocrypto.com/api-reference/#/operations/list-asset-addresses
  async fn get_asset_addresses(&self, asset_id: &str) -> anyhow::Result<HashSet<String>> {
    let getAllResult = get_all(|cursor| self.get_asset_addresses(asset_id, cursor)).await?;

    let mut result = HashSet::new();
    for asset in getAllResult.into_iter()
    {
      result.insert(asset.address);
    }

    Ok(result)
  }
}

fn parse_apiBody<T>(body: &String) -> anyhow::Result<ApiListRes<T>>
where
  T: DeserializeOwned
{
  let parsed: serde_json::Result<ApiListRes<T>> = serde_json::from_str(body);
  match parsed
  {
    Ok(result) => Ok(result),
    Err(_) => Err(anyhow::anyhow!("Error parsing json into custom type"))
  }
}