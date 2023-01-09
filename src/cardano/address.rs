/**
 * Takes an address and returns the stake address, if possible.
 */
pub fn get_address_stake_key(
    addr: &str,
) -> Result<Option<String>, cardano_serialization_lib::error::JsError> {
    let address = cardano_serialization_lib::address::Address::from_bech32(addr)?;
    match cardano_serialization_lib::address::BaseAddress::from_address(&address) {
        Some(base) => Ok(Some(
            cardano_serialization_lib::address::RewardAddress::new(
                address.network_id()?,
                &base.stake_cred(),
            )
            .to_address()
            .to_bech32(None)?,
        )),
        None => Ok(None),
    }
}

/**
 * Returns if an address corresponds to a stake address
 */
pub fn address_belongs_to_stake_address(address: &str, stake_address: &str) -> bool {
    let address_option = get_address_stake_key(address).unwrap();

    address_option.is_some() && (address_option.unwrap() == stake_address)
}
