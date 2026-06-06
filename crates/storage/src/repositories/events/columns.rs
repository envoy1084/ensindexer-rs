pub(super) fn transfer_event_columns() -> &'static str {
    "id, domain_id, block_number, transaction_id, owner_id"
}

pub(super) fn new_owner_event_columns() -> &'static str {
    "id, domain_id, block_number, transaction_id, parent_domain_id, owner_id"
}

pub(super) fn new_resolver_event_columns() -> &'static str {
    "id, domain_id, block_number, transaction_id, resolver_id"
}

pub(super) fn new_ttl_event_columns() -> &'static str {
    "id, domain_id, block_number, transaction_id, ttl"
}

pub(super) fn wrapped_transfer_event_columns() -> &'static str {
    "id, domain_id, block_number, transaction_id, owner_id"
}

pub(super) fn name_wrapped_event_columns() -> &'static str {
    "id, domain_id, block_number, transaction_id, name, fuses, owner_id, expiry_date"
}

pub(super) fn name_unwrapped_event_columns() -> &'static str {
    "id, domain_id, block_number, transaction_id, owner_id"
}

pub(super) fn fuses_set_event_columns() -> &'static str {
    "id, domain_id, block_number, transaction_id, fuses"
}

pub(super) fn expiry_extended_event_columns() -> &'static str {
    "id, domain_id, block_number, transaction_id, expiry_date"
}

pub(super) fn name_registered_event_columns() -> &'static str {
    "id, registration_id, block_number, transaction_id, registrant_id, expiry_date"
}

pub(super) fn name_renewed_event_columns() -> &'static str {
    "id, registration_id, block_number, transaction_id, expiry_date"
}

pub(super) fn name_transferred_event_columns() -> &'static str {
    "id, registration_id, block_number, transaction_id, new_owner_id"
}

pub(super) fn addr_changed_event_columns() -> &'static str {
    "id, resolver_id, block_number, transaction_id, addr_id"
}

pub(super) fn multicoin_addr_changed_event_columns() -> &'static str {
    "id, resolver_id, block_number, transaction_id, coin_type, addr"
}

pub(super) fn name_changed_event_columns() -> &'static str {
    "id, resolver_id, block_number, transaction_id, name"
}

pub(super) fn abi_changed_event_columns() -> &'static str {
    "id, resolver_id, block_number, transaction_id, content_type"
}

pub(super) fn pubkey_changed_event_columns() -> &'static str {
    "id, resolver_id, block_number, transaction_id, x, y"
}

pub(super) fn text_changed_event_columns() -> &'static str {
    "id, resolver_id, block_number, transaction_id, key, value"
}

pub(super) fn contenthash_changed_event_columns() -> &'static str {
    "id, resolver_id, block_number, transaction_id, hash"
}

pub(super) fn interface_changed_event_columns() -> &'static str {
    "id, resolver_id, block_number, transaction_id, interface_id, implementer"
}

pub(super) fn authorisation_changed_event_columns() -> &'static str {
    "id, resolver_id, block_number, transaction_id, owner, target, is_authorized"
}

pub(super) fn version_changed_event_columns() -> &'static str {
    "id, resolver_id, block_number, transaction_id, version"
}
