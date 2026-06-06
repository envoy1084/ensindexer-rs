use crate::{error::*, filters::*, models::*};

use super::{EventsRepo, columns::*};

impl EventsRepo<'_> {
    pub async fn find_transfer_by_id(&self, id: &str) -> StorageResult<Option<TransferEventRow>> {
        self.find_event("transfer_events", transfer_event_columns(), id)
            .await
    }

    pub async fn list_transfers(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<TransferEventRow>> {
        self.list_events(
            "transfer_events",
            transfer_event_columns(),
            "domain_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_new_owner_by_id(&self, id: &str) -> StorageResult<Option<NewOwnerEventRow>> {
        self.find_event("new_owner_events", new_owner_event_columns(), id)
            .await
    }

    pub async fn list_new_owners(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<NewOwnerEventRow>> {
        self.list_events(
            "new_owner_events",
            new_owner_event_columns(),
            "domain_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_new_resolver_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<NewResolverEventRow>> {
        self.find_event("new_resolver_events", new_resolver_event_columns(), id)
            .await
    }

    pub async fn list_new_resolvers(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<NewResolverEventRow>> {
        self.list_events(
            "new_resolver_events",
            new_resolver_event_columns(),
            "domain_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_new_ttl_by_id(&self, id: &str) -> StorageResult<Option<NewTtlEventRow>> {
        self.find_event("new_ttl_events", new_ttl_event_columns(), id)
            .await
    }

    pub async fn list_new_ttls(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<NewTtlEventRow>> {
        self.list_events(
            "new_ttl_events",
            new_ttl_event_columns(),
            "domain_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_wrapped_transfer_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<WrappedTransferEventRow>> {
        self.find_event(
            "wrapped_transfer_events",
            wrapped_transfer_event_columns(),
            id,
        )
        .await
    }

    pub async fn list_wrapped_transfers(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<WrappedTransferEventRow>> {
        self.list_events(
            "wrapped_transfer_events",
            wrapped_transfer_event_columns(),
            "domain_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_name_wrapped_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<NameWrappedEventRow>> {
        self.find_event("name_wrapped_events", name_wrapped_event_columns(), id)
            .await
    }

    pub async fn list_name_wrapped(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<NameWrappedEventRow>> {
        self.list_events(
            "name_wrapped_events",
            name_wrapped_event_columns(),
            "domain_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_name_unwrapped_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<NameUnwrappedEventRow>> {
        self.find_event("name_unwrapped_events", name_unwrapped_event_columns(), id)
            .await
    }

    pub async fn list_name_unwrapped(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<NameUnwrappedEventRow>> {
        self.list_events(
            "name_unwrapped_events",
            name_unwrapped_event_columns(),
            "domain_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_fuses_set_by_id(&self, id: &str) -> StorageResult<Option<FusesSetEventRow>> {
        self.find_event("fuses_set_events", fuses_set_event_columns(), id)
            .await
    }

    pub async fn list_fuses_set(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<FusesSetEventRow>> {
        self.list_events(
            "fuses_set_events",
            fuses_set_event_columns(),
            "domain_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }

    pub async fn find_expiry_extended_by_id(
        &self,
        id: &str,
    ) -> StorageResult<Option<ExpiryExtendedEventRow>> {
        self.find_event(
            "expiry_extended_events",
            expiry_extended_event_columns(),
            id,
        )
        .await
    }

    pub async fn list_expiry_extended(
        &self,
        first: i64,
        skip: i64,
        filter: EventFilter,
        order_by: EventOrderField,
        direction: OrderDirection,
    ) -> StorageResult<Vec<ExpiryExtendedEventRow>> {
        self.list_events(
            "expiry_extended_events",
            expiry_extended_event_columns(),
            "domain_id",
            first,
            skip,
            filter,
            order_by,
            direction,
        )
        .await
    }
}
