use std::{collections::HashSet, sync::Arc};

use crate::cardano::api::CardanoApi;
use crate::cardano::model::Asset;

use super::book::{BookId, BookListItem};


pub struct Bookshelf {
  api: Arc<Box<dyn CardanoApi>>,
  stake_address: String,
}

impl Bookshelf {
  pub fn new(api: Arc<Box<dyn CardanoApi>>, stake_address: String) -> Self {
    Bookshelf { api, stake_address }
  }

  /**
    * Gets the collection of books available on this bookshelf.
    */
  pub async fn get_books(
    &self,
    policy_ids: HashSet<String>,
  ) -> anyhow::Result<Vec<BookListItem>> 
  {
    let addresses = self.api.get_all_addresses(self.stake_address.as_str()).await?;

    let mut assets: Vec<Asset> = Vec::new();
    for addr in addresses.iter()
    {
      let mut assets_ = self.api.get_address_assets(addr).await?;
      assets.append(&mut assets_);
    }
    
    let books: Vec<BookListItem> = assets.into_iter()
      .filter(|x| policy_ids.contains(&x.policy_id))
      .map(|x| BookListItem 
      {
        id: BookId::new(x.policy_id, x.asset_name.clone()),
        token_name: x.asset_name
      }).collect();

    Ok(books)
  }

  /**
    * Returns true if the book exists on the bookshelf and false otherwise.
    */
  pub async fn has_book(&self, id: &BookId) -> bool 
  {
    // bonus points if you can implement this more efficiently than just
    // calling get_books and seeing if the BookId exists in that set.

    //BookId is the same as asset id as expected by the tangocrypto api:
    // https://www.tangocrypto.com/api-reference/#/operations/list-asset-addresses
    let assetAddresses = self.api.get_asset_addresses(id.0.as_str()).await;

    if let Ok(addresses) = assetAddresses
    {
      for addr in addresses.iter()
      {
        let mut assets= self.api.get_address_assets(addr).await;
        if let Ok(assets_) = assets
        {
          if assets_.len() > 0
          {
            return true;
          }
        }
      }
      false
    }
    else
    {
      false
    }
  }
}