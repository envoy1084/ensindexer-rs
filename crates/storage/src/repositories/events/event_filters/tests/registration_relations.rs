use super::*;
use crate::filters::{AccountFilter, DomainFilter, RegistrationFilter};

#[test]
fn registration_event_relation_filters_apply_parent_predicates() {
    let filter = EventFilter {
        registration_filter: Some(Box::new(RegistrationFilter {
            domain_filter: Some(Box::new(DomainFilter {
                name: Some("vitalik.eth".into()),
                ..DomainFilter::default()
            })),
            ..RegistrationFilter::default()
        })),
        ..EventFilter::default()
    };
    let mut query = QueryBuilder::<Postgres>::new("select id from name_registered_events");
    {
        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_event_filters(&mut separated, &mut has_where, "registration_id", &filter);
        push_concrete_parent_relation_filter(
            &mut separated,
            &mut has_where,
            "registration_id",
            &filter,
        );
        separated.push_unseparated(" ");
    }

    let built = query.build();
    assert_eq!(
        built.sql(),
        "select id from name_registered_events where registration_id in (select id from registrations where domain_id in (select id from domains where name = $1)) "
    );
}

#[test]
fn registration_event_interface_relation_filters_apply_parent_predicates() {
    let filter = EventFilter {
        registration_filter: Some(Box::new(RegistrationFilter {
            registrant_filter: Some(Box::new(AccountFilter {
                id: Some("0xregistrant".into()),
                ..AccountFilter::default()
            })),
            ..RegistrationFilter::default()
        })),
        ..EventFilter::default()
    };
    let mut query = QueryBuilder::<Postgres>::new("select id from registration_event_refs");
    {
        let mut separated = query.separated(" and ");
        let mut has_where = false;
        push_event_filters(&mut separated, &mut has_where, "parent_id", &filter);
        push_interface_parent_relation_filter(
            &mut separated,
            &mut has_where,
            "registration_event_refs",
            &filter,
        );
        separated.push_unseparated(" ");
    }

    let built = query.build();
    assert_eq!(
        built.sql(),
        "select id from registration_event_refs where parent_id in (select id from registrations where registrant_id in (select id from accounts where id = $1)) "
    );
}
