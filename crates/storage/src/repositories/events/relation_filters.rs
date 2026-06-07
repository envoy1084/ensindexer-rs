use sqlx::{Postgres, query_builder::Separated};

use crate::{
    filters::EventFilter,
    query::{
        push_account_relation_filter, push_domain_relation_filter, push_resolver_relation_filter,
    },
};

pub(super) fn push_concrete_parent_relation_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    parent_column: &'static str,
    filter: &EventFilter,
) {
    match parent_column {
        "domain_id" => {
            push_domain_relation_filter(
                separated,
                has_where,
                parent_column,
                filter.domain_filter.clone(),
            );
        }
        "resolver_id" => {
            push_resolver_relation_filter(
                separated,
                has_where,
                parent_column,
                filter.resolver_filter.clone(),
            );
        }
        _ => {}
    }
}

pub(super) fn push_interface_parent_relation_filter<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    interface_table: &'static str,
    filter: &EventFilter,
) {
    match interface_table {
        "domain_event_refs" => {
            push_domain_relation_filter(
                separated,
                has_where,
                "parent_id",
                filter.domain_filter.clone(),
            );
        }
        "resolver_event_refs" => {
            push_resolver_relation_filter(
                separated,
                has_where,
                "parent_id",
                filter.resolver_filter.clone(),
            );
        }
        _ => {}
    }
}

pub(super) fn push_event_specific_relation_filters<'qb>(
    separated: &mut Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    table: &'static str,
    filter: &EventFilter,
) {
    match table {
        "transfer_events" | "wrapped_transfer_events" | "name_unwrapped_events" => {
            push_account_relation_filter(
                separated,
                has_where,
                "owner_id",
                filter.owner_filter.clone(),
            );
        }
        "new_owner_events" => {
            push_domain_relation_filter(
                separated,
                has_where,
                "parent_domain_id",
                filter.parent_domain_filter.clone(),
            );
            push_account_relation_filter(
                separated,
                has_where,
                "owner_id",
                filter.owner_filter.clone(),
            );
        }
        "new_resolver_events" => {
            push_resolver_relation_filter(
                separated,
                has_where,
                "resolver_id",
                filter.resolver_filter.clone(),
            );
        }
        "name_wrapped_events" => {
            push_account_relation_filter(
                separated,
                has_where,
                "owner_id",
                filter.owner_filter.clone(),
            );
        }
        "name_registered_events" => {
            push_account_relation_filter(
                separated,
                has_where,
                "registrant_id",
                filter.registrant_filter.clone(),
            );
        }
        "name_transferred_events" => {
            push_account_relation_filter(
                separated,
                has_where,
                "new_owner_id",
                filter.new_owner_filter.clone(),
            );
        }
        "addr_changed_events" => {
            push_account_relation_filter(
                separated,
                has_where,
                "addr_id",
                filter.addr_filter.clone(),
            );
        }
        "domain_event_refs" => {
            push_domain_relation_filter(
                separated,
                has_where,
                "parent_domain_id",
                filter.parent_domain_filter.clone(),
            );
            push_resolver_relation_filter(
                separated,
                has_where,
                "resolver_id",
                filter.resolver_filter.clone(),
            );
            push_account_relation_filter(
                separated,
                has_where,
                "owner_id",
                filter.owner_filter.clone(),
            );
        }
        "registration_event_refs" => {
            push_account_relation_filter(
                separated,
                has_where,
                "registrant_id",
                filter.registrant_filter.clone(),
            );
            push_account_relation_filter(
                separated,
                has_where,
                "new_owner_id",
                filter.new_owner_filter.clone(),
            );
        }
        "resolver_event_refs" => {
            push_account_relation_filter(
                separated,
                has_where,
                "addr_id",
                filter.addr_filter.clone(),
            );
        }
        _ => {}
    }
}
