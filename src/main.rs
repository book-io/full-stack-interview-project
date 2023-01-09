use std::{collections::HashSet, env, sync::Arc};

use cardano::tango::client::TangoClient;

use crate::model::{book::BookId, bookshelf::Bookshelf};

pub mod cardano;
pub mod model;

// NOTE: do not change anything in this file. you are free to modify
// other files, but this driver should remain as-is.

fn get_tango() -> Option<TangoClient> {
    match (
        env::var("TANGO_BASE_URL").ok(),
        env::var("TANGO_APP_ID").ok(),
        env::var("TANGO_API_KEY").ok(),
    ) {
        (Some(base_url), Some(app_id), Some(api_key)) => {
            TangoClient::new(base_url, app_id, api_key).ok()
        }
        _ => None,
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Err(e) = dotenv::dotenv() {
        eprintln!("failed to load .env file: {}", e);
    }

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(anyhow::anyhow!(
            "Runs the interview project driver.\n\n\
              Usage: {} <stake_address>\n",
            args[0]
        ));
    }

    let stake_address = args.get(1).unwrap().to_string();
    let tango = get_tango().ok_or_else(|| anyhow::anyhow!("tango config missing"))?;

    let policy_id = "b27160f0c50a9cf168bf945dcbfcabbfbee5c7a801e7b467093b4153".to_string();
    let bookshelf = Bookshelf::new(Arc::new(Box::new(tango)), stake_address);
    let books = bookshelf
        .get_books(HashSet::from([policy_id.clone()]))
        .await?;

    let num_books = books.len();
    println!(
        "found {} {}",
        num_books,
        if num_books == 1 { "book" } else { "books" }
    );

    for book in books.iter().take(5) {
        println!("id: {}, token name: {}", book.id, book.token_name);
    }

    let id0 = BookId::new(
        policy_id.clone(),
        "53637265616d4f66416e67656c7330303033".into(),
    );
    let id1 = BookId::new(policy_id.clone(), "4d6574616c4d6f6e7374657230303036".into());
    let id2 = BookId::new(policy_id.clone(), "536c65657079486f6c6c6f77303030".into());

    println!("has {}: {}", &id0, bookshelf.has_book(&id0).await);
    println!("has {}: {}", &id1, bookshelf.has_book(&id1).await);
    println!("has {}: {}", &id2, bookshelf.has_book(&id2).await);

    Ok(())
}