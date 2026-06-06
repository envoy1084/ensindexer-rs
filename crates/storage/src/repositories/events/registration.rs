use crate::{error::*, filters::*, models::*};

use super::{EventsRepo, columns::*};

impl EventsRepo<'_> {
    pub async fn find_name_registered_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<NameRegisteredEventRow>> {
        self.find_event(
            "name_registered_events",
            name_registered_event_columns(),
            id,
        )
        .await
    }

    pub async fn list_name_registered(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<NameRegisteredEventRow>> {
        self.list_events(
            "name_registered_events",
            name_registered_event_columns(),
            "registration_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_name_renewed_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<NameRenewedEventRow>> {
        self.find_event("name_renewed_events", name_renewed_event_columns(), id)
            .await
    }

    pub async fn list_name_renewed(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<NameRenewedEventRow>> {
        self.list_events(
            "name_renewed_events",
            name_renewed_event_columns(),
            "registration_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_name_transferred_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<NameTransferredEventRow>> {
        self.find_event(
            "name_transferred_events",
            name_transferred_event_columns(),
            id,
        )
        .await
    }

    pub async fn list_name_transferred(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<NameTransferredEventRow>> {
        self.list_events(
            "name_transferred_events",
            name_transferred_event_columns(),
            "registration_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }
}
