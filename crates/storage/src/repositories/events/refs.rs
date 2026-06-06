use crate::{error::*, filters::*, models::*};

use super::{EventsRepo, event_sql::*};

impl EventsRepo<'_> {
    pub async fn list_domain_event_refs(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<EventReferenceRow>> {
        self.list_event_refs(
            domain_event_ref_union_sql(),
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn list_registration_event_refs(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<EventReferenceRow>> {
        self.list_event_refs(
            registration_event_ref_union_sql(),
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn list_resolver_event_refs(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<EventReferenceRow>> {
        self.list_event_refs(
            resolver_event_ref_union_sql(),
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }
}
