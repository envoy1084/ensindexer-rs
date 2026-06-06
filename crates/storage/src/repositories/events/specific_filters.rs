use sqlx::Postgres;

use crate::{filters::EventFilter, query::*};

pub(super) fn push_event_specific_filters<'qb>(
    separated: &mut sqlx::query_builder::Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    table: &'static str,
    filter: &EventFilter,
) {
    match table {
        "transfer_events" | "wrapped_transfer_events" | "name_unwrapped_events" => {
            push_account_event_filter(separated, has_where, "owner_id", filter.owner_id.clone());
        }
        "new_owner_events" => {
            push_text_filter(
                separated,
                has_where,
                "parent_domain_id",
                filter.parent_domain_id.clone(),
            );
            push_account_event_filter(separated, has_where, "owner_id", filter.owner_id.clone());
        }
        "new_resolver_events" => {
            push_text_filter(
                separated,
                has_where,
                "resolver_id",
                filter.resolver_id.clone(),
            );
        }
        "new_ttl_events" => {
            push_numeric_event_filter(separated, has_where, "ttl", filter, NumericEventField::Ttl);
        }
        "name_wrapped_events" => {
            push_text_event_field(separated, has_where, "name", text_field_name(filter));
            push_i32_event_filter(separated, has_where, "fuses", filter);
            push_account_event_filter(separated, has_where, "owner_id", filter.owner_id.clone());
            push_numeric_event_filter(
                separated,
                has_where,
                "expiry_date",
                filter,
                NumericEventField::ExpiryDate,
            );
        }
        "fuses_set_events" => {
            push_i32_event_filter(separated, has_where, "fuses", filter);
        }
        "expiry_extended_events" | "name_renewed_events" => {
            push_numeric_event_filter(
                separated,
                has_where,
                "expiry_date",
                filter,
                NumericEventField::ExpiryDate,
            );
        }
        "name_registered_events" => {
            push_account_event_filter(
                separated,
                has_where,
                "registrant_id",
                filter.registrant_id.clone(),
            );
            push_numeric_event_filter(
                separated,
                has_where,
                "expiry_date",
                filter,
                NumericEventField::ExpiryDate,
            );
        }
        "name_transferred_events" => {
            push_account_event_filter(
                separated,
                has_where,
                "new_owner_id",
                filter.new_owner_id.clone(),
            );
        }
        "addr_changed_events" => {
            push_account_event_filter(separated, has_where, "addr_id", filter.addr_id.clone());
        }
        "multicoin_addr_changed_events" => {
            push_numeric_event_filter(
                separated,
                has_where,
                "coin_type",
                filter,
                NumericEventField::CoinType,
            );
            push_text_filter(separated, has_where, "addr", filter.addr_id.clone());
        }
        "name_changed_events" => {
            push_text_event_field(separated, has_where, "name", text_field_name(filter));
        }
        "abi_changed_events" => {
            push_numeric_event_filter(
                separated,
                has_where,
                "content_type",
                filter,
                NumericEventField::ContentType,
            );
        }
        "pubkey_changed_events" => {
            push_text_filter(separated, has_where, "x", filter.x.clone());
            push_text_filter(separated, has_where, "y", filter.y.clone());
        }
        "text_changed_events" => {
            push_text_event_field(separated, has_where, "key", text_field_key(filter));
            push_text_event_field(separated, has_where, "value", text_field_value(filter));
        }
        "contenthash_changed_events" => {
            push_text_filter(separated, has_where, "hash", filter.hash.clone());
        }
        "interface_changed_events" => {
            push_text_filter(
                separated,
                has_where,
                "interface_id",
                filter.interface_id.clone(),
            );
            push_text_filter(
                separated,
                has_where,
                "implementer",
                filter.implementer.clone(),
            );
        }
        "authorisation_changed_events" => {
            push_text_filter(separated, has_where, "owner", filter.owner_id.clone());
            push_text_filter(separated, has_where, "target", filter.target.clone());
            push_bool_filter(
                separated,
                has_where,
                "is_authorized",
                "=",
                filter.is_authorized,
            );
            push_bool_filter(
                separated,
                has_where,
                "is_authorized",
                "!=",
                filter.is_authorized_not,
            );
            push_bool_array_filter(
                separated,
                has_where,
                "is_authorized",
                filter.is_authorized_in.clone(),
                false,
            );
            push_bool_array_filter(
                separated,
                has_where,
                "is_authorized",
                filter.is_authorized_not_in.clone(),
                true,
            );
        }
        "version_changed_events" => {
            push_numeric_event_filter(
                separated,
                has_where,
                "version",
                filter,
                NumericEventField::Version,
            );
        }
        _ => {}
    }
}

fn push_account_event_filter<'qb>(
    separated: &mut sqlx::query_builder::Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    value: Option<String>,
) {
    push_text_filter(separated, has_where, column, value);
}

fn push_i32_event_filter<'qb>(
    separated: &mut sqlx::query_builder::Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    filter: &EventFilter,
) {
    push_i32_filter(separated, has_where, column, "=", filter.fuses);
    push_i32_filter(separated, has_where, column, "!=", filter.fuses_not);
    push_i32_filter(separated, has_where, column, ">", filter.fuses_gt);
    push_i32_filter(separated, has_where, column, "<", filter.fuses_lt);
    push_i32_filter(separated, has_where, column, ">=", filter.fuses_gte);
    push_i32_filter(separated, has_where, column, "<=", filter.fuses_lte);
    push_i32_array_filter(separated, has_where, column, filter.fuses_in.clone(), false);
    push_i32_array_filter(
        separated,
        has_where,
        column,
        filter.fuses_not_in.clone(),
        true,
    );
}

fn push_text_event_field<'qb>(
    separated: &mut sqlx::query_builder::Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    filter: TextFieldFilter,
) {
    push_text_field_filters(separated, has_where, column, filter);
}

fn text_field_name(filter: &EventFilter) -> TextFieldFilter {
    TextFieldFilter {
        exact: filter.name.clone(),
        not: filter.name_not.clone(),
        gt: filter.name_gt.clone(),
        lt: filter.name_lt.clone(),
        gte: filter.name_gte.clone(),
        lte: filter.name_lte.clone(),
        in_values: filter.name_in.clone(),
        not_in: filter.name_not_in.clone(),
        contains: filter.name_contains.clone(),
        contains_nocase: filter.name_contains_nocase.clone(),
        not_contains: filter.name_not_contains.clone(),
        not_contains_nocase: filter.name_not_contains_nocase.clone(),
        starts_with: filter.name_starts_with.clone(),
        starts_with_nocase: filter.name_starts_with_nocase.clone(),
        not_starts_with: filter.name_not_starts_with.clone(),
        not_starts_with_nocase: filter.name_not_starts_with_nocase.clone(),
        ends_with: filter.name_ends_with.clone(),
        ends_with_nocase: filter.name_ends_with_nocase.clone(),
        not_ends_with: filter.name_not_ends_with.clone(),
        not_ends_with_nocase: filter.name_not_ends_with_nocase.clone(),
    }
}

fn text_field_key(filter: &EventFilter) -> TextFieldFilter {
    TextFieldFilter {
        exact: filter.key.clone(),
        not: filter.key_not.clone(),
        gt: filter.key_gt.clone(),
        lt: filter.key_lt.clone(),
        gte: filter.key_gte.clone(),
        lte: filter.key_lte.clone(),
        in_values: filter.key_in.clone(),
        not_in: filter.key_not_in.clone(),
        contains: filter.key_contains.clone(),
        contains_nocase: filter.key_contains_nocase.clone(),
        not_contains: filter.key_not_contains.clone(),
        not_contains_nocase: filter.key_not_contains_nocase.clone(),
        starts_with: filter.key_starts_with.clone(),
        starts_with_nocase: filter.key_starts_with_nocase.clone(),
        not_starts_with: filter.key_not_starts_with.clone(),
        not_starts_with_nocase: filter.key_not_starts_with_nocase.clone(),
        ends_with: filter.key_ends_with.clone(),
        ends_with_nocase: filter.key_ends_with_nocase.clone(),
        not_ends_with: filter.key_not_ends_with.clone(),
        not_ends_with_nocase: filter.key_not_ends_with_nocase.clone(),
    }
}

fn text_field_value(filter: &EventFilter) -> TextFieldFilter {
    TextFieldFilter {
        exact: filter.value.clone(),
        not: filter.value_not.clone(),
        gt: filter.value_gt.clone(),
        lt: filter.value_lt.clone(),
        gte: filter.value_gte.clone(),
        lte: filter.value_lte.clone(),
        in_values: filter.value_in.clone(),
        not_in: filter.value_not_in.clone(),
        contains: filter.value_contains.clone(),
        contains_nocase: filter.value_contains_nocase.clone(),
        not_contains: filter.value_not_contains.clone(),
        not_contains_nocase: filter.value_not_contains_nocase.clone(),
        starts_with: filter.value_starts_with.clone(),
        starts_with_nocase: filter.value_starts_with_nocase.clone(),
        not_starts_with: filter.value_not_starts_with.clone(),
        not_starts_with_nocase: filter.value_not_starts_with_nocase.clone(),
        ends_with: filter.value_ends_with.clone(),
        ends_with_nocase: filter.value_ends_with_nocase.clone(),
        not_ends_with: filter.value_not_ends_with.clone(),
        not_ends_with_nocase: filter.value_not_ends_with_nocase.clone(),
    }
}

enum NumericEventField {
    Ttl,
    ExpiryDate,
    CoinType,
    ContentType,
    Version,
}

fn push_numeric_event_filter<'qb>(
    separated: &mut sqlx::query_builder::Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    filter: &EventFilter,
    field: NumericEventField,
) {
    let (eq, not, gt, lt, gte, lte, in_values, not_in) = match field {
        NumericEventField::Ttl => (
            filter.ttl.clone(),
            filter.ttl_not.clone(),
            filter.ttl_gt.clone(),
            filter.ttl_lt.clone(),
            filter.ttl_gte.clone(),
            filter.ttl_lte.clone(),
            filter.ttl_in.clone(),
            filter.ttl_not_in.clone(),
        ),
        NumericEventField::ExpiryDate => (
            filter.expiry_date.clone(),
            filter.expiry_date_not.clone(),
            filter.expiry_date_gt.clone(),
            filter.expiry_date_lt.clone(),
            filter.expiry_date_gte.clone(),
            filter.expiry_date_lte.clone(),
            filter.expiry_date_in.clone(),
            filter.expiry_date_not_in.clone(),
        ),
        NumericEventField::CoinType => (
            filter.coin_type.clone(),
            filter.coin_type_not.clone(),
            filter.coin_type_gt.clone(),
            filter.coin_type_lt.clone(),
            filter.coin_type_gte.clone(),
            filter.coin_type_lte.clone(),
            filter.coin_type_in.clone(),
            filter.coin_type_not_in.clone(),
        ),
        NumericEventField::ContentType => (
            filter.content_type.clone(),
            filter.content_type_not.clone(),
            filter.content_type_gt.clone(),
            filter.content_type_lt.clone(),
            filter.content_type_gte.clone(),
            filter.content_type_lte.clone(),
            filter.content_type_in.clone(),
            filter.content_type_not_in.clone(),
        ),
        NumericEventField::Version => (
            filter.version.clone(),
            filter.version_not.clone(),
            filter.version_gt.clone(),
            filter.version_lt.clone(),
            filter.version_gte.clone(),
            filter.version_lte.clone(),
            filter.version_in.clone(),
            filter.version_not_in.clone(),
        ),
    };

    push_numeric_text_filter(separated, has_where, column, "=", eq);
    push_numeric_text_filter(separated, has_where, column, "!=", not);
    push_numeric_text_filter(separated, has_where, column, ">", gt);
    push_numeric_text_filter(separated, has_where, column, "<", lt);
    push_numeric_text_filter(separated, has_where, column, ">=", gte);
    push_numeric_text_filter(separated, has_where, column, "<=", lte);
    push_numeric_text_array_filter(separated, has_where, column, in_values, false);
    push_numeric_text_array_filter(separated, has_where, column, not_in, true);
}
