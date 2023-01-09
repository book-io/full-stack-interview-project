use crate::cardano::address::address_belongs_to_stake_address;
use std::{collections::HashSet, sync::Arc};

use crate::cardano::api::CardanoApi;

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
        let mut books: Vec<BookListItem> = vec![];

        let addresses = self.api.get_all_addresses(&self.stake_address).await?;

        for address in addresses {
            let assets = self.api.get_address_assets(&address).await?;

            for asset in &assets {
                let asset_policy_id = asset.policy_id.clone();
                let asset_policy_is_for_valid_book = policy_ids
                    .iter()
                    .any(|policy_id| &asset_policy_id == policy_id);

                if asset_policy_is_for_valid_book {
                    let asset_name_hex = hex::encode(asset.asset_name.clone());
                    let token_name = asset.asset_name.clone();

                    let book_id: BookId = BookId::new(asset_policy_id, asset_name_hex);

                    let book: BookListItem = BookListItem {
                        id: book_id,
                        token_name,
                    };

                    books.push(book);
                }
            }
        }

        Ok(books)
    }

    /**
     * Returns true if the book exists on the bookshelf and false otherwise.
     */
    pub async fn has_book(&self, id: &BookId) -> bool {
        // bonus points if you can implement this more efficiently than just
        // calling get_books and seeing if the BookId exists in that set.
        let mut has_book = false;

        let asset_id = id.as_asset_id();
        let book_addresses: Vec<String> = self
            .api
            .get_asset_addresses(&asset_id)
            .await
            .unwrap_or(HashSet::new())
            .into_iter()
            .collect();

        let is_an_nft = book_addresses.len() == 1;

        if is_an_nft {
            let book_address = book_addresses[0].clone();

            has_book = address_belongs_to_stake_address(&book_address, &self.stake_address);
        }

        has_book
    }
}
