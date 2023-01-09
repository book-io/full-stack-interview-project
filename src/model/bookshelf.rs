use std::{collections::HashSet, sync::Arc};

use crate::cardano::{api::CardanoApi, model::Asset, address};
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
    ) -> anyhow::Result<Vec<BookListItem>> {

        let addresses = self.api.get_all_addresses(&self.stake_address).await?;

        let mut assets: Vec<Asset> = Vec::new();

        for address in addresses.into_iter() {
            assets.append(&mut self.api.get_address_assets(&address).await?);
        }
        let book_list: Vec<BookListItem> = assets
            .into_iter()
            .filter(|a| policy_ids.contains(&a.policy_id))
            .map(|asset|BookListItem{
                id: BookId::new(asset.policy_id.to_string(),hex::encode(asset.fingerprint.to_string())),
                token_name: asset.asset_name.to_string()
            })
            .collect();
        Ok(book_list)
    }

    /**
     * Returns true if the book exists on the bookshelf and false otherwise.
     */
    pub async fn has_book(&self, id: &BookId) -> bool {
        //dbg!(id);
        // bonus points if you can implement this more efficiently than just
        // calling get_books and seeing if the BookId exists in that set.
        //
        let asset_addresses = self.api.get_asset_addresses(&id.as_asset_id()).await.unwrap();
        
        asset_addresses.into_iter().fold(true, |acc, ast|{
            acc && address::get_address_stake_key(&ast).map(|stkaddr|{
                match stkaddr {
                    Some(stk) => {
                        stk.contains(&self.stake_address)
                    },
                    None => false
                }
            }).unwrap()
        })
    }
}
