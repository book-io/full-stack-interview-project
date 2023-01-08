pub fn add_cursor_to_url(url: &str, cursor: &str) -> String {
    format!("{}?cursor={}", url, cursor).to_string()
}

pub fn generate_get_all_addresses_endpoint(
    base_url: &str,
    app_id: &str,
    stake_address: &str,
) -> String {
    format!(
        "{}/{}/v1/wallets/{}/addresses",
        base_url, app_id, stake_address
    )
    .to_string()
}
