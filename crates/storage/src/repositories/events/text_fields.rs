use sqlx::Postgres;

use crate::{filters::EventFilter, query::*};

pub(super) fn push_text_event_field<'qb>(
    separated: &mut sqlx::query_builder::Separated<'qb, Postgres, &'static str>,
    has_where: &mut bool,
    column: &'static str,
    filter: TextFieldFilter,
) {
    push_text_field_filters(separated, has_where, column, filter);
}

pub(super) fn text_field_name(filter: &EventFilter) -> TextFieldFilter {
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

pub(super) fn text_field_key(filter: &EventFilter) -> TextFieldFilter {
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

pub(super) fn text_field_value(filter: &EventFilter) -> TextFieldFilter {
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
