pub fn add_cursor_to_url(url: &str, cursor: &str) -> String {
    format!("{url}?cursor={cursor}")
}

pub fn generate_get_all_addresses_for_stake_address_endpoint(
    base_url: &str,
    app_id: &str,
    stake_address: &str,
) -> String {
    format!("{base_url}/{app_id}/v1/wallets/{stake_address}/addresses")
}

pub fn generate_get_all_assets_endpoint(base_url: &str, app_id: &str, address: &str) -> String {
    format!("{base_url}/{app_id}/v1/addresses/{address}/assets")
}

pub fn generate_get_all_addresses_for_asset_endpoint(
    base_url: &str,
    app_id: &str,
    asset_id: &str,
) -> String {
    format!("{base_url}/{app_id}/v1/assets/{asset_id}/addresses")
}
