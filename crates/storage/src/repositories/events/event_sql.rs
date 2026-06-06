pub(super) fn domain_event_ref_union_sql() -> &'static str {
    r#"
    select 'Transfer' as kind, id, block_number, transaction_id, domain_id as parent_id from transfer_events
    union all select 'NewOwner' as kind, id, block_number, transaction_id, domain_id as parent_id from new_owner_events
    union all select 'NewResolver' as kind, id, block_number, transaction_id, domain_id as parent_id from new_resolver_events
    union all select 'NewTTL' as kind, id, block_number, transaction_id, domain_id as parent_id from new_ttl_events
    union all select 'WrappedTransfer' as kind, id, block_number, transaction_id, domain_id as parent_id from wrapped_transfer_events
    union all select 'NameWrapped' as kind, id, block_number, transaction_id, domain_id as parent_id from name_wrapped_events
    union all select 'NameUnwrapped' as kind, id, block_number, transaction_id, domain_id as parent_id from name_unwrapped_events
    union all select 'FusesSet' as kind, id, block_number, transaction_id, domain_id as parent_id from fuses_set_events
    union all select 'ExpiryExtended' as kind, id, block_number, transaction_id, domain_id as parent_id from expiry_extended_events
    "#
}

pub(super) fn registration_event_ref_union_sql() -> &'static str {
    r#"
    select 'NameRegistered' as kind, id, block_number, transaction_id, registration_id as parent_id from name_registered_events
    union all select 'NameRenewed' as kind, id, block_number, transaction_id, registration_id as parent_id from name_renewed_events
    union all select 'NameTransferred' as kind, id, block_number, transaction_id, registration_id as parent_id from name_transferred_events
    "#
}

pub(super) fn resolver_event_ref_union_sql() -> &'static str {
    r#"
    select 'AddrChanged' as kind, id, block_number, transaction_id, resolver_id as parent_id from addr_changed_events
    union all select 'MulticoinAddrChanged' as kind, id, block_number, transaction_id, resolver_id as parent_id from multicoin_addr_changed_events
    union all select 'NameChanged' as kind, id, block_number, transaction_id, resolver_id as parent_id from name_changed_events
    union all select 'AbiChanged' as kind, id, block_number, transaction_id, resolver_id as parent_id from abi_changed_events
    union all select 'PubkeyChanged' as kind, id, block_number, transaction_id, resolver_id as parent_id from pubkey_changed_events
    union all select 'TextChanged' as kind, id, block_number, transaction_id, resolver_id as parent_id from text_changed_events
    union all select 'ContenthashChanged' as kind, id, block_number, transaction_id, resolver_id as parent_id from contenthash_changed_events
    union all select 'InterfaceChanged' as kind, id, block_number, transaction_id, resolver_id as parent_id from interface_changed_events
    union all select 'AuthorisationChanged' as kind, id, block_number, transaction_id, resolver_id as parent_id from authorisation_changed_events
    union all select 'VersionChanged' as kind, id, block_number, transaction_id, resolver_id as parent_id from version_changed_events
    "#
}
